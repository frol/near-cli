#[derive(Debug, clap::Clap)]
#[clap(version, author, setting(clap::AppSettings::ColoredHelp))]
pub struct CliArgs {
    name: String,
}

impl CliArgs {
    pub async fn process(&self, _parent_cli_args: &super::CliArgs) -> crate::CliResult {
        println!("blocks");
        Ok(())
    }
}
