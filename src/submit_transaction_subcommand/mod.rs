mod transfer_subcommand;

/// Submit transactions (like transfer tokens, call a function, etc) to NEAR
/// protocol network
///
/// See the help for individual subcommands
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
    fn rpc_client(&self) -> near_jsonrpc_client::JsonRpcClient {
        near_jsonrpc_client::new_client("https://rpc.testnet.near.org")
    }

    pub async fn process(&self, _parent_cli_args: &super::CliArgs) -> crate::CliResult {
        let online_signer_access_key_response = self
            .rpc_client()
            .query(near_primitives::rpc::RpcQueryRequest {
                block_reference: near_primitives::types::Finality::Final.into(),
                request: near_primitives::views::QueryRequest::ViewAccessKey {
                    account_id: self.signer_account_id.clone(),
                    public_key: self.signer_public_key.clone(),
                },
            })
            .await
            .map_err(|err| {
                color_eyre::Report::msg(format!(
                    "Failed to fetch public key information for nonce: {:?}",
                    err
                ))
            })?;
        let current_nonce =
            if let near_primitives::views::QueryResponseKind::AccessKey(online_signer_access_key) =
                online_signer_access_key_response.kind
            {
                online_signer_access_key.nonce
            } else {
                return Err(color_eyre::Report::msg(format!(
                    "Something unexpected was received instead of the access key information"
                )));
            };
        let unsigned_transaction = near_primitives::transaction::Transaction {
            signer_id: self.signer_account_id.clone(),
            public_key: self.signer_public_key.clone(),
            receiver_id: self.receiver_account_id.clone(),
            actions: vec![],
            block_hash: online_signer_access_key_response.block_hash,
            nonce: current_nonce + 1,
        };

        match &self.subcommand {
            CliSubCommand::Transfer(transfer_subcommand) => {
                transfer_subcommand
                    .process(self, unsigned_transaction)
                    .await
            }
        }
    }
}
