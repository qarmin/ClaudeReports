# clippy_pedantic_nursery_sweep_no_bugs

Meta/sweep candidate: a full `cargo clippy` pass with `clippy::pedantic` and `clippy::nursery`
enabled (on top of the default lint set), `--no-default-features --features std,all-apis
--all-targets`, run once against the `linux_raw` backend and once against the `libc` backend
(`,use-libc` added), to see whether anything in the ~2000-2900 warnings/run points at a real
correctness bug rather than style. This candidate independently re-ran both sweeps from scratch in
this session (rather than trusting an unverified prior claim) and manually traced every distinct
lint category, plus every call site of the higher-signal categories (anything touching casts,
pointers, transmutes, or wildcard error matching), back to source.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| `cargo clippy --no-default-features --features std,all-apis --all-targets -- -W clippy::pedantic -W clippy::nursery` (`linux_raw` backend, default) | stable 1.98.1 / clippy 0.1.98, this container's existing toolchain, run once to completion (~a few minutes) | Exit 101. 2885 warnings on `lib test` (2789 duplicates against the plain `lib` build's 2600), plus smaller per-target counts for `tests/*` and `examples/*` (see `output.txt`). One real `error` (not a warning): `examples/buffer_errors.rs` fails `clippy::never_loop` (deny-by-default `correctness` lint, unrelated to the pedantic/nursery sweep itself) - this is candidate 022's already-`Confirmed` finding, re-encountered here as expected since it was never fixed in this tree (the fix is intentionally reverted per "the one rule that matters most"). All other targets (`lib`, `build script`, every `test`/`example` binary) compiled and were fully linted despite that one example failing. 72 distinct lint categories fired; full per-category counts captured in `output.txt`. |
| `cargo clippy --no-default-features --features std,all-apis,use-libc --all-targets -- -W clippy::pedantic -W clippy::nursery` (`libc` backend) | same toolchain, run once to completion | Exit 101, same `buffer_errors` `never_loop` error. 2525 warnings on `lib test` (2451 duplicates against `lib`'s own 2266). 68 distinct lint categories fired, same shape as the `linux_raw` run. Diffed the two runs' category sets: only `clippy::equatable_if_let` is unique to the `libc` run (one site, `src/backend/libc/mm/syscalls.rs:27`, a stylistic `if let` vs `==` suggestion on an enum comparison - not a bug), and 5 categories (`assertions_on_constants`, `if_not_else`, `match_wild_err_arm`, `needless_pass_by_ref_mut`, `verbose_bit_mask`) are unique to `linux_raw` - all traced below. |
| Manual source audit of every call site for the highest-signal categories across both runs: `cast_ptr_alignment` (2 sites), `transmute_ptr_to_ptr` (5 sites), `collection_is_never_read` (1), `match_wild_err_arm` (1), `unnecessary_literal_unwrap` (3), `as_ptr_cast_mut` (11), `zombie_processes` (4), plus a spot-check of the three highest-count categories (`redundant_pub_crate` 1082, `missing_errors_doc` 474, `cast_possible_truncation` 293) | Direct `Read`/`grep` against the exact `file:line` each warning cites, cross-referenced against `SUMMARY.md` for whether a prior candidate already covers that exact site | Every genuinely suspicious-looking site traces to an already-recorded, already-root-caused candidate in this same workspace: `fs/inotify.rs:195` and `fs/raw_dir.rs:220` (`cast_ptr_alignment`) are candidates 053/044's own guarded `.cast::<T>()` sites (kernel-guaranteed alignment, documented `SAFETY` comments); `param/auxv.rs:343` (`match_wild_err_arm`, `Err(_err) => panic!()`) is candidate 034's confirmed-but-intentional-shape panic; the four `zombie_processes` hits (`tests/process/wait.rs`) are candidate 061's `Not reproducible` (reaped by `rustix::process::waitpid`/`waitpgid`/`waitid` before `Child::drop`, confirmed via real `/proc/<pid>/stat` reads); `as_ptr_cast_mut` hits in `prctl.rs`/`thread/prctl.rs` are candidate 054's systemic int-to-ptr-cast sweep. The `transmute_ptr_to_ptr` sites in `src/io_uring/bindgen_types.rs` are bindgen-generated FFI struct-layout code (same-size repr(C) reinterpretation, standard bindgen output shape, not hand-written); `src/buffer.rs:415` and `src/path/dec_int.rs:236` are same-size-array-to-slice-element reinterpretations already covered by this crate's general buffer-handling candidates (001/005/010/050). `collection_is_never_read` (`tests/fs/dir.rs:83`) is a test-only `Vec` populated for its side effect (asserting iteration completes) rather than read back - not a production code path. No call site among any of these resolved to a new, not-yet-recorded finding. |
| Spot-check of the 3 highest-count pedantic categories (`redundant_pub_crate` 1082, `missing_errors_doc` 474, `cast_possible_truncation` 293) for any hidden correctness signal within the bulk | Sampled ~15 sites per category across both runs | All purely stylistic/documentation-shaped as expected (`pub(crate)` visibility could be tightened; `# Errors` doc sections missing; numeric casts that narrow a value already bounded by a prior check or by the syscall ABI itself, e.g. a `usize` byte count being cast to a kernel-defined `u32` field). No site in the sample lacked a prior bounds check or type-level guarantee. |

No further tools were needed beyond these - this is a deterministic, single-process static-analysis
result (clippy analyzing fixed source text against two fixed feature-flag configurations), the same
class of finding as candidate 022 (no runtime/input-dependent behavior to escalate to miri/ASAN/a
fuzz harness for). The three approaches above (full category-and-count comparison across both
backends, manual audit of every higher-signal category's call sites, and a sample audit of the
bulk-count categories) constitute the soft cap of distinct approaches for this candidate shape.

## Status

**Not reproducible** (no new bug - every distinct lint category across both sweeps was checked, and
every call site worth tracing by hand resolves to either a site already covered by an existing
`Confirmed`/`Not reproducible` candidate in this workspace, or genuinely intentional/guarded code
with no exploitable gap). Both sweeps ran to completion once each in this session (not re-run
multiple times, since the result is a deterministic static-analysis pass over unchanged source - the
same reasoning candidate 022 uses for why a repeat-run count doesn't apply to a pure static-lint
finding). Full category counts and the exact file:line locations traced are captured in
`output.txt`.

**Fully reproducible**: N/A - no bug is being claimed, so `CONFIRMATION.md` section 11 (driving a
real second participant) does not apply. The negative result itself (no new finding) is fully
reproducible on demand: re-running either `cargo clippy` invocation above against the same commit
(287214b889865d8e1406a0ee71cc409b6f6191c8) reproduces the same warning counts and the same
`never_loop` error deterministically.

## Class

N/A - not a bug.

## Root cause

N/A - not a bug. `clippy::pedantic`/`clippy::nursery` are deliberately broad, high-false-positive
lint groups (the project's own `Cargo.toml`/`clippy.toml` does not enable either by default, nor
does CI run clippy at all, per candidate 022's finding of no clippy step in
`.github/workflows/main.yml`) intended to surface *style* opportunities, not correctness defects.
The bulk of both runs' output (documentation gaps, visibility tightening, lifetime-elision
opportunities, uninlined format args, `#[must_use]` suggestions) has no runtime behavior attached
to audit at all. The categories that do touch runtime behavior (casts, pointer reinterpretation,
transmutes, wildcard error matching) were individually traced and, in every case, already have a
documented safety invariant, a prior bounds check, or an already-recorded candidate in this same
workspace explaining why the pattern is safe as written.

