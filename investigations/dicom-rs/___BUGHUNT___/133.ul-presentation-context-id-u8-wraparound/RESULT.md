# 133. ul: presentation context ID `u8` wraparound for more than 128 proposed contexts

`ClientAssociationOptions::create_a_associate_req` (`ul/src/association/client.rs:779` at
df2b9e42) assigns each proposed presentation context an identifier with
`id: (2 * i + 1) as u8`, where `i` is the index of the context in the caller-supplied
`presentation_contexts` vector. A clippy pedantic sweep flagged the truncating cast; PS3.8 defines
the presentation context identifier as a single odd byte, so only 128 identifiers exist, and this
crate's builder API (`with_presentation_context` / `with_abstract_syntax`) accepts an unbounded
number of contexts with no check anywhere on the way to the wire.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of the whole SCU proposal path | `with_presentation_context` -> `presentation_contexts: Vec<_>` -> `create_a_associate_req` -> `write_pdu`, plus `process_a_association_resp`'s id-based lookup, plus the SCP side in `server.rs` | No length guard exists anywhere. The only validation in `create_a_associate_req` is `ensure!(!presentation_contexts.is_empty(), MissingAbstractSyntaxSnafu)`. The SCP echoes the requestor's ids back verbatim, so a collision propagates into both peers' negotiated state. |
| Debug-assertions build | `cargo build`/`cargo test -p dicom-ul` | No panic and no overflow trap: `2 * i + 1` is computed in `usize` (257 for `i = 128`, no overflow) and only the `as u8` cast truncates, which is silent in every Rust build profile. This is why the bug never surfaces as a crash. |
| Real end-to-end SCU/SCP association over loopback TCP (`repro/`) | Release build, real `ClientAssociationOptions` + real `ServerAssociationOptions`, 128 / 129 / 200 proposed contexts, rustc 1.98.0 | Reproduced deterministically. See below. |

## Status

**Confirmed**

Reproduces 5/5 consecutive runs of `repro/`, release build, no randomness involved (the id
assignment is a pure function of the context index). Also reproduced by a full `just test` run from
a fresh upstream clone at df2b9e42.

Observed with 129 proposed contexts:

- the real SCP parses 129 contexts out of the real A-ASSOCIATE-RQ, but only **128 distinct**
  context IDs - index 0 and index 128 both arrive as ID `1`;
- the SCU's own `presentation_contexts()` then reports ID `1` twice, both times labelled with
  index 0's abstract syntax; index 128's abstract syntax is absent from the negotiated list
  entirely.

With 200 proposed contexts the same run shows 72 colliding IDs and 72 abstract syntaxes lost.

The 128-context control case (the largest legal number) negotiates correctly with 128 distinct
IDs, confirming the boundary is exactly where the arithmetic says it is.

**Fully reproducible**: **Yes**. `repro/` runs a real `dicom-ul` SCU and a real `dicom-ul` SCP as
two participants on a real loopback TCP socket, each using the crate's own public API and its own
genuine send/receive path. No PDU bytes are hand-authored: the duplicate IDs reported are what the
SCP actually decoded from the A-ASSOCIATE-RQ the SCU actually transmitted.

## Class

logic (protocol/wire-format correctness - silently malformed output)

## Root cause

`PresentationContextProposed::id` (`ul/src/pdu/mod.rs:188`) is a `u8`, matching PS3.8's one-byte
presentation context identifier. `create_a_associate_req` derives it positionally as
`(2 * i + 1) as u8`. For `i >= 128` the value `2 * i + 1` exceeds 255 and the `as u8` cast
truncates modulo 256 without any diagnostic (`as` casts never panic, in any build profile), so
context index `128 + k` is emitted with the same identifier as context index `k`.

The identifier is the *only* thing the PDU format gives either peer to match an accept/reject
result back to the context it belongs to. Both sides are corrupted as a result:

- the SCP builds its own negotiated list straight from the received ids, so it holds two entries
  with the same id but different abstract syntaxes;
- the SCU's `process_a_association_resp` resolves each accepted context with
  `presentation_contexts_proposed.iter().find(|pc| pc.id == c.id)`, which returns the *first*
  match - so every context past index 127 is silently relabelled with the abstract syntax of the
  context 128 positions earlier.

