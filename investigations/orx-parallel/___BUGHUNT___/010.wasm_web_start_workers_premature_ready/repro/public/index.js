// Stub stand-in for the real wasm-bindgen generated package module
// ("pkg.js"), simulating a build that does NOT export wasm_web_start_worker
// (e.g. a mismatched/non-atomics artifact loaded into the worker) but whose
// default init function still succeeds. Deliberately does NOT export
// wasm_web_start_worker, matching candidate 010's trigger condition.
export default async function init(_init) {
    return true;
}
// Note: wasm_web_start_worker is intentionally absent.
