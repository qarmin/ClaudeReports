# Repro for candidate 048

`fix_negative_nsecs`/`adjust` (`src/timespec.rs`, `#[cfg(apple)]`-only) is `pub(crate)`, so it
can't be called from an external crate using rustix's public API. The repro instead drives the
real, unmodified function (not a reimplementation of its logic) directly inside the project's own
source tree, cross-interpreted by Miri for `x86_64-apple-darwin` so the real `#[cfg(apple)]` code
path actually compiles and runs without macOS hardware (same technique used by this workspace's
candidate 039).

`add_test.diff` adds one `#[test]` function (`test_negative_timestamp_secs_zero_boundary`) to
`src/timespec.rs`'s existing `#[cfg(test)] mod tests`, calling `fix_negative_nsecs(0,
-100_000_000)` - the exact `secs <= 0` boundary case the upstream writeup this workaround is based
on (linked from the function's own doc comment, rust-lang/rust#108277, comment by joshtriplett)
describes as real, reachable macOS kernel output: "if we get a Timespec value with seconds less
than or equal to zero, and nanoseconds between -999_999_999 and -1 (inclusive), we can add
1_000_000_000 to the nanoseconds and subtract 1 from the seconds". `secs == 0` is explicitly
included there; `adjust()`'s own `assert!(*secs < 0)` excludes it.

See `../justfile` for the runnable before/after cycle (`just test` from this candidate's own
folder, after filling in `repo_url`/`base_commit` - already done for this candidate).