Any subsequent P-DATA exchange on such an association can therefore carry data for one SOP class
under a context the peer resolves to a different SOP class. The same `create_a_associate_req` is
used by all six establishment entry points (`establish`, `establish_with`, `establish_tls`,
`establish_with_tls`, and their `_async` counterparts), so every SCU path is affected.

## Introduced by

bf320315 "Association API" (2021-01-10) - the original implementation already wrote
`id: (i + 1) as u8` with no bound on the number of contexts, wrapping at `i >= 255`.
81abad6b "MAIN: Use odd presentation context IDs" (2024-08-05) changed it to `(2*i + 1) as u8`,
which is protocol-correct for the odd-identifier rule but halves the point at which the cast
starts truncating, from 256 contexts to 129. The missing validation - the actual defect - dates
from bf320315.

## The fix

`patch.diff` (+23 lines, no deletions, against df2b9e42):

- adds `pub const MAX_PRESENTATION_CONTEXTS: usize = 128;` to `ul/src/association/client.rs`,
  documenting the PS3.8 odd-8-bit-identifier reason for the limit;
- adds a `TooManyPresentationContexts { count, max }` variant to `association::Error` (the enum is
  `#[non_exhaustive]`, so this is not a breaking change);
- adds an `ensure!` in `create_a_associate_req`, right after the existing
  `MissingAbstractSyntaxSnafu` check, so an over-limit proposal fails with a clear error before
  any PDU is written.

Validation-with-a-clear-error was chosen over checked/saturating arithmetic because there is no
correct id to saturate to: with more than 128 contexts the proposal is unrepresentable in the
protocol, so silently dropping or renumbering contexts would trade a wire-format bug for a
silently-truncated request. It also matches the existing convention in the same function
(`MissingAbstractSyntax` for the analogous "caller supplied a nonsensical context list" case), and
covers all six establishment entry points at once since they all go through this one function.

Per `CONFIRMATION.md` section 7, the fix was checked against more than the one triggering input:
129 (the first over-limit value) and 200 (well past it) are both refused, while the 128-context
control - the largest legal proposal, one below the boundary - still negotiates all 128 contexts
correctly. `repro/` exits 1 before the patch and 0 after it.

- Project test suite (`cargo test -p dicom-ul --all-features`): 83 passed / 0 failed before ->
  83 passed / 0 failed after (1 ignored, unchanged).
- Project test suite (`cargo test --workspace`): 537 passed / 0 failed before -> 537 passed /
  0 failed after.
- `cargo clippy -p dicom-ul --all-features`: no new warnings. `cargo fmt -p dicom-ul` produces no
  further changes on the patched tree.

## What's not done / open questions

- No regression test was added to the crate's own test suite. A cheap one would be an assertion
  that `ClientAssociationOptions::new()` with 129 `with_abstract_syntax` calls fails to establish
  with `TooManyPresentationContexts` - but `create_a_associate_req` is private, so such a test
  either lives inside the crate or needs a listening socket; the maintainers may prefer one shape
  over the other.
- The SCP side (`ServerAssociationOptions`) was checked and does **not** generate identifiers - it
  echoes back whatever the requestor sent - so it has no equivalent of this bug. It also does not
  *reject* a request carrying duplicate identifiers; hardening the acceptor against a
  non-conforming peer that sends colliding IDs is a separate question this candidate does not
  cover.
- `process_a_association_resp`'s `find(|pc| pc.id == c.id)` silently takes the first match. With
  the fix above, the requestor can no longer create the ambiguity itself, but a non-conforming
  peer could still return a duplicated id in the A-ASSOCIATE-AC; that path was not investigated
  here.
- The candidate justfile's `repo_url` points at https://github.com/Enet4/dicom-rs.git rather than
  this checkout's own `origin` (git@github.com:qarmin/dicom-rs.git): that fork does not carry
  df2b9e42, and the upstream does, so pointing at the fork would leave a justfile that cannot
  clone the base commit. `just test` was run end-to-end from a fresh clone and passes.
