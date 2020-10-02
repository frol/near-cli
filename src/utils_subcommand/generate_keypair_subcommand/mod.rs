use strum::VariantNames;

/// Generate a key pair of secret and public keys (use it anywhere you need
/// Ed25519 keys)
#[derive(Debug, clap::Clap)]
#[clap(version, author, setting(clap::AppSettings::ColoredHelp))]
pub struct CliArgs {
    #[clap(
        short,
        long,
        default_value=crate::common::OutputFormat::default().into(),
        possible_values=crate::common::OutputFormat::VARIANTS
    )]
    format: crate::common::OutputFormat,
}

impl CliArgs {
    pub async fn process(&self, _parent_cli_args: &super::CliArgs) -> crate::CliResult {
        let secret_key = near_crypto::SecretKey::from_random(near_crypto::KeyType::ED25519);
        let public_key = secret_key.public_key();
        let account_id = hex::encode(secret_key.public_key().key_data());
        match self.format {
            crate::common::OutputFormat::Plaintext => {
                println!(
                    "Account ID: {}\nPublic Key: {}\nSECRET KEY: {}",
                    account_id, public_key, secret_key,
                );
            }
            crate::common::OutputFormat::Json => {
                println!(
                    "{}",
                    serde_json::json!({
                        "account_id": account_id,
                        "public_key": public_key,
                        "secret_key": secret_key,
                    })
                );
            }
        }
        Ok(())
    }
}
