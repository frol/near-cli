use strum::VariantNames;

/// Generate a key pair of secret and public keys (use it anywhere you need Ed25519 keys)
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
    pub fn process(&self, _parent_cli_args: &super::CliArgs) -> crate::CliResult {
        let secret_key = near_crypto::SecretKey::from_random(near_crypto::KeyType::ED25519);
        match self.format {
            crate::common::OutputFormat::Plaintext => {
                println!(
                    "SECRET KEY: {}\nPublic Key: {}",
                    secret_key,
                    secret_key.public_key()
                );
            }
            crate::common::OutputFormat::Json => {
                println!(
                    "{}",
                    serde_json::json!({ "public_key": secret_key.public_key(), "secret_key": secret_key })
                );
            }
        }
        Ok(())
    }
}
