use std::convert::TryInto;

/// Transfer tokens in NEAR protocol
#[derive(Debug, clap::Clap)]
#[clap(version, author, setting(clap::AppSettings::ColoredHelp))]
pub struct CliArgs {
    tokens_to_transfer: u128,
}

impl CliArgs {
    pub fn process(&self, parent_cli_args: &super::CliArgs) -> crate::CliResult {
        let signer_id = parent_cli_args.signer_account_id.clone();
        let public_key = parent_cli_args
            .signer_public_key
            .as_str()
            .try_into()
            .map_err(|err| {
                color_eyre::Report::msg(format!("Public key could not be parsed: {}", err))
            })?;
        let secret_key: near_crypto::SecretKey = parent_cli_args
            .signer_secret_key
            .parse()
            .map_err(|_err| color_eyre::Report::msg(format!("Secret key could not be parsed")))?;
        let block_hash = "5dMs1XfiUWCaP4USZ4EuSY6oLzm3w2FBNZeQu29H55sa"
            .try_into()
            .unwrap(); //fetch_latest_block_hash();
        let nonce = 10; //fetch_access_key(signer_account_id, signer_public_key).nonce;
        let unsigned_transaction = near_primitives::transaction::Transaction {
            signer_id,
            public_key,
            receiver_id: parent_cli_args.receiver_account_id.clone(),
            actions: vec![near_primitives::transaction::Action::Transfer(
                near_primitives::transaction::TransferAction {
                    deposit: self.tokens_to_transfer,
                },
            )],
            block_hash,
            nonce,
        };
        let signature = secret_key.sign(unsigned_transaction.get_hash().as_ref());
        let signed_transaction =
            near_primitives::transaction::SignedTransaction::new(signature, unsigned_transaction);
        println!("Transaction: {:#?}", signed_transaction);
        //submit(signed_transaction);
        Ok(())
    }
}
