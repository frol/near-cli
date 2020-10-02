use color_eyre::eyre::WrapErr;
use strum::VariantNames;

use near_primitives::borsh::{BorshDeserialize, BorshSerialize};

/// Given an unsigned transaction and a secret key corresponding to the
/// public key used in the construction of the unsigned transaction,
/// produce a signed transaction
#[derive(Debug, clap::Clap)]
#[clap(version, author, setting(clap::AppSettings::ColoredHelp))]
pub struct CliArgs {
    #[clap(short, long)]
    secret_key: String,
    #[clap(
        short,
        long,
        default_value=crate::common::TransactionFormat::default().into(),
        possible_values=crate::common::TransactionFormat::VARIANTS
    )]
    transaction_format: crate::common::TransactionFormat,
    unsigned_transaction: String,
}

impl CliArgs {
    pub async fn process(&self, _parent_cli_args: &super::CliArgs) -> crate::CliResult {
        let unsigned_transaction = {
            let unsigned_transaction_borsh = match self.transaction_format {
                crate::common::TransactionFormat::Base64 => {
                    base64::decode(&self.unsigned_transaction)
                        .wrap_err("Unable to parse base64-encoded transaction")?
                }
                crate::common::TransactionFormat::Hex => hex::decode(&self.unsigned_transaction)
                    .wrap_err("Unable to parse hex-encoded transaction")?,
            };
            near_primitives::transaction::Transaction::try_from_slice(&unsigned_transaction_borsh)
                .wrap_err("Unable to parse transaction")?
        };

        let secret_key: near_crypto::SecretKey = self.secret_key.parse().map_err(|_err| {
            color_eyre::Report::msg(format!(
                "Could not parse '{}' as a secret key",
                self.secret_key
            ))
        })?;

        let signature = secret_key.sign(unsigned_transaction.get_hash().as_ref());

        let signed_transaction =
            near_primitives::transaction::SignedTransaction::new(signature, unsigned_transaction);

        println!(
            "The transaction has been successfully signed:\n{:#?}",
            signed_transaction
        );
        println!(
            "Base64-encoded signed transaction: {}",
            base64::encode(signed_transaction.try_to_vec()?)
        );
        println!(
            "Hex-encoded signed transaction: {}",
            hex::encode(signed_transaction.try_to_vec()?)
        );

        Ok(())
    }
}
