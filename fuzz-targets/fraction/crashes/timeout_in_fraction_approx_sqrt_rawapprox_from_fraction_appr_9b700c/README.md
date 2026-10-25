# timeout in fraction::approx::sqrt::RawApprox::from<fraction::approx::sqrt::RawApprox>

- crate: https://github.com/dnsl48/fraction @ 53d04cddb4640d1fe7756c289b3ce34be14da504
- kind: timeout
- last API call: `fraction::approx::sqrt::RawApprox::from<fraction::approx::sqrt::RawApprox>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling fraction::approx::sqrt::RawApprox::from<fraction::approx::sqrt::RawApprox>
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
fraction = { git = "https://github.com/dnsl48/fraction", rev = "53d04cddb4640d1fe7756c289b3ce34be14da504", features = ["byteorder", "bytes", "juniper", "num-bigint", "postgres-types", "serde", "serde_derive", "with-approx", "with-bigint", "with-decimal", "with-dynaint", "with-juniper-support", "with-postgres-support", "with-serde-support", "with-unicode"] }
```

`src/main.rs`:

```rust
use fraction::approx::sqrt::RawApprox;

fn main() {
    let _ = fraction::generic::read_generic_integer::<fraction::BigUint, u16>(22u16);
    let _ = RawApprox::from(<fraction::prelude::GenericFraction<u16> as fraction::Zero>::zero().sqrt_abs_raw(4080515371u32));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_fraction_approx_sqrt_rawapprox_from_fraction_appr_9b700c`).

