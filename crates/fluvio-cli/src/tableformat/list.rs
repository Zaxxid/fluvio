//! # List TableFormats CLI
//!
//! CLI tree and processing to list TableFormats
//!

use std::sync::Arc;
use structopt::StructOpt;

use fluvio::Fluvio;
use fluvio_controlplane_metadata::tableformat::TableFormatSpec;

use fluvio_extension_common::Terminal;
use fluvio_extension_common::OutputFormat;
use crate::CliError;

#[derive(Debug, StructOpt)]
pub struct ListTableFormatsOpt {
    #[structopt(flatten)]
    output: OutputFormat,
}

impl ListTableFormatsOpt {
    /// Process list connectors cli request
    pub async fn process<O: Terminal>(self, out: Arc<O>, fluvio: &Fluvio) -> Result<(), CliError> {
        let admin = fluvio.admin().await;
        let lists = admin.list::<TableFormatSpec, _>(vec![]).await?;

        output::tableformats_response_to_output(out, lists, self.output.format)
    }
}

mod output {

    //!
    //! # Fluvio SC - output processing
    //!

    use prettytable::Row;
    use prettytable::row;
    use prettytable::Cell;
    use prettytable::cell;
    use prettytable::format::Alignment;
    use tracing::debug;
    use serde::Serialize;
    use fluvio_extension_common::output::OutputType;
    use fluvio_extension_common::Terminal;

    use fluvio::metadata::objects::Metadata;
    use fluvio_controlplane_metadata::tableformat::TableFormatSpec;

    use crate::CliError;
    use fluvio_extension_common::output::TableOutputHandler;
    use fluvio_extension_common::t_println;

    #[derive(Serialize)]
    struct ListTableFormats(Vec<Metadata<TableFormatSpec>>);

    // -----------------------------------
    // Format Output
    // -----------------------------------

    /// Format TableFormat list
    pub fn tableformats_response_to_output<O: Terminal>(
        out: std::sync::Arc<O>,
        list_tableformats: Vec<Metadata<TableFormatSpec>>,
        output_type: OutputType,
    ) -> Result<(), CliError> {
        debug!("tableformats: {:#?}", list_tableformats);

        if !list_tableformats.is_empty() {
            let tableformats = ListTableFormats(list_tableformats);
            out.render_list(&tableformats, output_type)?;
            Ok(())
        } else {
            t_println!(out, "no tableformats");
            Ok(())
        }
    }

    // -----------------------------------
    // Output Handlers
    // -----------------------------------
    impl TableOutputHandler for ListTableFormats {
        /// tableformat header implementation
        fn header(&self) -> Row {
            row!["NAME", "STATUS",]
        }

        /// return errors in string format
        fn errors(&self) -> Vec<String> {
            vec![]
        }

        /// table content implementation for tableformat (sry, naming makes this confusing)
        fn content(&self) -> Vec<Row> {
            self.0
                .iter()
                .map(|r| {
                    let _spec = &r.spec;
                    Row::new(vec![
                        Cell::new_align(&r.name, Alignment::RIGHT),
                        Cell::new_align(&r.status.to_string(), Alignment::RIGHT),
                    ])
                })
                .collect()
        }
    }
}