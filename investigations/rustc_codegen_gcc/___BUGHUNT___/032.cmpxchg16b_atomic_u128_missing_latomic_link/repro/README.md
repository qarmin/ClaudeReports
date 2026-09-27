# repro for candidate 032 (cmpxchg16b_atomic_u128_missing_latomic_link)

`atomic128.rs`: a single, `std`-only source file compiled through the real, unmodified backend
entry point (`y.sh rustc`, i.e. `rustc -Zcodegen-backend=librustc_codegen_gcc.so`) with
`-C target-feature=+cmpxchg16b`, and run for real.

Before the fix: linking fails with `undefined reference to \`__atomic_compare_exchange_16'` and
`undefined reference to \`__atomic_fetch_add_16'` - exactly matching the candidate's report -
regardless of whether `-C target-feature=+cmpxchg16b` is passed (identical failure either way, see
`output.txt`). A build of the exact same source with plain host `rustc`/LLVM only succeeds *with*
`+cmpxchg16b` (LLVM inlines native `cmpxchg16b` and never references the libcall symbol at all);
without the feature, plain LLVM fails with the identical undefined-reference error - so LLVM's
"inline when the feature is enabled, libcall otherwise" behavior is the actual point of comparison,
not "LLVM always links fine".

After the fix (`patch.diff`, cherry-picked from upstream's own not-yet-merged
`fix/atomic-undefined` branch, commit 72b88cf9): the same command links and runs, printing
`cmpxchg: old=1 ok=true x=2`, `fetch_add: prev=10 y=15`, `OK`. Without `+cmpxchg16b`, it still
correctly falls back to the libcall (matching LLVM's own behavior in that case) and fails to link
for the same reason as before - this is expected and correct, not a regression.

## Why this doesn't use `std::sync::atomic::AtomicU128` directly

`AtomicU128`/`AtomicI128` require `#[cfg(target_has_atomic_load_store = "128")]`, which is false
for `x86_64-unknown-linux-gnu` (`rustc -Z unstable-options --print target-spec-json --target
x86_64-unknown-linux-gnu` prints `"max-atomic-width": 64`) - independent of any `-C target-feature`
flag, on both LLVM and this backend. So the exact code the original candidate named
(`AtomicU128::compare_exchange` with `+cmpxchg16b`) is a compile error (`E0432`/`E0635`) on this
target regardless of backend, and isn't the real trigger path here. `core::intrinsics::atomic_cxchg`/
`atomic_xadd::<u128, ...>` (used above) are the exact same compiler intrinsics `AtomicU128`'s
methods desugar to on a target where the type *is* available (e.g. `aarch64-unknown-linux-gnu`,
`x86_64-apple-darwin`, both `max-atomic-width: 128`) - calling them directly bypasses only the
`target_has_atomic` cfg gate on the *type*, not the codegen path itself, so this is a real
end-to-end run through the real compiler and the real `src/builder.rs` lowering, not a hand-built
stand-in. See `RESULT.md` for the full reasoning.
