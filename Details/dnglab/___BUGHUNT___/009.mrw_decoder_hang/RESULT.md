# mrw_decoder_hang

Fuzzer-found artifact `rawler/fuzz/artifacts/fuzz_decode_metadata/crash-bc64cdd6e3d69fd6fa76d067616b6ecba4707c64`
(10 bytes: `00 4d 52 4d 3b 3b 3b c0 0a bf`, MRW magic `\0MRM`) was reported to hang (not return within
8s) when run through the `fuzz_decode_metadata` cargo-fuzz harness. Assigned to check whether
`rawler`'s Minolta MRW decoder (`rawler/src/decoders/mrw.rs`, `MrwDecoder::new_mrw`) has a genuine
infinite loop reachable from this input.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct call to the real public API (`rawler::decode_dummy()`, the exact function the fuzz harness itself calls) on the exact 10-byte artifact, via a small `rawler/examples/mrw_repro.rs` | debug build (`cargo build -p rawler --example mrw_repro`), 3 repeat runs, `timeout 8` each | No hang. Panics instantly (`bits.rs:149`, OOB slice index) and returns cleanly - the panic is caught by `RawLoader::decode()`'s own `panic::catch_unwind` wrapper (`rawler/src/decoders/mod.rs:1093-1098`). 3/3 runs, exit 0, well under 1s each |
| Same call | release build (`cargo build -p rawler --release --example mrw_repro`), no ASAN, no libFuzzer instrumentation, 3 repeat runs, `timeout 8` each | Same: no hang, panics and returns cleanly, 3/3 runs, exit 0 |
| The real `fuzz_decode_metadata` cargo-fuzz binary (libFuzzer + ASAN, `cargo +nightly fuzz build fuzz_decode_metadata`) run directly against the same crash artifact | release+ASAN+sancov, `timeout 8`, 3 repeat runs | Reproduces the reported hang every time: `exit 124` (`timeout` killed it) 3/3 runs. Panic message (`bits.rs:149`) is printed, then `libFuzzer: ERROR: deadly signal` / `run interrupted; exiting` is printed, then the process never actually exits (still spinning at ~99% CPU in a child process observed via `ps aux`, confirmed non-hanging-on-I/O) |
| Control: the *same* ASAN/libFuzzer binary run against a completely unrelated crash artifact already sitting in the same fuzz corpus (`crash-c97ea70a0507febbf5d6663f51a737bd5dd5588a`, 5 bytes, panics in `x3f.rs:43`, nothing to do with MRW) | same binary, `timeout 60` | Same hang: panic printed, `deadly signal`/`run interrupted` printed, then never exits (`exit 124` even at 60s) |
| Control: the same ASAN/libFuzzer binary run against a benign, non-crashing 10-byte input (`\x00`×10) | same binary, `timeout 8` | Completes normally in 200-624ms, exit 0, prints "Executed ... in N ms" |
| `gdb` direct-launch (`gdb --args ./fuzz_decode_metadata <artifact>`, `run`, `SIGINT` after 3s) | same ASAN/libFuzzer binary, one run | Panic happens, process receives `SIGABRT` immediately and gdb catches it right away (no hang observed under gdb's direct launch - gdb attaching to the already-forked libFuzzer-monitor process via `ptrace` separately was not possible in this sandbox, `ptrace_scope=1` blocks it) |

All commands and their real captured output are in `output.txt` (sections 1-5, matching the table
rows above in order).

## Status

**Not reproducible** (as the claimed bug: an infinite loop/hang inside `rawler`'s MRW decoder logic
reachable through the library's real public API).

What *does* reproduce, deterministically (3/3 and more): the 10-byte artifact makes
`MrwDecoder::new_mrw` panic on an out-of-bounds slice index (`rawler/src/bits.rs:149`, called from
`rawler/src/decoders/mrw.rs:161`) - a real but different bug class (a crash/panic, not a hang), and
one that is already caught safely by the library's own `panic::catch_unwind` wrapper around
`RawLoader::decode()`/`decode_dummy()`. Called through the real public API in either a plain debug
or plain release build (no ASAN, no libFuzzer), this always returns an `Err` in well under a
second - it never hangs.

The observed "hang within `timeout 8`" is real and 3/3 repeatable, but only inside the ASAN +
libFuzzer-instrumented `fuzz_decode_metadata` binary specifically, and it is not specific to this
MRW input or to any MRW code path: an entirely unrelated crash artifact that panics in `x3f.rs`
(nothing to do with MRW) hangs the exact same way in the exact same binary, while a non-crashing
input in that same binary returns in well under a second. So the discriminator for "hangs or not"
in this harness is "did the target panic/abort at all", not "which decoder" or "which input". This
points at the ASAN/libFuzzer crash-reporting path itself (something after `deadly signal` is
detected - most likely a symbolizer/backtrace step - stalling in this sandboxed environment, where
`ptrace_scope=1` also blocked a normal `gdb -p <pid>` attach to inspect it further), not at a bug in
`rawler`'s decoding logic.

**Fully reproducible**: N/A - no `Confirmed` bug is being reported here (see "A repro must exercise
the real end-to-end trigger path" - the claimed hang does not reproduce via the real end-to-end
public-API entry point at all, only inside the fuzzer's own instrumented binary, and identically so
for unrelated inputs, which is the reason this stays Not reproducible rather than Confirmed).

## Class

N/A (Not reproducible).

## Root cause

Not a hang. The actual mechanism behind the fuzzer's original report is:

1. `MrwDecoder::new_mrw` (`rawler/src/decoders/mrw.rs:147-204`) computes `data_offset` straight from
   an attacker-controlled 4-byte field (`BEu32(buf, 4) + 8`, no upper bound against the actual file
   length) and then loops `while currpos + 20 < data_offset { ... currpos += len + 8 ... }`, reading
   a tag/len pair via `BEu32(buf, currpos)` on every iteration with no check that `currpos + 4` (or
   `+20`) is within `buf`'s actual length.
2. For this 10-byte input, `data_offset` computes to ~993,640,392 (from bytes 4-7 = `3b 3b 3b c0`),
   so the loop's guard (`currpos + 20 < data_offset`) is trivially true on the very first iteration
   (`currpos == 8`), and `BEu32(buf, 8)` immediately indexes `buf[8..12]` on a 10-byte buffer via
   `byteorder::BigEndian::read_u32(&buf[pos..pos+4])` (`rawler/src/bits.rs:148-150`), which panics
   Rust's own slice-bounds check ("range end index 12 out of range for slice of length 10").
3. That panic propagates up through `MrwDecoder::new` -> `get_decoder` -> `decode_unsafe`, which
   `RawLoader::decode()` (`rawler/src/decoders/mod.rs:1093-1098`) already wraps in
   `panic::catch_unwind`, converting it to `Err(RawlerError::DecoderFailed(...))`. This is the same
   defensive wrapper every other decoder benefits from - it is why the real public API never hangs
   on this input.
4. `libfuzzer-sys`'s `fuzz_target!` macro installs its own panic hook that calls `abort()`
   unconditionally on any panic (specifically so a library's internal `catch_unwind` can't hide a
   bug from the fuzzer), which raises `SIGABRT` before `catch_unwind` ever gets a chance to run.
   libFuzzer's signal handler in the ASAN-instrumented binary detects this ("deadly signal"), prints
   its own diagnostic, and then - in this specific sandboxed environment - never finishes exiting.
   That last step reproduces identically for a second, structurally unrelated crash (a different
   decoder, different panic site), and does not reproduce at all for a non-panicking input in the
   same binary, which rules out anything MRW-specific or `rawler`-specific as the cause.

So the suspect the candidate named (an infinite loop inside the MRW decoder) is not what actually
happens; what actually happens is an already-mitigated OOB-slice panic, observed exclusively
through a fuzzer-tooling side effect of *how any panic* is handled in this sandbox's ASAN/libFuzzer
build, not through anything the MRW decoding logic itself does.

## Introduced by

N/A - no bug is being confirmed here. (The unrelated, already-mitigated OOB-slice-panic pattern in
`new_mrw`'s header loop was not bisected, since it is not the subject of this candidate - see "New
suspects" below.)

## The fix

None applicable - no bounded fix is being proposed for this candidate, since the claimed hang does
not reproduce through the real product code path.

- Project test suite: not run before/after, since no source change was made this candidate (N/A -
  no fix to verify).

## What's not done / open questions

- The *actual* mechanism behind the fuzzer's original crash-file classification (an OOB-slice panic
  in `MrwDecoder::new_mrw`'s attacker-controlled header-tag loop, `rawler/src/decoders/mrw.rs:147-186`
  via `rawler/src/bits.rs:149`) was not independently confirmed/root-caused/fixed as its own
  candidate - it is the same well-established "unchecked-offset-slice" bug class as this project's
  candidates 003 (`x3f.rs`), 005 (`ciff/mod.rs`), and 006 (`bmff/vendor.rs`), just in
  `mrw.rs`, and it is already safely caught by `RawLoader::decode()`'s `panic::catch_unwind` (so it
  is a "malformed input causes an internal panic" finding, not a memory-safety or hang finding).
  Flagged in `IDEAS.md` for a future candidate rather than folded into this one, since this
  candidate's Class (a hang/infinite loop) is not what that would be.
  - Sibling entries in `WBG`/`TTW` tag handling (`rawler/src/decoders/mrw.rs:171-182`) share the
    exact same unchecked-`currpos`-arithmetic pattern and were not individually exercised.
- Why the ASAN+libFuzzer binary's post-"deadly signal" step never returns in this sandbox was not
  root-caused further (no working `gdb -p <pid>` attach was available - `ptrace_scope=1` blocked it,
  and `gdb --args ... run` launching the binary directly did not reproduce the hang, since gdb's own
  SIGABRT trap fires before libFuzzer's handler gets a chance to run). This looks like an
  ASAN/libFuzzer/sandbox interaction (possibly a stuck symbolizer subprocess, given the huge ASAN
  shadow-memory VSZ and the CPU-spinning child process observed via `ps aux`), not a `rawler` bug,
  so it was not pursued further as part of this bug-hunt (out of scope: it's about the fuzzing
  tooling, not the code under test).
