use rand::rngs::OsRng;
use secp256k1::SECP256K1;
use std::{io::Write, path::Path};

fn main() {
    let node_path = Path::new("./output"); // Define the output_path variable
    std::fs::create_dir_all(&node_path);
    // Create the secret key
    let mut rng = OsRng;
    let sk = secp256k1::SecretKey::new(&mut rng);
    let pk = sk.public_key(SECP256K1);

    // Write the public key
    let public_key_path = Path::new(&node_path).join("public-key");
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

    // Lastly we need to create the jwt secret key
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
}
