# object-filemeta-invalid-uid-characters-accepted

`object/src/meta.rs`'s `read_str_body` (used by `FileMetaTable::from_reader` to decode every
UID-valued field in the File Meta Information group - Media Storage SOP Class UID, Media
Storage SOP Instance UID, Transfer Syntax UID, and Implementation Class UID) decodes the raw
bytes as text and returns them verbatim, with no validation that the result is actually a
well-formed UID:

```rust
fn read_str_body<'s, S, T>(source: &'s mut S, text: &T, len: u32) -> Result<String>
where S: Read + 's, T: TextCodec {
    let mut v = Vec::new();
    v.try_reserve_exact(len as usize).context(AllocationSizeSnafu)?;
    v.resize(len as usize, 0);
    source.read_exact(&mut v).context(ReadValueDataSnafu)?;

    text.decode(&v).context(DecodeTextSnafu { name: text.name() })
}
```

A DICOM UID (per PS3.5) can only contain ASCII digits and `.` separators (plus trailing
whitespace/NUL padding). Any byte the declared text codec can still successfully *decode* -
including printable non-ASCII characters - is accepted into these fields with no further check,
so a file with a garbage/non-conformant byte anywhere in one of these four UID fields parses
without error, producing a `FileMetaTable` whose UID fields don't actually satisfy the invariant
every other part of this library (and any downstream consumer) assumes they do.

This is a real, unmerged fix from this project's actual maintainer (`Eduardo Pinho
<enet4mikeenet@gmail.com>`, commit `af8de9fd` on `upstream/bug/object/meta-check-uid`) - found via
the same git-archaeology technique used for the Claude-authored `origin/fix/*` branches this
session, extended to also check `upstream/bug/*` branches. The commit already includes its own
regression test (`reject_bad_uid`), reused verbatim here.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Git archaeology: checked `upstream/bug/*` branches (in addition to the ~35 `origin/fix/*` branches already covered by candidates 056-076) against `df2b9e42` via `git merge-base --is-ancestor` | Found `upstream/bug/object/meta-check-uid` (commit `af8de9fd`), from the project's real maintainer, not a Claude-authored triage branch | Confirmed unmerged |
| Reused the commit's own hand-crafted test bytes (a well-formed FMI group except for one invalid byte, `0xF6`, appended to an otherwise-valid Implementation Class UID) as a temporary in-tree `#[test]` (reverted before write-up) | `cargo test -p dicom-object reject_bad_uid_repro` | `FileMetaTable::from_reader` succeeds and returns a table whose `implementation_class_uid` field literally contains the invalid byte (decoded as `'ö'`), no error at all |
| Standalone `repro/` (path-dependency on a pristine clone of `df2b9e42`, via `just test`), using the same test bytes against the public `dicom_object::meta::FileMetaTable::from_reader` API | `cargo run` | Reproduces the same silent acceptance deterministically; see `backtrace.txt` |

## Status

**Confirmed** - deterministic, reproduces every run (a fixed, well-formed-except-for-one-byte
input).

## Class

`logic`/`security`-adjacent (missing input validation - not a crash - on a value (UID) that many
other parts of this library and its consumers treat as trustworthy/well-formed once parsed;
UID fields are frequently used to build filesystem paths (see candidate 057's storescp path
traversal finding, at the dataset level rather than FMI), used as dictionary/lookup keys, or
embedded in generated output, so accepting non-conformant characters here widens the class of
"unexpected" values that need to be handled defensively downstream).

## Root cause

`read_str_body`'s only concern was successfully decoding the declared character encoding into a
`String` - it was never extended to also confirm the decoded text is a valid UID, even though
all four of its call sites are exclusively for UID-valued fields.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

After decoding, verify every character is one a legal UID may contain (ASCII digit, `.`, NUL, or
ASCII whitespace - the last two accounting for even-length padding), rejecting with a new
`InvalidUid` error otherwise:

```rust
let out = text.decode(&v).context(DecodeTextSnafu { name: text.name() })?;

// do a quick UID verification for invalid characters
ensure!(
    out.chars().all(|c| c == '.' || c.is_ascii_digit() || c == '\0' || c.is_ascii_whitespace()),
    InvalidUidSnafu,
);

Ok(out)
```

See `patch.diff` for the exact diff (scoped to `object/src/meta.rs` only; adds the `InvalidUid`
error variant and includes the maintainer's own regression test, reused verbatim).

- `cargo build -p dicom-object`: builds cleanly with the fix applied.
- `cargo test -p dicom-object --lib` (against the patched pristine clone): **76 passed / 0
  failed** (75 pre-existing + the reused regression test), confirming zero regressions - all
  other FMI-parsing tests in this crate already use well-formed UIDs.
- `cargo test --workspace --lib` (against the live tree, fix applied then reverted): all tests
  passed, no failures introduced.
- `repro/` (`just test`): silently accepts the invalid byte before the patch; cleanly rejects it
  with `InvalidUid` after.
