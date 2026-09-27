# Repro scripts for candidate 006 (type_uint_from_ty_inverted_signedness)

Both files are standalone Rust source files (no external crates, `std`-using but not depending on
`mini_core`), compiled directly with the codegen-under-test via the project's own `y.sh rustc`
entry point - the real, unmodified end-to-end path a user hits when compiling any Rust program
with this backend (`rustc -Zcodegen-backend=librustc_codegen_gcc.so`, wrapped by `y.sh rustc`).

- `simd_scatter_u32.rs` - `simd_scatter` (`core::intrinsics::simd::simd_scatter`) writing a
  `u32x4` vector through a `*mut u32` pointer vector with an all-true `i32x4` mask. This exercises
  `src/intrinsic/simd.rs`'s `simd_scatter` lowering when `pointer_count == 1`, which calls the
  local `vector_ty` helper (`src/intrinsic/simd.rs:903`) on the pointee's Rust type
  (`ty::Uint(UintTy::U32)`), which calls `CodegenCx::type_uint_from_ty` (`src/type_of.rs:47`).
- `simd_scatter_u8.rs` - the same shape at the narrower `u8` element width, to vary the input
  within the same suspected bug class (a different bit width hitting the same code path) rather
  than relying on a single triggering input.

Both fail to compile against the unpatched backend with a hard libgccjit type-mismatch error at
`gcc_jit_block_add_assignment` (see `../output.txt`), and both compile and run correctly
(printing nothing, exiting 0, with the assertion inside each file passing) once `patch.diff` is
applied.

## Running

Requires the project's own toolchain already set up (see `___BUGHUNT___/README.md`) - a prebuilt
`libgccjit.so` and a built sysroot (`core`/`alloc`/`std`) under `build/`.

```
./y.sh prepare
./y.sh build --sysroot --release
CHANNEL=release ./y.sh rustc repro/simd_scatter_u32.rs -o /tmp/simd_scatter_u32 && /tmp/simd_scatter_u32; echo "exit: $?"
CHANNEL=release ./y.sh rustc repro/simd_scatter_u8.rs -o /tmp/simd_scatter_u8 && /tmp/simd_scatter_u8; echo "exit: $?"
```

Against the unpatched tree, the `./y.sh rustc ...` compile step itself fails (libgccjit error,
linker never runs) - there is no executable to run. Against the patched tree, both compile
cleanly and exit 0. See `../output.txt` for the actual captured before/after output and
`../justfile` for a fully self-contained clone-and-verify version of the same steps.
