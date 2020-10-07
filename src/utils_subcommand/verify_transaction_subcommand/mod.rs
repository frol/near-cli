use color_eyre::eyre::WrapErr;
use strum::VariantNames;

use near_primitives::borsh::BorshDeserialize;

/// Verify that the specified signed transaction is signed with the right key
///
/// asd
#[derive(Debug, clap::Clap)]
#[clap(version, author, setting(clap::AppSettings::ColoredHelp))]
pub struct CliArgs {
    #[clap(
        short,
        long,
        default_value=crate::common::TransactionFormat::default().into(),
        possible_values=crate::common::TransactionFormat::VARIANTS
    )]
    transaction_format: crate::common::TransactionFormat,
    #[clap(short, long = "public-key")]
    public_keys: Vec<near_crypto::PublicKey>,
    signed_transaction: String,
}

impl CliArgs {
    pub async fn process(&self, _parent_cli_args: &super::CliArgs) -> crate::CliResult {
        let signed_transaction = {
            let signed_transaction_borsh = match self.transaction_format {
                crate::common::TransactionFormat::Base64 => {
                    base64::decode(&self.signed_transaction)
                        .wrap_err("Unable to parse base64-encoded transaction")?
                }
                crate::common::TransactionFormat::Hex => hex::decode(&self.signed_transaction)
                    .wrap_err("Unable to parse hex-encoded transaction")?,
            };
            near_primitives::transaction::SignedTransaction::try_from_slice(
                &signed_transaction_borsh,
            )
            .wrap_err("Unable to parse transaction")?
        };

        if !near_primitives::transaction::verify_transaction_signature(
            &signed_transaction,
            &self.public_keys,
        ) {
            Err(color_eyre::Report::msg(
                "Transaction is NOT signed with any of the public keys.",
            ))
        } else {
            println!("The transaction is valid.");
            Ok(())
        }
    }
}
