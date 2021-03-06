mod blocks_subcommand;

/// Inspect data stored in NEAR protocol network (e.g. blocks, transactions,
/// balances)
#[derive(Debug, clap::Clap)]
#[clap(version, author, setting(clap::AppSettings::ColoredHelp))]
pub struct CliArgs {
    #[clap(subcommand)]
    subcommand: CliSubCommand,
}

#[derive(Debug, clap::Clap)]
pub enum CliSubCommand {
    Blocks(blocks_subcommand::CliArgs),
}

impl CliArgs {
    fn rpc_client(&self) -> near_jsonrpc_client::JsonRpcClient {
        near_jsonrpc_client::new_client("https://rpc.testnet.near.org")
    }

    pub async fn process(&self, _parent_cli_args: &super::CliArgs) -> crate::CliResult {
        match &self.subcommand {
            CliSubCommand::Blocks(blocks_subcommand) => blocks_subcommand.process(self).await,
        }
    }
}
