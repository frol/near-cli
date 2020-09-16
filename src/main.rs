use clap::Clap;

pub(crate) mod common;
mod inspect_data_subcommand;
mod inspect_network_subcommand;
mod submit_subcommand;
mod utils_subcommand;

type CliResult = color_eyre::Result<()>;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Debug, Clap)]
#[clap(version, author, about, setting(clap::AppSettings::ColoredHelp))]
pub struct CliArgs {
    #[clap(subcommand)]
    subcommand: CliSubCommand,
}

#[derive(Debug, clap::Clap)]
pub enum CliSubCommand {
    InspectData(inspect_data_subcommand::CliArgs),
    InspectNetwork(inspect_network_subcommand::CliArgs),
    Submit(submit_subcommand::CliArgs),
    Utils(utils_subcommand::CliArgs),
}

impl CliArgs {
    pub fn process(&self) -> CliResult {
        match &self.subcommand {
            CliSubCommand::InspectData(inspect_data_subcommand) => {
                inspect_data_subcommand.process(self)
            }
            CliSubCommand::InspectNetwork(inspect_network_subcommand) => {
                inspect_network_subcommand.process(self)
            }
            CliSubCommand::Submit(send_transaction_subcommand) => {
                send_transaction_subcommand.process(self)
            }
            CliSubCommand::Utils(utils_subcommand) => utils_subcommand.process(self),
        }
    }
}

fn main() -> CliResult {
    let cli = CliArgs::parse();

    // We use it to automatically search the for root certificates to perform HTTPS calls
    // (inspect and submit subcommands)
    openssl_probe::init_ssl_cert_env_vars();

    color_eyre::install()?;

    cli.process()
}
