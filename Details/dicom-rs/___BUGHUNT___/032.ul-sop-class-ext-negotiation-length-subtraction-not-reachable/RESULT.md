# ul-sop-class-ext-negotiation-length-subtraction-not-reachable

`ul/src/pdu/reader.rs:856`, inside `read_pdu_variable`'s handling of the User Information
sub-item type `0x56` (SOP Class Extended Negotiation), computes
`let data_length = (item_length - 2 - sop_class_uid_length) as usize;` where both `item_length`
and `sop_class_uid_length` are `u16` values read directly off the wire (attacker-controlled, from
an Association Request/Accept PDU). This matches HEURISTICS.md's "arithmetic on sizes/lengths
derived from input" pattern closely - two independently-read wire integers subtracted from each
other, the same shape as the confirmed `voilut-transform-min-pixel-value-subtract-overflow` bug
from the 2026-08-24 session (candidate 021). Found via manual code reading of `ul/src/pdu/
reader.rs`, a network-facing parser not covered by either bug-hunt session so far (the prior
session's 26 candidates were entirely file/pixel-data-focused).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Short fuzz burst against the project's own existing harness | `ASAN_OPTIONS=symbolize=0 cargo +nightly fuzz run pdu_roundtrip -- -max_total_time=150` (150s wall-clock, 8,811,050 executions, corpus 360 entries/40Kb) | No crash. Does not rule this out by itself - `pdu_roundtrip` round-trips `Pdu` values rather than directly fuzzing arbitrary wire bytes into `read_pdu`, and this specific 6-byte-short gap (see below) is a narrow target unlikely to be hit by mutation-based fuzzing without a dictionary entry for it |
| Manual boundary-value construction (per HEURISTICS.md "arithmetic on sizes... construct 0, 1, max, max+1 boundary values") on `item_length`/`sop_class_uid_length`, traced against the exact byte budget the wire format allows | Static reasoning, no build needed | The `u16` addition `2 + sop_class_uid_length` (checked by the `ensure!` immediately preceding this line, guarding this exact subtraction) only overflows when `sop_class_uid_length >= 65534`. Reaching that requires the sub-item's SOP-class-uid data field to actually contain that many real bytes (the immediately preceding `if bytes.remaining() < sop_class_uid_length { return Ok(None); }` check enforces this - the bytes must genuinely be present, not just declared). Including the sub-item's own fixed 4-byte header (`item_type` + reserved + `item_length`) and the 2-byte `sop_class_uid_length` field itself, the minimum wire bytes needed for this sub-item to reach the overflow-triggering state is `4 + 2 + 65534 = 65540` bytes - but the *outer* User Information item (type `0x50`) that contains this sub-item is itself bounded to a `u16` `item_length` (max `65535`, enforced structurally: `read_pdu_variable`'s top-level `let mut bytes = buf.copy_to_bytes(item_length as usize);` hands the `0x50` handler a slice of at most 65535 bytes, and every sub-item's cursor advances within that same shared slice with no re-extension). `65540 > 65535` - the minimum input needed to trigger the overflow is structurally 5 bytes larger than the wire format's own maximum size for the container it would need to live inside. No input satisfying both constraints simultaneously exists |

## Status

**Not reproducible** - ruled out by exact arithmetic on the wire format's own size limits, not
just "seems unlikely." The overflow condition and the format's structural size cap are mutually
exclusive by a fixed margin of 5 bytes, for any valid encoding of this sub-item type.

## Class

n/a

## Root cause

Not applicable (no bug) - a coincidental but real protective effect: the `u16` `item_length` field
on the *outer* User Information item (not on the `0x56` sub-item causing the arithmetic itself)
caps the sub-item area at one byte less than `u16::MAX`, which combined with the 6 bytes of
mandatory per-sub-item overhead is just barely enough to make the specific overflow-triggering
value of `sop_class_uid_length` (`>= 65534`) unconstructible. This is a fragile protection, not a
designed one - nothing in the code comments or logic suggests this was an intentional mitigation,
and the `ensure!`/subtraction would still be worth widening to a wider integer type (`u32`/`usize`)
defensively, since the "no overflow is reachable" property depends on a coincidence between two
unrelated size fields (the outer item's `u16` cap and the sub-item's own minimum overhead) rather
than on any explicit validation tying them together. A future refactor that, for example, changed
`item_length` to a wider type for one of the two nested levels but not the other could silently
remove this protection.

## The fix

N/A - no bug found, no fix needed. **Left for a future session as a hardening suggestion, not
applied here** (per SKILL.md, this workspace only carries `patch.diff` for verified bugs, not
speculative defensive changes to code with no reachable failure): widening `item_length` and
`sop_class_uid_length`'s use in the line 856 computation to a wider integer type (e.g.
`u32::from(item_length).saturating_sub(2).saturating_sub(u32::from(sop_class_uid_length))`) would
remove the dependency on this coincidence without changing any currently-observable behavior.

## What's not done / open questions

- This was the only two-variable-subtraction site found in `ul/src/pdu/reader.rs` by manual
  reading (grep for ` - ` narrowed to arithmetic, not comments, found exactly one: line 856). The
  file's other arithmetic (`item_length - 2` at line 327/334) is separately guarded by its own
  `ensure!(item_length >= 2, ...)` immediately before it and was not re-derived here.
- `ul/src/pdu/writer.rs` (the encode side, which could have the same class of subtraction bug on
  values supplied via the public API rather than the wire) was not checked this round.
- The rest of `ul/src/association/*.rs` (higher-level association state machine code, built on top
  of this reader) was not audited this round - only the raw PDU byte-level parser.
- The existing `pdu_roundtrip` fuzz harness's actual coverage of `read_pdu_variable`'s `0x56` arm
  specifically was not verified (e.g. via `cargo fuzz coverage`) - the 150s/8.8M-exec burst found
  nothing, but per CONFIRMATION.md section 10, that states what was covered, not a proof this
  specific arm was exercised at all.