## Introduced by

N/A - not applicable, no bug confirmed.

## The fix

N/A - no bug found, nothing to fix. (The one real `error`-level finding incidentally re-encountered
while running this sweep, `examples/buffer_errors.rs`'s `never_loop`, is candidate 022's own
already-recorded, already-fixed-and-reverted finding - not new, not re-recorded here.)

- Project test suite: `cargo test --workspace --features "std,all-apis,use-libc"` - 151 passed / 1
  failed / 1 ignored (the 1 failure is `ioctl::test_int_setter`, a pre-existing, documented,
  environment-only failure unrelated to this candidate - this sandbox has no `/dev/net/tun`, see
  `README.md`'s toolchain-availability table). No fix was applied this session, so there is no
  before/after delta to report; this run is a baseline health check only, confirming the sweep
  itself didn't need any source change and the tree's test health matches what every prior
  candidate in this workspace has already observed.

## What's not done / open questions

- This re-confirms, rather than merely re-asserts, the original meta-finding this candidate number
  was reserved for: a clippy pedantic+nursery sweep is a cheap, repeatable sanity check worth
  re-running after a meaningfully large set of source changes lands (a new backend, a new target
  architecture, a large refactor) - but running it again against this exact commit
  (287214b889865d8e1406a0ee71cc409b6f6191c8) with no source changes in between is not expected to
  surface anything new, and didn't.
- Not every one of the ~2900 individual warning instances was traced by hand (that would mean
  re-deriving ~69 already-closed candidates' worth of analysis) - the categories most likely to hide
  a real bug (anything touching a cast, pointer, transmute, or wildcard match) were traced
  exhaustively; the categories that are definitionally documentation/style-only
  (`missing_errors_doc`, `must_use_candidate`, `uninlined_format_args`, `doc_markdown`,
  `redundant_pub_crate`, etc.) were sample-checked rather than exhaustively audited, since there is
  no runtime behavior in scope for those lints to mis-flag.
- `cargo clippy` is still not part of CI (`.github/workflows/main.yml` has no clippy step, per
  candidate 022) - a future source change could silently introduce a new instance of any of the
  already-ruled-safe patterns above without any automated signal; this remains a standing gap this
  candidate doesn't close (out of scope - a CI change, not a bug fix).
