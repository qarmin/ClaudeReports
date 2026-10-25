# tiny-skia/src/scan/path.rs:221:13

- crate: https://github.com/linebender/tiny-skia @ 5d4754777746eef0828be166896eaf482c49f8f2
- kind: panic
- location: `tiny-skia/src/scan/path.rs:221:13`
- last API call: `tiny_skia::Mask::fill_path`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: edges[curr_idx].last_y >= curr_y as i32
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
tiny-skia = { git = "https://github.com/linebender/tiny-skia", rev = "5d4754777746eef0828be166896eaf482c49f8f2" }
tiny-skia-path = { git = "https://github.com/linebender/tiny-skia", rev = "5d4754777746eef0828be166896eaf482c49f8f2" }
```

`src/main.rs`:

```rust
use tiny_skia::FillRule;
use tiny_skia::Mask;
use tiny_skia_path::PathBuilder;
use tiny_skia_path::Rect;
use tiny_skia_path::Transform;

fn main() {
    let transform = Transform::from_skew(1.0f32, 1.0f32);
    let transform2 = transform.pre_rotate(0.0f32);
    let bool_v = transform2.has_skew();
    let fill_rule = FillRule::default();
    let path = PathBuilder::from_rect(Rect::from_ltrb(-1e30f32, 0.0f32, 0.0f32, 1.0f32).unwrap());
    let mut mask = Mask::decode_png(&include_bytes!("data_0.bin").to_vec()).unwrap();
    mask.fill_path(&path, fill_rule, bool_v, transform);
}
```

Run with `cargo run` in this directory (package `repro_tiny_skia_src_scan_path_rs_221_13_11b844`).

## Output

```
thread 'main' (20714) panicked at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/src/scan/path.rs:221:13:
assertion failed: edges[curr_idx].last_y >= curr_y as i32
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: tiny_skia::scan::path::walk_edges
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/src/scan/path.rs:221:13
   4: tiny_skia::scan::path::fill_path_impl
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/src/scan/path.rs:188:5
   5: tiny_skia::scan::path_aa::fill_path_impl
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/src/scan/path_aa.rs:120:5
   6: tiny_skia::scan::path_aa::fill_path
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/src/scan/path_aa.rs:73:5
   7: <tiny_skia::mask::Mask>::fill_path
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/src/mask.rs:331:21
   8: <tiny_skia::mask::Mask>::fill_path
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/src/mask.rs:345:18
   9: repro_tiny_skia_src_scan_path_rs_221_13_11b844::main
             at ./src/main.rs:14:10
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
