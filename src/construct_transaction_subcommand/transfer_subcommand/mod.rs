use near_primitives::borsh::BorshSerialize;

/// Transfer tokens in NEAR protocol
#[derive(Debug, clap::Clap)]
#[clap(version, author, setting(clap::AppSettings::ColoredHelp))]
pub struct CliArgs {
    tokens_to_transfer: u128,
}

impl CliArgs {
    pub fn process(
        &self,
        parent_cli_args: &super::CliArgs,
        prepopulated_unsigned_transaction: near_primitives::transaction::Transaction,
    ) -> crate::CliResult {
        let unsigned_transaction = near_primitives::transaction::Transaction {
            actions: vec![near_primitives::transaction::Action::Transfer(
                near_primitives::transaction::TransferAction {
                    deposit: self.tokens_to_transfer,
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
