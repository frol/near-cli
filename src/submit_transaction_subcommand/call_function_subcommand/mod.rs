use near_primitives::borsh::BorshSerialize;

#[derive(Debug)]
struct JsonObject(serde_json::Map<String, serde_json::Value>);

impl std::str::FromStr for JsonObject {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(serde_json::from_str(s)?))
    }
}

impl JsonObject {
    fn to_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        Ok(serde_json::to_string(&self.0)?.into_bytes())
    }
}

/// Transfer tokens in NEAR protocol
#[derive(Debug, clap::Clap)]
#[clap(version, author, setting(clap::AppSettings::ColoredHelp))]
pub struct CliArgs {
    #[clap(long)]
    method_name: String,
    #[clap(long)]
    json_args: JsonObject,
    #[clap(long, default_value = "100000000000000")]
    gas: near_primitives::types::Gas,
    #[clap(long, default_value = "0")]
    deposit: near_primitives::types::Balance,
}

impl CliArgs {
    pub async fn process(
        &self,
        parent_cli_args: &super::CliArgs,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        let unsigned_transaction = near_primitives::transaction::Transaction {
            actions: vec![near_primitives::transaction::Action::FunctionCall(
                near_primitives::transaction::FunctionCallAction {
                    method_name: self.method_name.clone(),
                    args: self.json_args.to_bytes()?,
                    gas: self.gas,
                    deposit: self.deposit,
                },
            )],
            ..prepopulated_unsigned_transaction
        };

        let signature = parent_cli_args
            .signer_secret_key
            .sign(unsigned_transaction.get_hash().as_ref());
        let signed_transaction =
            near_primitives::transaction::SignedTransaction::new(signature, unsigned_transaction);
        println!("{:#?}", signed_transaction);
        println!(
            "Signed transaction in base64: {}",
            near_primitives::serialize::to_base64(&signed_transaction.try_to_vec()?)
        );

        let transaction_info = parent_cli_args
            .rpc_client()
            .broadcast_tx_commit(near_primitives::serialize::to_base64(
                signed_transaction
                    .try_to_vec()
                    .expect("Transaction is not expected to fail on serialization"),
            ))
            .await
            .map_err(|err| {
                color_eyre::Report::msg(format!("Transaction submition failed: {:?}", err))
            })?;

        println!("Success: {:#?}", transaction_info);
        Ok(())
    }
}
