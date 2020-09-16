use strum::VariantNames;

/// Submit a signed transaction to the network
#[derive(Debug, clap::Clap)]
#[clap(version, author, setting(clap::AppSettings::ColoredHelp))]
pub struct CliArgs {
    #[clap(
        short,
        long,
        default_value=crate::common::OutputFormat::default().into(),
        possible_values=crate::common::OutputFormat::VARIANTS
    )]
    format: crate::common::OutputFormat,
}

impl CliArgs {
    pub fn process(&self, _parent_cli_args: &super::CliArgs) -> crate::CliResult {
        unimplemented!()
    }
}
