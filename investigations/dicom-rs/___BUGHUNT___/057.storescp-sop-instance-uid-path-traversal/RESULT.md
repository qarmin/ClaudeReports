# storescp-sop-instance-uid-path-traversal

`storescp` (the DICOM Storage SCP / server CLI tool) writes each received C-STORE instance to disk
using its SOP Instance UID as the filename, taken directly from the incoming request's "Affected
SOP Instance UID" data element with **no sanitization** beyond stripping a trailing NUL pad byte:

```rust
// storescp/src/store_sync.rs (identically in store_async.rs)
let mut file_path = out_dir.to_path_buf();
file_path.push(sop_instance_uid.trim_end_matches('\0').to_string() + ".dcm");
```

This crate does not restrict a decoded UI-VR value to the DICOM UID character repertoire (digits
and `.` only) - it is read as a plain string. A malicious or simply non-compliant DICOM client can
therefore set the SOP Instance UID to a value containing `..` path segments (or an absolute path),
causing the server to write the received file **outside** the intended output directory, with no
error raised.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Git archaeology (same technique as candidate 056): `git log --all --grep="#768"` | Searched all refs for commit messages referencing the same external report already seen fixing sibling issues elsewhere in this session | Found commit `866a1942` ("fix(storescp): replace TransferSyntaxRegistry unwrap and sanitize SOP UID filename") on the same family of unmerged branches, covering two issues (`LOGIC_9`, a `.unwrap()` DoS panic - split off as its own resume note in `IDEAS.md`'s backlog, not investigated in this session; `LOGIC_10`, this candidate) |
| `git merge-base --is-ancestor 866a1942 df2b9e42` | N/A | Returned false - confirmed this fix was never merged, so the vulnerability is live in the exact commit (`df2b9e42`) this session audits |
| **[REBUILT per CONFIRMATION.md section 11]** Real, two-real-binary, real-loopback-TCP reproduction (`repro/run_e2e.sh`): builds and runs the actual `dicom-storescp` and `dicom-storescu` binaries; a small helper (`repro/make_evil_file`) constructs a genuine, valid DICOM file (via the public `dicom-object`/`dicom-core` API) whose Media Storage SOP Instance UID contains `../escaped_057`; `dicom-storescu` sends it to `dicom-storescp` over a real loopback TCP association | Real DICOM association negotiation + C-STORE exchange, both binaries built from a pristine clone of `df2b9e42` | **Genuine escape observed**: `storescp`'s own log reports `Stored /tmp/.../received/../escaped_057.dcm`; the `received/` output directory ends up empty, and the file is independently found sitting one directory level above it. Reproduced deterministically: verified first directly against the live project tree's own built binaries (a completely separate confirmation, not just the packaged `repro/`), then again via `repro/`'s `just test` against a fresh clone (2 independent real end-to-end runs total; see `backtrace.txt` for both) |
| (superseded, kept for context) Earlier session's hand-built, single-process reproduction of the vulnerable path-construction computation in isolation, without running either real binary or a real socket | `std` only, no DICOM dependencies | Demonstrated the same conclusion but did not run the real `storescp`/`storescu` binaries or a real network exchange - retired in favor of the real two-binary repro above once `CONFIRMATION.md` section 11 (added mid-session, after this candidate's original write-up) required it |

## Status

**Confirmed**, with a bounded fix applied and verified via a real end-to-end (two real binaries,
real loopback TCP) reproduction. Deterministic - reproduces on every run, for any C-STORE request
whose Affected SOP Instance UID contains characters outside the DICOM UID repertoire (in
particular `/`, `\`, or `..` sequences) or an absolute path.

**Fully reproducible: Yes** - `repro/` runs the actual `dicom-storescp` and `dicom-storescu`
binaries communicating over a real loopback TCP socket (see `CONFIRMATION.md` section 11).

## Class

`security` (path traversal / arbitrary file write, reachable by any DICOM client that can
establish an association with this server - `storescp` is a network-facing service intended to
accept associations from other DICOM peers, so this is attacker-reachable without any special
privileges beyond being able to open a TCP connection to the server and negotiate a C-STORE).

## Root cause

`storescp/src/store_sync.rs`'s (and identically `store_async.rs`'s) receiving loop extracts the
`AffectedSOPInstanceUID` element from the incoming C-STORE command, keeping it as a plain `String`
(`sop_instance_uid`), and later uses it, after only trimming a trailing NUL padding byte, as a
single path component appended to the configured output directory via `PathBuf::push`. Nothing in
this crate's DICOM decoding enforces that a UI-VR value's characters are restricted to the DICOM
UID repertoire (digits and `.`) - a decoded string can contain anything the wire bytes encode,
including `/`, `\`, and `..`. `PathBuf::push` resolves such components exactly as the OS would (an
absolute-looking component replaces the base path entirely; `..` components are preserved
literally and resolved by the OS on the actual file open/write) - so a crafted UID directly
controls where the received file is written, with no bound to `out_dir`.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Filters the SOP Instance UID down to a safe character set (alphanumeric, `.`, and `-`) before
using it as a filename component, in both `store_sync.rs` and `store_async.rs`:

```rust
let safe_uid: String = sop_instance_uid
    .trim_end_matches('\0')
    .chars()
    .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '-')
    .collect();
let mut file_path = out_dir.to_path_buf();
file_path.push(safe_uid + ".dcm");
```

This removes every character that could be used to escape `out_dir` (path separators, `..`
sequences, drive-letter/absolute-path prefixes) while preserving the entire legitimate DICOM UID
character repertoire (digits and `.`) plus a couple of additional characters (`-`, and letters)
that don't affect path safety but avoid unnecessarily mangling filenames for common
non-strictly-conformant UIDs seen in the wild.

See `patch.diff` for the exact diff.

- `cargo build -p dicom-storescp --all-features`: builds cleanly with the fix applied (both
  `store_sync.rs` and `store_async.rs`, the latter gated behind an "async"-style feature enabled
  by `--all-features`).
- `cargo test -p dicom-storescp --all-features`: **1 passed, 0 failed** (the crate's only existing
  test, a CLI-argument-parsing smoke test - unaffected by this fix, since there is no existing
  dedicated unit test for the network receiving loop this fix lives in; see "What's not done").
- Full workspace (`cargo test --workspace --lib`; `dicom-storescp` is a `--bin`-only crate with no
  `--lib` target, so it is not itself included in this suite): **351 passed / 0 failed / 2
  ignored** before and after - unchanged, as expected, confirming this fix has zero effect outside
  `storescp`.

## What's not done / open questions

- No dedicated unit test exists in `storescp`'s own test suite for the receiving loop this fix
  lives in (the crate's only test is a CLI-parsing smoke test) - adding one would require
  refactoring the loop to be independently testable, which is out of scope for a bounded bug-hunt
  fix; this candidate's `repro/` covers the verification instead.
- The companion `.unwrap()` panic on an unrecognized transfer syntax (`LOGIC_9`, from the same
  unmerged external commit) was deliberately **not** fixed as part of this candidate - it is a
  distinct bug (different mechanism: a DoS-class panic, not a path/security issue) and was left as
  a resume note in `IDEAS.md`'s backlog for a future candidate in this session or a later one.
- The character-filtering approach (allow-list: alphanumeric, `.`, `-`) matches the already-
  diagnosed external fix exactly; a stricter DICOM-UID-only filter (digits and `.` exclusively)
  was considered but not adopted, to avoid being any more restrictive than the external fix this
  candidate adopted and extended validation for elsewhere in this session's history.
