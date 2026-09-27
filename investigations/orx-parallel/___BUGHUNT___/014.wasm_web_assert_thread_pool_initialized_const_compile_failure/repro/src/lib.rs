// Downstream crate exercising the documented wasm-browser setup from docs/wasm.md:
// `orx-parallel = { default-features = false, features = ["wasm"] }`, built for wasm32.
// This is the normal wasm-pack invocation *before* opting into the extra
// `-C target-feature=+atomics` RUSTFLAGS the docs describe as an additional, separate step.
use orx_parallel::WasmWebPool;

#[allow(dead_code)]
fn use_pool() -> WasmWebPool {
    WasmWebPool::default()
}
