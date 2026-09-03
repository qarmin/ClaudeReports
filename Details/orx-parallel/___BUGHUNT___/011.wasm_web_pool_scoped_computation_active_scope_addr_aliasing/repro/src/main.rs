//! Standalone reproduction for candidate 011
//! (wasm_web_pool_scoped_computation_active_scope_addr_aliasing).
//!
//! `WasmWebPool::scoped_computation_impl` (src/pool/pool_impl/wasm_web.rs) publishes the
//! currently-active scope's `ScopeRuntime` as a single process-wide address
//! (`WorkerState::active_scope_addr`) instead of giving each queued task its own reference to
//! its scope's runtime. A second, nested (reentrant) call to `WasmWebPool::scoped_computation`
//! - made through the crate's real public `ParThreadPool::scoped_computation` entry point, while
//! the outer scope's call is still on the stack - overwrites that single slot. This is guarded
//! only by a `debug_assert!`, which is compiled out in release builds.
//!
//! This driver:
//! 1. builds three tiny wasm32+atomics binaries from `../clone` (a real, unmodified-except-for-
//!    a-whitebox-test-hook checkout of the crate - see `add_repro_hooks.diff`), each calling the
//!    real public `ParThreadPool::scoped_computation` trait method directly, at a different
//!    nesting depth (1 = no nesting, 2 = one nested call, 3 = two nested calls);
//! 2. runs each one under a real wasm engine (`wasmtime`, with the WASM threads/shared-memory
//!    proposal enabled) - the same execution semantics a browser's Worker-backed runtime relies
//!    on, just hosted without a browser;
//! 3. reports, for each, whether it completed normally or trapped.
//!
//! Before the fix: the non-nested case completes; both nested cases trap inside
//! `scoped_computation_impl` (the `debug_assert!` firing on the aliased slot). After the fix:
//! all three complete normally.

use std::path::{Path, PathBuf};
use std::process::Command;
use wasmtime::*;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn clone_dir() -> PathBuf {
    manifest_dir().join("..").join("clone")
}

const RUSTFLAGS: &str = "-C target-feature=+atomics,+bulk-memory,+mutable-globals \
     -C link-arg=--shared-memory -C link-arg=--max-memory=1073741824";

fn build_repro_bins(clone: &Path) -> anyhow::Result<()> {
    let status = Command::new("cargo")
        .current_dir(clone)
        .env("RUSTFLAGS", RUSTFLAGS)
        .args([
            "+nightly",
            "build",
            "--target",
            "wasm32-unknown-unknown",
            "--no-default-features",
            "--features",
            "wasm",
            "-Z",
            "build-std=panic_abort,std",
            "--bin",
            "bughunt_baseline",
            "--bin",
            "bughunt_nested",
            "--bin",
            "bughunt_depth3",
        ])
        .status()?;
    anyhow::ensure!(
        status.success(),
        "cargo build of the repro wasm binaries failed - see output above. \
         This needs: rustup toolchain install nightly, \
         rustup component add rust-src --toolchain nightly, \
         rustup target add wasm32-unknown-unknown --toolchain nightly."
    );
    Ok(())
}

enum Outcome {
    Completed(i32),
    Trapped(String),
}

