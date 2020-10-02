use std::convert::TryInto;

mod transfer_subcommand;

#[derive(derive_more::AsRef)]
struct BlobAsBase58String<T>
where
    for<'a> T: std::convert::TryFrom<&'a [u8]> + AsRef<[u8]>,
{
    inner: T,
}

impl<T> std::fmt::Debug for BlobAsBase58String<T>
where
    for<'a> T: std::convert::TryFrom<&'a [u8]> + AsRef<[u8]>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        near_primitives::serialize::to_base(self.inner.as_ref()).fmt(f)
    }
}

impl<T> std::str::FromStr for BlobAsBase58String<T>
where
    for<'a> T: std::convert::TryFrom<&'a [u8]> + AsRef<[u8]>,
{
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            inner: near_primitives::serialize::from_base(value)
                .map_err(|_| "err")?
                .as_slice()
                .try_into()
                .map_err(|_| "err")?,
        })
    }
}

impl<T> BlobAsBase58String<T>
where
    for<'a> T: std::convert::TryFrom<&'a [u8]> + AsRef<[u8]>,
{
    fn into_inner(self) -> T {
        self.inner
    }
}

/// Construct transactions (like transfer tokens, call a function, etc) ready to
/// be sent to NEAR protocol network
///
/// Construction just prepares a transaction, but does not submit (send) it. You
/// may want to use the following utils after constructing a transaction:
///
/// * `near-cli utils sign-transaction`
/// * `near-cli utils submit-raw-transaction`
#[derive(Debug, clap::Clap)]
#[clap(version, author, setting(clap::AppSettings::ColoredHelp))]
pub struct CliArgs {
    #[clap(long)]
    base_block_hash: BlobAsBase58String<near_primitives::hash::CryptoHash>,
    #[clap(long)]
    signer_account_id: near_primitives::types::AccountId,
    #[clap(long)]
    signer_public_key: near_crypto::PublicKey,
    #[clap(long)]
    signer_public_key_nonce: near_primitives::types::Nonce,
    #[clap(long)]
    signer_secret_key: Option<near_crypto::SecretKey>,
    #[clap(long)]
    receiver_account_id: String,
    #[clap(subcommand)]
    subcommand: CliSubCommand,
}

#[derive(Debug, clap::Clap)]
pub enum CliSubCommand {
    Transfer(transfer_subcommand::CliArgs),
}

impl CliArgs {
    pub async fn process(&self, _parent_cli_args: &super::CliArgs) -> crate::CliResult {
        let unsigned_transaction = near_primitives::transaction::Transaction {
            signer_id: self.signer_account_id.clone(),
            public_key: self.signer_public_key.clone(),
            receiver_id: self.receiver_account_id.clone(),
            actions: vec![],
            block_hash: self.base_block_hash.as_ref().clone(),
            nonce: self.signer_public_key_nonce,
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
