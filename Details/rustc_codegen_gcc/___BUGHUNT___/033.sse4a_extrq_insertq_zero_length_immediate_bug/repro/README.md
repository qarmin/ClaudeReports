# Repro script for candidate 033 (sse4a_extrq_insertq_zero_length_immediate_bug)

`zero_len.rs` is a standalone Rust source file (no external crates, `std` only), compiled directly
with the codegen-under-test via the project's own `y.sh rustc` entry point - the real, unmodified
end-to-end path a user hits when compiling any Rust program with this backend (`rustc
-Zcodegen-backend=librustc_codegen_gcc.so`, wrapped by `y.sh rustc`).

It calls `_mm_extracti_si64::<0, 0>`/`_mm_inserti_si64::<0, 0>` (the `LEN=0, IDX=0` boundary case
this candidate's hypothesis was about) inside a real `#[target_feature(enable = "sse4a")]`
function, with every runtime input passed through `std::hint::black_box` so the call can't be
constant-folded away by either backend's optimizer - the real SSE4a hardware instruction actually
executes at runtime. It checks 64 fixed boundary-value pairs plus 4000 pseudo-random pairs (2000
each for extract and insert) against the ISA-documented expected result (`LEN=0` means "all 64
bits", per stdarch's own doc comment on these intrinsics).

Requires a host CPU with `sse4a` (checked at runtime via `is_x86_feature_detected!`; prints `SKIP`
and exits cleanly if unavailable).

## Running

Requires the project's own toolchain already set up (see `___BUGHUNT___/README.md`):

```
./y.sh prepare
./y.sh build --sysroot --release

# Reference oracle: plain rustc (LLVM backend)
rustc --edition 2021 -O repro/zero_len.rs -o /tmp/zero_len_llvm && /tmp/zero_len_llvm

# Under test: this backend
CHANNEL=release ./y.sh rustc repro/zero_len.rs -o /tmp/zero_len_gcc && /tmp/zero_len_gcc
```

Both currently print `total=4128 failures=0` / `ALL OK` and exit 0 - see `../output.txt` for the
actual captured run, including an `objdump` check confirming the real hardware `extrq $0x0,$0x0` /
`insertq $0x0,$0x0` instructions are what actually execute. See `../RESULT.md` for why this
candidate's hypothesis did not reproduce.
