use std::convert::TryInto;

mod transfer_subcommand;

/// Submit transactions (like transfer tokens, call a function, etc) to NEAR
/// protocol network
#[derive(Debug, clap::Clap)]
#[clap(version, author, setting(clap::AppSettings::ColoredHelp))]
pub struct CliArgs {
    #[clap(long)]
    signer_account_id: near_primitives::types::AccountId,
    #[clap(long)]
    signer_public_key: near_crypto::PublicKey,
    #[clap(long)]
    signer_secret_key: near_crypto::SecretKey,
    #[clap(long)]
    receiver_account_id: near_primitives::types::AccountId,
    #[clap(subcommand)]
    subcommand: CliSubCommand,
}

#[derive(Debug, clap::Clap)]
pub enum CliSubCommand {
    Transfer(transfer_subcommand::CliArgs),
}

impl CliArgs {
    pub fn process(&self, _parent_cli_args: &super::CliArgs) -> crate::CliResult {
        let block_hash = "5dMs1XfiUWCaP4USZ4EuSY6oLzm3w2FBNZeQu29H55sa"
            .try_into()
            .unwrap(); // fetch_latest_block_hash();
        let nonce = 10; // fetch_access_key(signer_account_id, signer_public_key).nonce;
        let unsigned_transaction = near_primitives::transaction::Transaction {
            signer_id: self.signer_account_id.clone(),
            public_key: self.signer_public_key.clone(),
            receiver_id: self.receiver_account_id.clone(),
            actions: vec![],
            block_hash,
            nonce,
        };

        match &self.subcommand {
            CliSubCommand::Transfer(transfer_subcommand) => {
                transfer_subcommand.process(self, unsigned_transaction)
            }
        }
    }
}
