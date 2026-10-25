# timeout in pbkdf2::pbkdf2_hmac_with_params

- crate: https://github.com/RustCrypto/password-hashes @ 958527c7d855e1687e28f9e592e537003833ed1e
- kind: timeout
- last API call: `pbkdf2::pbkdf2_hmac_with_params`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling pbkdf2::pbkdf2_hmac_with_params
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
pbkdf2 = { git = "https://github.com/RustCrypto/password-hashes", rev = "958527c7d855e1687e28f9e592e537003833ed1e", features = ["alloc", "getrandom", "hmac", "kdf", "mcf", "password-hash", "phc", "rand_core", "sha2"] }
```

`src/main.rs`:

```rust
use pbkdf2::Algorithm;
use pbkdf2::Params;

fn main() {
    let params = Params::recommended_for(Algorithm::default());
    let algorithm = Algorithm::Pbkdf2Sha256;
    pbkdf2::pbkdf2_hmac_with_params(&vec![], &vec![], algorithm, params, &mut include_bytes!("image_0.webp").to_vec());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_pbkdf2_pbkdf2_hmac_with_params_fd74fb`).

