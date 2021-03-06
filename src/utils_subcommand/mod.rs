mod generate_keypair_subcommand;
mod sign_transaction_subcommand;
mod submit_raw_transaction_subcommand;
mod verify_transaction_subcommand;

/// Collection of various low-level helpers
#[derive(Debug, clap::Clap)]
#[clap(version, author, setting(clap::AppSettings::ColoredHelp))]
pub struct CliArgs {
    #[clap(subcommand)]
    subcommand: CliSubCommand,
}

impl CliArgs {
    fn rpc_client(&self) -> near_jsonrpc_client::JsonRpcClient {
        near_jsonrpc_client::new_client("https://rpc.testnet.near.org")
    }
}

#[derive(Debug, clap::Clap)]
pub enum CliSubCommand {
    GenerateKeypair(generate_keypair_subcommand::CliArgs),
    SignTransaction(sign_transaction_subcommand::CliArgs),
    VerifyTransaction(verify_transaction_subcommand::CliArgs),
    SubmitRawTransaction(submit_raw_transaction_subcommand::CliArgs),
}

impl CliArgs {
    pub async fn process(&self, _parent_cli_args: &super::CliArgs) -> crate::CliResult {
        match &self.subcommand {
            CliSubCommand::GenerateKeypair(generate_keypair_subcommand) => {
                generate_keypair_subcommand.process(self).await
            }
            CliSubCommand::SignTransaction(sign_transaction_subcommand) => {
                sign_transaction_subcommand.process(self).await
            }
            CliSubCommand::VerifyTransaction(verify_transaction_subcommand) => {
                verify_transaction_subcommand.process(self).await
            }
            CliSubCommand::SubmitRawTransaction(submit_raw_transaction_subcommand) => {
                submit_raw_transaction_subcommand.process(self).await
            }
        }
    }
}
