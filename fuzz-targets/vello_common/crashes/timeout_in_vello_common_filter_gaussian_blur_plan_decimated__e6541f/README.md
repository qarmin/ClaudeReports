# timeout in vello_common::filter::gaussian_blur::plan_decimated_blur

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: timeout
- last API call: `vello_common::filter::gaussian_blur::plan_decimated_blur`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 7620207697338833695

## Message

```
process ended abnormally (timeout) while calling vello_common::filter::gaussian_blur::plan_decimated_blur
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = vello_common::filter::gaussian_blur::plan_decimated_blur(f32::INFINITY);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_vello_common_filter_gaussian_blur_plan_decimated__e6541f`).

