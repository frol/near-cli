mod transfer_subcommand;

/// Submit transactions (like transfer tokens, call a function, etc) to NEAR protocol network
#[derive(Debug, clap::Clap)]
#[clap(version, author, setting(clap::AppSettings::ColoredHelp))]
pub struct CliArgs {
    signer_account_id: String,
    signer_public_key: String,
    signer_secret_key: String,
    receiver_account_id: String,
    #[clap(subcommand)]
    subcommand: CliSubCommand,
}

#[derive(Debug, clap::Clap)]
pub enum CliSubCommand {
    Transfer(transfer_subcommand::CliArgs),
}

impl CliArgs {
    pub fn process(&self, _parent_cli_args: &super::CliArgs) -> crate::CliResult {
        match &self.subcommand {
            CliSubCommand::Transfer(transfer_subcommand) => transfer_subcommand.process(self),
        }
    }
}