/// Runs one of the built wasm32 binaries under a real wasm engine with the WASM
/// threads/shared-memory proposal enabled (no browser, no wasm-bindgen JS glue - just the
/// compiled module's own `main` export, called directly).
fn run_wasm(path: &Path) -> anyhow::Result<Outcome> {
    let mut config = Config::new();
    config.wasm_threads(true);
    let engine = Engine::new(&config)?;
    let module = Module::from_file(&engine, path)?;

    let mut linker = Linker::new(&engine);
    // wasm-bindgen "placeholder" imports pulled in because `#[wasm_bindgen]`-attributed
    // functions (e.g. `init_wasm_parallel_runtime`) stay force-alive in the export/import
    // tables even though this repro's own exercised entry point (a nested
    // `ParThreadPool::scoped_computation` call) never calls them. None of these stubs are
    // expected to actually run for the cases exercised here - they only exist so the module
    // instantiates at all.
    linker.func_wrap(
        "__wbindgen_placeholder__",
        "__wbg_startWorkers_6184905e191e0e72",
        |_: i32, _: i32, _: i32| -> i32 { 0 },
    )?;
    linker.func_wrap("__wbindgen_placeholder__", "__wbindgen_describe", |_: i32| {})?;
    linker.func_wrap(
        "__wbindgen_placeholder__",
        "__wbg_resolve_020f95d838c6ef25",
        |_: i32| -> i32 { 0 },
    )?;
    linker.func_wrap(
        "__wbindgen_placeholder__",
        "__wbg_reject_671a1c459689d0e0",
        |_: i32| -> i32 { 0 },
    )?;
    linker.func_wrap(
        "__wbindgen_placeholder__",
        "__wbindgen_describe_cast",
        |_: i32, _: i32| -> i32 { 0 },
    )?;
    linker.func_wrap(
        "__wbindgen_placeholder__",
        "__wbg___wbindgen_throw_bb96b2010945f0bc",
        |_: i32, _: i32| {},
    )?;
    linker.func_wrap(
        "__wbindgen_placeholder__",
        "__wbg___wbindgen_memory_5dc2a138835b0f8e",
        || -> i32 { 0 },
    )?;
    linker.func_wrap(
        "__wbindgen_placeholder__",
        "__wbg___wbindgen_module_d70c256490b5f616",
        || -> i32 { 0 },
    )?;
    linker.func_wrap(
        "__wbindgen_externref_xform__",
        "__wbindgen_externref_table_grow",
        |_: i32| -> i32 { 0 },
    )?;
    linker.func_wrap(
        "__wbindgen_placeholder__",
        "__wbindgen_object_drop_ref",
        |_: i32| {},
    )?;
    linker.func_wrap(
        "__wbindgen_externref_xform__",
        "__wbindgen_externref_table_set_null",
        |_: i32| {},
    )?;

    let mut store = Store::new(&engine, ());
    let instance = linker.instantiate(&mut store, &module)?;
    let main_fn = instance.get_typed_func::<(i32, i32), i32>(&mut store, "main")?;

    Ok(match main_fn.call(&mut store, (0, 0)) {
        Ok(code) => Outcome::Completed(code),
        Err(trap) => Outcome::Trapped(format!("{trap:?}")),
    })
}

fn main() -> anyhow::Result<()> {
    let clone = clone_dir();
    anyhow::ensure!(
        clone.join("Cargo.toml").is_file(),
        "expected a checkout at {} (run `just clone` in the candidate folder first)",
        clone.display()
    );

    println!("Building repro wasm binaries from {} ...", clone.display());
    build_repro_bins(&clone)?;

    let out_dir = clone.join("target/wasm32-unknown-unknown/debug");
    let cases = [
        ("bughunt_baseline.wasm", "single scope, no nesting"),
        (
            "bughunt_nested.wasm",
            "scope nested inside another still-active scope (2 levels)",
        ),
        ("bughunt_depth3.wasm", "scope nested 3 levels deep"),
    ];

    println!();
    println!("=== Results ===");
    let mut any_trap = false;
    for (file, desc) in cases {
        let path = out_dir.join(file);
        match run_wasm(&path)? {
            Outcome::Completed(code) => {
                println!("{file:24} [{desc}]: completed normally (return code {code})");
            }
            Outcome::Trapped(msg) => {
                any_trap = true;
                println!("{file:24} [{desc}]: TRAPPED");
                for line in msg.lines().take(4) {
                    println!("    {line}");
                }
            }
        }
    }

    println!();
    if any_trap {
        println!(
            "==> at least one nested-scope case trapped inside scoped_computation_impl: the \
             single process-wide active-scope slot was aliased by the nested call (candidate 011, \
             pre-fix / unpatched behavior)."
        );
    } else {
        println!(
            "==> all cases (including the nested ones) completed normally: no aliasing observed \
             (candidate 011, post-fix / patched behavior)."
        );
    }

    Ok(())
}
