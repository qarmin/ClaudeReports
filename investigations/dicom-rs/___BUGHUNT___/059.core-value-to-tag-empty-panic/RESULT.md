# core-value-to-tag-empty-panic

`dicom_core::value::Value::to_tag()` (`core/src/value/mod.rs`) indexes into the underlying
`Tags` vector unconditionally:

```rust
pub fn to_tag(&self) -> Result<Tag, CastValueError> {
    match self {
        Value::Primitive(PrimitiveValue::Tags(v)) => Ok(v[0]),
        _ => Err(CastValueError { requested: "tag", got: self.value_type() }),
    }
}
```

If the `Tags` container is empty, `v[0]` panics with an index-out-of-bounds instead of
returning the `CastValueError` this function is otherwise designed to produce for every other
mismatched-type case.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Git archaeology (same technique as candidates 056-058): `git branch -r` lists ~35 `origin/fix/*` branches, each a single self-contained already-diagnosed commit never merged into master; checked via `git merge-base --is-ancestor` | Found `origin/fix/to-tag-empty-panic` (commit `735964b0`) fixing this exact function | Confirmed unmerged: `git merge-base --is-ancestor 735964b0 df2b9e42` returns false |
| Temporary in-tree `#[test]` in `core/src/value/mod.rs` constructing `Value::Primitive(PrimitiveValue::Tags(smallvec![]))` and calling `.to_tag()` (reverted before write-up) | `cargo test -p dicom-core to_tag_empty_repro` | Panics: `index out of bounds: the len is 0 but the index is 0` at `core/src/value/mod.rs:736:62` |
| Standalone `repro/` (path-dependency on a pristine clone of `df2b9e42`, via `just test`) | `cargo run` | Reproduces the same panic deterministically; see `backtrace.txt` |
| Traced how an empty `Tags` value can arise from real DICOM parsing, not just manual construction | Read `parser/src/stateful/decode.rs::read_value_tag` | `ntags = len >> 2`, so a zero-length AT (Attribute Tag) element decodes to `PrimitiveValue::Tags(<empty>)` via completely ordinary parsing - no malformed/adversarial length needed, just length 0 |

## Status

**Confirmed** - deterministic, reproduces every run, no fixed seed needed. Reproduced both via a
temporary in-tree test against the live tree and independently via `repro/` against a pristine
clone of the exact commit this session audits (`df2b9e42`).

## Class

`crash` (panic / index-out-of-bounds on a value derived directly from element content that can
be attacker/file-controlled - a zero-length AT element - reached through this crate's own
public value-casting API).

## Root cause

`to_tag()` was written mirroring the type-mismatch pattern used by every other `to_*` accessor
on `Value`/`PrimitiveValue` (return `CastValueError` for the wrong variant), but forgot that the
*correct* variant (`Tags`) can itself still be legitimately empty - exactly as `PrimitiveValue`'s
own `tag()` getter (via the `impl_primitive_getters!` macro elsewhere in the crate) already
guards against via an `is_empty()` check. `to_tag()` was the one accessor that didn't inherit
that same guard.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Add an explicit empty-check arm before the indexing arm, returning the same
`CastValueError { requested: "tag", got: ValueType::Empty }` shape already used elsewhere in
this file for the "value present but empty" case (e.g. `to_multi_str`'s siblings):

```rust
pub fn to_tag(&self) -> Result<Tag, CastValueError> {
    match self {
        Value::Primitive(PrimitiveValue::Tags(v)) if v.is_empty() => Err(CastValueError {
            requested: "tag",
            got: ValueType::Empty,
        }),
        Value::Primitive(PrimitiveValue::Tags(v)) => Ok(v[0]),
        _ => Err(CastValueError {
            requested: "tag",
            got: self.value_type(),
        }),
    }
}
```

See `patch.diff` for the exact diff (scoped to `core/src/value/mod.rs` only).

- `cargo build -p dicom-core`: builds cleanly with the fix applied.
- `cargo test -p dicom-core --lib` (against the patched pristine clone): **67 passed / 0
  failed** - unchanged pass count from before the fix, confirming zero regressions.
- `cargo test --workspace --lib` (against the live tree, fix applied then reverted): all tests
  passed, no failures introduced.
- `repro/` (`just test`): panics before the patch, exits cleanly with a handled `Err` after.
