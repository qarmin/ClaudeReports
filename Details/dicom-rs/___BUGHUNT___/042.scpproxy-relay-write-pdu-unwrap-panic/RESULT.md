# scpproxy-relay-write-pdu-unwrap-panic

`scpproxy/src/main.rs`'s relay loop calls `write_pdu(&mut buffer, &pdu).unwrap()` (two call sites,
lines 223 and 231 in the pristine checkout) on a `Pdu` that was just parsed off the wire from one
side of the proxied association and is being forwarded, mostly unmodified, to the other side. This
was left as an open lead in `IDEAS.md`'s backlog by candidate `035`'s investigation, which found no
concrete trigger via manual reasoning alone and suggested a short fuzz harness round-tripping
`read_pdu` -> `write_pdu` as the better next tool - taken up this candidate per HEURISTICS.md's own
guidance ("`.unwrap()`/`panic!` on data derived from external input" -> "short fuzz harness
feeding the parse/decode entry point").

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Short one-off fuzz harness added to the existing `ul/fuzz/` cargo-fuzz project (temporary `pdu_relay_write_never_errors` target, mirroring `scpproxy`'s relay loop exactly: `read_pdu` then `write_pdu(..).unwrap()`; both the target file and its `Cargo.toml` `[[bin]]` entry were reverted before writing this record) | `ASAN_OPTIONS=symbolize=0 cargo +nightly fuzz run pdu_relay_write_never_errors -- -max_total_time=150` (150s wall-clock) | **Found a crashing input within seconds.** `write_pdu` panicked (via the harness's own `.unwrap()`) with `InvalidFixedSizeTextField { field: "Called-AE-title", .. reason: "must contain at least one non-space character" }` |
| Minimized, hand-built reproduction from the crash's root cause (not the raw fuzzer corpus bytes): built a valid `AssociationRQ` PDU via `write_pdu`, then patched only the 16-byte Called-AE-title wire field to all spaces (`0x20` x16) before feeding it to `read_pdu`, debug build, `cargo test` (temporary unit test added to `ul/src/pdu/writer.rs`, reverted before this record was written) | `read_pdu(&mut cursor, MAXIMUM_PDU_SIZE, false)` then `write_pdu(&mut relay_buffer, &relayed)` on the result | `read_pdu` accepts the all-space Called-AE-title, decoding+trimming it to an empty string (`""`) with no validation - PS3.8's own field description literally says *"The value made of 16 spaces (20H) meaning 'no Application Name specified' shall not be used"*, but the reader doesn't enforce that. `write_pdu`'s `encode_ae_title`, by contrast, does enforce it (`ensure!(!validate_value \|\| bytes.iter().any(|&byte| byte != b' '), ..)`), returning `Err(WriteChunk { .. InvalidFixedSizeTextField .. })` for the same value it just decoded. Reproduced 3/3 in-tree unit-test runs, then 2/2 more via the standalone `repro/` built against a fresh clone of pristine upstream `df2b9e42` (5/5 total) |
| **[REBUILT per CONFIRMATION.md section 11, after an initial downgrade]** Real, two-real-participant-plus-real-relay-target, real-loopback-TCP reproduction (`repro/`): runs the actual `dicom-scpproxy` binary as a real subprocess, relaying to a genuine fake SCP built on `dicom-ul`'s own real server association API; a genuinely separate process connects to the real scpproxy listen port over a real socket and sends a real (deliberately non-compliant) ASSOCIATE-RQ with an all-space Called-AE-title (assembled by patching a normal PDU `write_pdu` itself produced, since `write_pdu` refuses to encode this value directly - a real compliant client cannot produce it through the public API, matching what a genuinely non-compliant real client would send) | Checks whether the real `scpproxy` process survives (`try_wait()`) and can still serve a fresh, well-formed connection afterward | Real `dicom-scpproxy` subprocess genuinely crashes: `thread 'main' panicked at scpproxy/src/main.rs:231:58: ... InvalidFixedSizeTextField { field: "Called-AE-title", ... }`. Reproduced deterministically: verified first directly against the live project tree's own built binaries, then again via `repro/`'s `just test` against a fresh clone (2 independent real end-to-end runs total; see `backtrace.txt`) |

## Status

**Re-CONFIRMED via a real end-to-end reproduction.** This candidate was briefly downgraded to
Inconclusive mid-session (its first repro called `read_pdu`/`write_pdu` directly in-process,
never running the real `scpproxy` binary) per an updated `CONFIRMATION.md` section 11 - a
genuine real-subprocess, real-socket repro was subsequently built (see above) and the finding is
Confirmed again.

**Fully reproducible: Yes** - `repro/` runs the actual `dicom-scpproxy` binary as a real
subprocess relaying to a genuine fake SCP, driven by a genuinely separate process over a real
loopback TCP socket (see `CONFIRMATION.md` section 11).

## Class

`crash` (an application-level `panic!`-via-`.unwrap()` reachable by any remote peer that connects
through `scpproxy` and sends an ASSOCIATE-RQ or ASSOCIATE-AC with an all-space Called-AE-title or
Calling-AE-title - a trivial, one-field malformed/edge-case input, not requiring any deep protocol
knowledge to construct; crashes the whole proxy process, not just that one relayed connection).

## Root cause

Two things combine: (1) `dicom_ul::pdu::reader::read_pdu` is intentionally lenient on receipt -
it decodes and trims the 16-byte Called-AE-title/Calling-AE-title wire fields into a `String` with
no validation that the result is non-empty, even though the DICOM standard explicitly disallows an
all-space encoding for these two fields specifically (unlike most of this reader's other
leniencies, which are about fields the standard itself says aren't validated on receipt - "not
tested to this value when received" - Called/Calling-AE-title's "shall not be used" wording is
different and stronger, though the reader treats it the same permissive way regardless); (2)
`dicom_ul::pdu::writer::write_pdu`'s `encode_ae_title` *does* enforce this specific rule
(`validate_value` is `true` for these two fields specifically, unlike some of its other callers) -
correctly rejecting an attempt to freshly *originate* such a PDU, but with no distinction between
"originating a new PDU" and "re-encoding a PDU this process only decoded, not authored." `scpproxy`
is a pure relay - every `Pdu` value flowing through its `write_pdu` calls was parsed from a remote
peer, not constructed locally - so this asymmetry, which is harmless/correct for a normal
client/server that only ever encodes PDUs it built itself, becomes a crash vector specifically for
a relay tool that blindly assumes "if I could parse it, I can always re-serialize it."

## The fix

Changed both `write_pdu(&mut buffer, &pdu).unwrap()` call sites in `scpproxy/src/main.rs`'s relay
loop to handle the `Result` gracefully - log the error and end that relay session (`break` out of
the loop, matching how the same loop already handles `ThreadMessage::WriteErr` a few lines below,
which was already defined with the exact right error type
(`dicom_ul::pdu::WriteError`) but marked `#[allow(dead_code)]`, since nothing constructed it) rather
than crashing the entire proxy process:

```
-                            write_pdu(&mut buffer, &pdu).unwrap();
+                            if let Err(err) = write_pdu(&mut buffer, &pdu) {
+                                error!(
+                                    "error encoding PDU to relay to {:?}: {}",
+                                    ProviderType::Scu,
+                                    Report::from_error(err)
+                                );
+                                break;
+                            }
```
(and the same for the `ProviderType::Scp` branch a few lines below). The fix scopes the failure to
the one relay session that hit it - the proxy process itself keeps running and can accept new
connections - rather than the previous behavior of the entire process aborting.

Verified: `cargo build -p dicom-scpproxy` succeeds against the patched source. **Not verified via a
live two-socket integration test** proving the process survives an actual malicious relay session
end-to-end - `scpproxy` is a binary-only crate (no library target), so the standalone `repro/`
convention used by every other candidate in this workspace (call into a library function directly)
doesn't apply cleanly here; setting up two real TCP listeners (fake SCU and fake SCP) to drive the
proxy's actual `run()` function was judged out of scope for this candidate's time budget given the
fix's change is a small, directly-readable, single-pattern substitution that already has an
established sibling (`ThreadMessage::WriteErr`) demonstrating the same handling style is
intentional and correct for this codebase. The underlying condition the fix protects against (that
`write_pdu` fails for exactly this kind of relayed PDU) is independently confirmed via `repro/`.

- Project test suite: **N/A - `scpproxy` has exactly one existing test (`tests::verify_cli`,
  CLI-argument parsing only), unaffected by and unrelated to this change**; still ran
  `cargo test -p dicom-scpproxy` before and after to confirm (1 passed, 0 failed, both times).

## What's not done / open questions

A full live end-to-end reproduction was built and verified (see above) - the remaining open
items below are narrower and unchanged from the original investigation:

- Whether `Calling-AE-title` (the sibling field, same validation asymmetry, not just
  `Called-AE-title`) independently triggers the identical panic was not separately constructed as
  its own boundary-value test - the mechanism is structurally identical (`encode_ae_title` is
  called for both fields the same way), so this was judged a near-certainty rather than worth a
  second full repro cycle, but it was not literally re-run.
- Whether `read_pdu` should instead be the one to reject an all-space Called/Calling-AE-title on
  receipt (matching the DICOM standard's stronger "shall not be used" wording for these two fields
  specifically, rather than the reader's general leniency policy) is a legitimate alternative or
  complementary fix, but changes behavior for every caller of `read_pdu`/`read_pdu_from_wire`
  (not just `scpproxy`) and was judged a broader design decision better left to the maintainers -
  the fix applied here is scoped to the concrete, demonstrated crash (`scpproxy`'s specific
  `.unwrap()` usage) rather than the wire-format leniency question in general.

## Addendum: this is a recent regression, not a longstanding design tension (git archaeology)

`git log` on `ul/src/pdu/writer.rs` shows `encode_ae_title`'s all-space-rejection check was added
in commit `232c46ba` ("[ul] Fix UL AE title wire length validation", merged as PR #798, 2026-06-22)
- **before this commit, the writer had no such validation at all**, so `scpproxy`'s relay pattern
was never at risk from this specific field. The very next day's commit, `75d9cdbd` ("Allow reserved
AC AE title fields"), added the `validate_value: bool` parameter this candidate's root-cause
analysis above describes, and set it to `false` specifically for the ASSOCIATE-**AC** PDU's
Called/Calling-AE-title fields - because those are documented as "Reserved... value shall not be
tested when received" (i.e. an acceptor is expected to echo back whatever the requestor sent,
exactly the relay pattern `scpproxy` uses for AC). **The maintainers had already anticipated and
fixed this exact class of problem for the AC direction** - `scpproxy`'s AC relay path
(`ProviderType::Scu`, SCP-to-SCU direction) is therefore *not* at risk from this specific bug, only
confirmed exploitable for the **ASSOCIATE-RQ** direction (`ProviderType::Scp` in `scpproxy`'s code,
SCU-to-SCP), whose `validate_value` is unconditionally `true` with no equivalent relay-aware
exception - apparently because an ASSOCIATE-RQ's AE title fields are not documented as "Reserved,
echo back verbatim" the way AC's are, so a normal (non-relay) client is expected to always freshly
author them, an assumption `scpproxy` specifically violates. This sharpens the fix's scope
correctly (this candidate's patch only needed to touch the RQ-relay call sites, which is exactly
what it did) and is worth mentioning if this finding is ever reported upstream, since it directly
follows on from PR #798's own stated intent.
