use base64::{engine::general_purpose::STANDARD as base64, Engine};
use ethers::{
    core::k256::ecdsa::SigningKey,
    signers::{LocalWallet, Signer},
};
use rand::rngs::OsRng;
use secp256k1::SECP256K1;
use serde::Deserialize;
use std::{fs, io::Write, path::Path};

fn main() {
    let node_path = Path::new("./output"); // Define the output_path variable
    std::fs::create_dir_all(&node_path).expect("create output directory");
    // Create the secret key
    let mut rng = OsRng;
    let sk = secp256k1::SecretKey::new(&mut rng);
    let pk = sk.public_key(SECP256K1);

    // Write the public key
    let public_key_path = Path::new(&node_path).join("federation-public-key");
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(public_key_path.clone())
        .expect("public key file cannot be created/opened");
    let _ = file
        .write_all(&pk.to_string().as_bytes())
        .expect("error writing public key to file");

    // Write the discovery secret key
    let discovery_secret_path = Path::new(&node_path).join("discovery-secret");
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(discovery_secret_path.clone())
        .expect("discovery secret file cannot be created/opened");
    let _ = file
        .write_all(&sk.display_secret().to_string().as_bytes())
        .expect("error writing secret key to file");

    // Create and write the jwt
    let jwt_secret_path = Path::new(&node_path).join("bjwt.hex");
    let jwt_sk = secp256k1::SecretKey::new(&mut rng);
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(jwt_secret_path.clone())
        .expect("jwt secret file cannot be created/opened");
    let _ = file
        .write_all(&jwt_sk.display_secret().to_string().as_bytes())
        .expect("error writing jwt secret key to file");

    // Create the EVM private key and address from CometBFT from `priv_validator_key.json`
    // The address is where the block fees will be sent
    let priv_validator_key_path = Path::new("./priv_validator_key.json");
    let priv_validator_key =
        extract_priv_key(priv_validator_key_path).expect("error reading priv_validator_key.json");
    let (evm_priv_key, evm_address) =
        create_evm_key_and_address(priv_validator_key).expect("error creating EVM key and address");

    // Write the EVM (block builder) private key
    let evm_priv_key_path = Path::new(&node_path).join("block_builder_priv_key");
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(evm_priv_key_path.clone())
        .expect("EVM private key file cannot be created/opened");
    let _ = file
        .write_all(&evm_priv_key.as_bytes())
        .expect("error writing EVM private key to file");

    // Write the EVM(block builder) address
    let evm_address_path = Path::new(&node_path).join("block_builder_address");
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(evm_address_path.clone())
        .expect("EVM address file cannot be created/opened");
    let _ = file
        .write_all(&evm_address.as_bytes())
        .expect("error writing EVM address to file");
}

#[derive(Debug, Deserialize)]
struct PrivValidatorKey {
    priv_key: KeyEntry,
}

#[derive(Debug, Deserialize)]
struct KeyEntry {
    value: String,
}

fn extract_priv_key<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let parsed: PrivValidatorKey = serde_json::from_str(&contents)?;
    Ok(parsed.priv_key.value)
}

fn create_evm_key_and_address(
    priv_validator_key: String,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    // Decode the base64-encoded 32-byte private key
    let privkey_bytes = base64
        .decode(priv_validator_key)
        .expect("Invalid base64 priv key");
    assert_eq!(privkey_bytes.len(), 32, "Expected 32-byte private key");

    // Create a SigningKey from bytes and then a LocalWallet
    let signing_key = SigningKey::from_slice(privkey_bytes.as_slice()).expect("Valid secret key");
    let wallet: LocalWallet = signing_key.into();

    Ok((
        hex::encode(wallet.signer().to_bytes()),
        hex::encode(wallet.address().as_bytes()),
    ))
}
