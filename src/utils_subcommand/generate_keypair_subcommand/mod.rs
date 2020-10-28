use strum::VariantNames;

fn bip32path_to_string(bip32path: &slip10::BIP32Path) -> String {
    const HARDEND: u32 = 1 << 31;

    format!(
        "m/{}",
        (0..bip32path.depth())
            .map(|index| {
                let value = *bip32path.index(index).unwrap();
                if value < HARDEND {
                    value.to_string()
                } else {
                    format!("{}'", value - HARDEND)
                }
            })
            .collect::<Vec<String>>()
            .join("/")
    )
}

/// Generate a key pair of secret and public keys (use it anywhere you need
/// Ed25519 keys)
#[derive(Debug, clap::Clap)]
#[clap(version, author, setting(clap::AppSettings::ColoredHelp))]
pub struct CliArgs {
    #[clap(long)]
    master_seed_phrase: Option<String>,
    #[clap(long, default_value = "12")]
    new_master_seed_phrase_words_count: usize,
    #[clap(long, default_value = "m/44'/397'/0'")]
    seed_phrase_hd_path: slip10::BIP32Path,
    #[clap(
        short,
        long,
        default_value = crate::common::OutputFormat::default().into(),
        possible_values = crate::common::OutputFormat::VARIANTS
    )]
    format: crate::common::OutputFormat,
}

impl CliArgs {
    pub async fn process(&self, _parent_cli_args: &super::CliArgs) -> crate::CliResult {
        let (master_seed_phrase, master_seed) =
            if let Some(ref master_seed_phrase) = self.master_seed_phrase {
                (
                    master_seed_phrase.clone(),
                    bip39::Mnemonic::parse(master_seed_phrase)?.to_seed(""),
                )
            } else {
                let mnemonic = bip39::Mnemonic::generate(self.new_master_seed_phrase_words_count)?;
                (mnemonic.as_str().to_owned(), mnemonic.to_seed(""))
            };

        let derived_private_key = slip10::derive_key_from_path(
            &master_seed,
            slip10::Curve::Ed25519,
            &self.seed_phrase_hd_path,
        )
        .map_err(|err| {
            color_eyre::Report::msg(format!("Key derivation from path failed: {:?}", err))
        })?;

        let secret_keypair = {
            let secret = ed25519_dalek::SecretKey::from_bytes(&derived_private_key.key)?;
            let public = ed25519_dalek::PublicKey::from(&secret);
            ed25519_dalek::Keypair { secret, public }
        };

        let implicit_account_id = hex::encode(&secret_keypair.public);
        let public_key_str = format!(
            "ed25519:{}",
            bs58::encode(&secret_keypair.public).into_string()
        );
        let secret_keypair_str = format!(
            "ed25519:{}",
            bs58::encode(secret_keypair.to_bytes()).into_string()
        );

        match self.format {
            crate::common::OutputFormat::Plaintext => {
                println!(
                    "Master Seed Phrase: {}\nSeed Phrase HD Path: {}\nImplicit Account ID: {}\nPublic Key: {}\nSECRET KEYPAIR: {}",
                    master_seed_phrase,
                    bip32path_to_string(&self.seed_phrase_hd_path),
                    implicit_account_id,
                    public_key_str,
                    secret_keypair_str,
                );
            }
            crate::common::OutputFormat::Json => {
                println!(
                    "{}",
                    serde_json::json!({
                        "master_seed_phrase": master_seed_phrase,
                        "seed_phrase_hd_path": bip32path_to_string(&self.seed_phrase_hd_path),
                        "account_id": implicit_account_id,
                        "public_key": public_key_str,
                        "private_key": secret_keypair_str,
                    })
                );
            }
        }
        Ok(())
    }
}
