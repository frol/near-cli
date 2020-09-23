use strum::VariantNames;

/// Submit a signed transaction to the network
#[derive(Debug, clap::Clap)]
#[clap(version, author, setting(clap::AppSettings::ColoredHelp))]
pub struct CliArgs {
    signed_transaction: String,
}

impl CliArgs {
    pub fn process(&self, _parent_cli_args: &super::CliArgs) -> crate::CliResult {
        let signed_transaction = self.signed_transaction.clone();

        let transaction_broadcast_result = actix::System::builder().build().block_on(async {
            let client = near_jsonrpc_client::new_client("https://rpc.testnet.near.org");
            client
                .broadcast_tx_async(signed_transaction)
                .await
                .map_err(|err| {
                    color_eyre::Report::msg(format!(
                        "Could not broadcast the transaction: {:?}",
                        err
                    ))
                })
        })?;

        println!(
            "Transaction has been submitted: {}",
            transaction_broadcast_result
        );
        Ok(())
    }
}
