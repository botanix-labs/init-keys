
# README

This project generates cryptographic keys and writes them to files in a specified directory. The keys include:

1. **Federation Public Key**: The public key derived from a randomly generated secret key using the secp256k1 elliptic curve.
2. **Discovery Secret Key**: The secret key for the discovery protocol and P2P communication.
3. **JWT Secret Key**: A secret key used for JSON Web Tokens (JWT).
4. **Block Builder Private Key**: The private key used for the EVM (block builder) address.
5. **Block Builder Address**: The associated address where block fees will be sent, derived from the CometBFT `priv_validator_key.json`.

## Prerequisites

To run this project, ensure that you have the following dependencies installed:

1. **Rust**: You need to have Rust installed. You can install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

2. **Dependencies**:
    - `secp256k1`: The cryptographic library for generating public and private keys.
    - `rand`: Used for random number generation.
    - `ethers`: Used for creating the EVM key and address.
    - `serde`: Used for deserializing JSON.
    - `serde_json`: Used for deserializing JSON.
    - `base64`: For decoding base64-encoded keys.
    - `hex`: For encoding keys in hexadecimal format.

You can install the necessary dependencies by adding the following to your `Cargo.toml`:

```toml
[dependencies]
rand = "0.8.5"
secp256k1 = { version = "0.29", features = ["global-context", "rand-std", "recovery"]}
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
ethers = "2.0.14"
base64 = "0.21"
hex = "0.4"
```

## Instructions

1. **Clone the repository** (if applicable) or navigate to your project directory.

2. **Compile and run**:

    To compile and run the program, use the following command:

    ```bash
    cargo run
    ```

    This will:

    - Create an output directory (if it doesn't exist).
    - Generate five keys:
        - Federation Public Key (federation-public-key)
        - Discovery Secret Key (discovery-secret)
        - JWT secret key (`bjwt.hex`)
        - Block Builder Private Key (block_builder_priv_key)
        - Block Builder Address (block_builder_address)
    - Save the keys into the respective files in the `./output` directory.

## File Outputs

After running the program, the following files will be generated in the `./output` directory:

1. `federation-public-key`: Contains the public key generated from the secp256k1 secret key.
2. `discovery-secret`: Contains the secret key used for the discovery protocol.
3. `bjwt.hex`: Contains the JWT secret key.
4. `block_builder_priv_key`: Contains the EVM (block builder) private key.
5. `block_builder_address`: Contains the EVM (block builder) address.

These keys are stored as plain text (hexadecimal) in the respective files.

## Example Output

After running the program, the `./output` directory might look something like this:

```bash
./output/
├── federation-public-key
├── discovery-secret
├── bjwt.hex
├── block_builder_priv_key
└── block_builder_address

```

## Notes

- **Security**: Make sure to keep the generated secret keys (especially the discovery secret and JWT secret) secure. Do not expose them in public repositories or share them openly.
- **Key Format**: The keys are stored in a human-readable format, but for production environments, consider converting them to a more secure format (e.g., binary or base64 encoding) as appropriate.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
