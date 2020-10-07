use clap::Clap;

pub(crate) mod common;
mod construct_transaction_subcommand;
mod inspect_data_subcommand;
mod inspect_network_subcommand;
mod submit_transaction_subcommand;
mod utils_subcommand;

type CliResult = color_eyre::Result<()>;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Debug, Clap)]
#[clap(
    version,
    author,
    about,
    setting(clap::AppSettings::ColoredHelp),
    setting(clap::AppSettings::DisableHelpSubcommand),
    setting(clap::AppSettings::VersionlessSubcommands),
    setting(clap::AppSettings::NextLineHelp)
)]
pub struct CliArgs {
    #[clap(subcommand)]
    subcommand: CliSubCommand,
}

#[derive(Debug, clap::Clap)]
pub enum CliSubCommand {
    InspectData(inspect_data_subcommand::CliArgs),
    InspectNetwork(inspect_network_subcommand::CliArgs),
    ConstructTransaction(construct_transaction_subcommand::CliArgs),
    SubmitTransaction(submit_transaction_subcommand::CliArgs),
    Utils(utils_subcommand::CliArgs),
}

impl CliArgs {
    pub async fn process(&self) -> CliResult {
        match &self.subcommand {
            CliSubCommand::InspectData(inspect_data_subcommand) => {
                inspect_data_subcommand.process(self).await
            }
            CliSubCommand::InspectNetwork(inspect_network_subcommand) => {
                inspect_network_subcommand.process(self).await
            }
            CliSubCommand::ConstructTransaction(construct_transaction_subcommand) => {
                construct_transaction_subcommand.process(self).await
            }
            CliSubCommand::SubmitTransaction(submit_transaction_subcommand) => {
                submit_transaction_subcommand.process(self).await
            }
            CliSubCommand::Utils(utils_subcommand) => utils_subcommand.process(self).await,
        }
    }
}

fn main() -> CliResult {
    let cli = CliArgs::parse();

    // We use it to automatically search the for root certificates to perform HTTPS
    // calls (inspect and submit subcommands)
    openssl_probe::init_ssl_cert_env_vars();

    color_eyre::install()?;

    actix::System::builder()
        .build()
        .block_on(async move { cli.process().await })
}
