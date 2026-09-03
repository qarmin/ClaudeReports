# Correction

This candidate's original "Not reproducible" verdict **still holds for the specific line it
examined** (the `let CollectionSource::Parser(parser) = self else { unreachable!() }` re-match
immediately after `*self = CollectionSource::Parser(...)` in
`CollectionSource::set_parser_with_ts` - genuinely unreachable, by Rust's `&mut` exclusivity
guarantee, exactly as originally reasoned).

However, a later session found a **separate, real, confirmed bug earlier in that same function**
that this candidate's investigation did not consider: `src.take().unwrap()` runs *before* the
transfer-syntax lookup that can fail, so a failed lookup leaves `self` in a `Raw { reader: None,
.. }` state that `has_parser()` can't distinguish from a normal not-yet-parsed state - a second
call to any of `DicomCollector`'s read methods (e.g. simply retrying after catching the first
error) then panics on that same `unwrap()`.

Written up in full as candidate
[`061.object-collector-ts-lookup-retry-panic`](../061.object-collector-ts-lookup-retry-panic/RESULT.md),
including its own confirmation, root cause, fix, and verification. This function
(`CollectionSource::set_parser_with_ts`) is not "safe" overall - only the one narrow
`unreachable!()` line this candidate's original write-up addressed is safe.
