use std::convert::TryInto;

use near_primitives::serialize::BaseEncode;

/// Inspect blocks in NEAR protocol network
#[derive(Debug, clap::Clap)]
#[clap(version, author, setting(clap::AppSettings::ColoredHelp))]
pub struct CliArgs {
    #[clap(subcommand)]
    subcommand: CliSubCommand,
}

#[derive(Debug, clap::Clap)]
pub enum CliSubCommand {
    /// Fetch information about the latest finalized block
    GetFinalized,
    /// Fetch information about the latest block
    GetLatest,
    GetByHash(GetBlockByHashCliArgs),
    GetByHeight(GetBlockByHeightCliArgs),
}

/// Fetch information about a block by its hash
#[derive(Debug, clap::Clap)]
#[clap(version, author, setting(clap::AppSettings::ColoredHelp))]
pub struct GetBlockByHashCliArgs {
    hash: String,
}

/// Fetch information about a block by its height
#[derive(Debug, clap::Clap)]
#[clap(version, author, setting(clap::AppSettings::ColoredHelp))]
pub struct GetBlockByHeightCliArgs {
    height: near_primitives::types::BlockHeight,
}

impl CliArgs {
    pub async fn process(&self, parent_cli_args: &super::CliArgs) -> crate::CliResult {
        let block_reference = match &self.subcommand {
            CliSubCommand::GetFinalized => near_primitives::types::Finality::Final.into(),
            CliSubCommand::GetLatest => near_primitives::types::Finality::None.into(),
            CliSubCommand::GetByHash(args) => {
                near_primitives::types::BlockId::Hash(args.hash.as_str().try_into().map_err(
                    |err| color_eyre::Report::msg(format!("Could not parse block hash: {}", err)),
                )?)
                .into()
            }
            CliSubCommand::GetByHeight(args) => {
                near_primitives::types::BlockId::Height(args.height).into()
            }
        };

        let block = parent_cli_args
            .rpc_client()
            .block(block_reference)
            .await
            .map_err(|err| {
                color_eyre::Report::msg(format!("Could not fetch the latest block: {:?}", err))
            })?;
        // TODO: Show more or less details based on a CLI flag
        println!("The requested block: {:#?}", block);
        // TODO: Have it under a flag
        println!(
            "See it in Explorer: https://explorer.testnet.near.org/blocks/{}",
            block.header.hash.to_base()
        );
        Ok(())
    }
}
