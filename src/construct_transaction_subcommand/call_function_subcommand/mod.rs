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
    method_name: String,
    json_args: JsonObject,
    #[clap(default_value = "100000000000000")]
    gas: near_primitives::types::Gas,
    #[clap(default_value = "0")]
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

        if let Some(ref secret_key) = parent_cli_args.signer_secret_key {
            let signature = secret_key.sign(unsigned_transaction.get_hash().as_ref());
            let signed_transaction = near_primitives::transaction::SignedTransaction::new(
                signature,
                unsigned_transaction,
            );
            println!("{:#?}", signed_transaction);
            println!(
                "Signed transaction in base64: {}",
                near_primitives::serialize::to_base64(&signed_transaction.try_to_vec()?)
            )
        } else {
            println!("{:#?}", unsigned_transaction);
        }
        Ok(())
    }
}
