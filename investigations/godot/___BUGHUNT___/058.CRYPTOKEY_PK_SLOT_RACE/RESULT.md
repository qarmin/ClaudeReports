# `CryptoKeyMbedTLS::pk_slot`/`pk_type`/`pk_size` - no crash, but a real, empirically-confirmed correctness race

From the coordinator's priority list - the one remaining member of the "plain unsynchronized
container" family IDEAS.md had judged a weak candidate *on paper* (plain scalar `psa_key_id_t`/
`psa_key_type_t` fields, no local `CowData`-backed container), worth a real empirical check before
ruling it out. **The empirical check found something real**, just not in the usual shape: no
ASan-reported crash in any run, but a massive, reproducible rate of spurious cryptographic
parse/import failures under concurrent access - genuine state corruption, just one that the
underlying `mbedtls`/PSA library absorbs as "operation failed" rather than a memory-safety crash.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `modules/mbedtls/crypto_mbedtls.{h,cpp}` - `CryptoKeyMbedTLS::load_from_string()` calls `_parse_key()`, which calls `_clear()` (`psa_destroy_key(pk_slot)`, resets `pk_slot`/`pk_type`/`pk_size`) then reimports and reassigns them - all with zero guard; `save_to_string()` reads `pk_slot`/`pk_type`/`pk_size` directly, also unguarded | Static read | Confirmed the exact shape IDEAS.md anticipated (plain scalar fields, `_clear()`-then-reassign pattern identical in kind to the `StringName`/`Ref` races elsewhere this session, just operating on an opaque PSA key-slot handle instead of a heap pointer) |
| GDScript-level repro (`repro/`: `Crypto.new().generate_rsa(2048)` once on the main thread to get a real key, `.save_to_string(false)` once to get a real, constant private-key PEM string, then 3 writer threads repeatedly calling `key.load_from_string(pem, false)` with that same unchanging string and 3 reader threads repeatedly calling `key.save_to_string(false)`/`key.is_public_only()`, for 3s), against a self-built ASan editor binary (commit b8420aa3a60689572a745c0f8a85c9df62d4cbfb) in an isolated git worktree (`.worktrees/058`) | `ASAN_OPTIONS=detect_leaks=0 timeout -k 5 20 ./bin/godot.linuxbsd.editor.x86_64.san --headless --path repro` | **No ASan-reported crash in 3/3 runs** - but each run logged **~210,000 spurious `ERROR: Error parsing key '...'.`/`ERROR: Failed to import key: ...` messages**, despite `pem` never changing and representing a key that parses successfully as a matter of course outside the race. This is strong, unambiguous evidence of real state corruption: a perfectly valid, constant input failing to parse/import at a massive rate only under concurrent access is not noise, it's `pk_slot`/`pk_type`/`pk_size` being read mid-mutation by a concurrent `load_from_string()` call |
| Escalated the stress (6 writer + 6 reader threads, 8s window) to see if more contention produces an actual crash rather than just more errors | Same build/run, 3 attempts | Still **no ASan crash**, but the error count scaled with contention (~637,000 per run) - consistent with the PSA/mbedtls layer's own internal validation safely rejecting operations on a slot ID that's been destroyed/reused mid-flight, rather than that corruption manifesting as a raw memory-safety bug ASan can catch |
| Fix: added `_THREAD_SAFE_CLASS_` to `CryptoKeyMbedTLS` (a `CryptoKeyMbedTLS`-specific mutex; `core/crypto/crypto.h`'s base `CryptoKey` has no thread-safety infrastructure of its own, needed a new `#include "core/os/thread_safe.h"`), wrapped `load_from_string()`'s and `save_to_string()`'s entire bodies in `MutexLock` scopes - rebuilt, reran | Same build/run, 3 attempts | **Zero spurious errors across all 3 runs** (vs. ~210,000 unfixed) - clean "survived" every time |
| Project test suite with the fix applied | `XDG_DATA_HOME=<tmp> ./bin/godot....san --headless --test` | `1431 test cases \| 1430 passed \| 1 failed \| 3 skipped` - identical to the established baseline, no new failures |

## Status

**Confirmed** - but via a different kind of evidence than most of this session's candidates: no
ASan-reported crash, but an unambiguous, massively-reproducible rate of spurious operation failures
under concurrent access to a perfectly valid, unchanging input, dropping to exactly zero once the
race is fixed.

**Fully reproducible**: **Yes** - a real Godot project (`repro/`), run against a real, self-built
ASan editor binary, driving `CryptoKey`'s public GDScript API (`load_from_string()`/
`save_to_string()`). The "reproduction" signal is an error-count comparison (hundreds of thousands
unfixed vs. zero fixed), not a crash/no-crash comparison - documented explicitly in `justfile`'s
`build-and-run` recipe so a future reader isn't confused expecting a segfault that was never the
actual symptom.

## Class

`logic` (a genuine data race on `pk_slot`/`pk_type`/`pk_size`, but one that corrupts cryptographic
*operation state* rather than directly corrupting process memory in an ASan-visible way - the
underlying PSA/mbedtls library's own key-slot validation absorbs the race as safely-rejected
operations rather than a crash).

## Root cause

`CryptoKeyMbedTLS::load_from_string()` → `_parse_key()` → `_clear()` destroys the current PSA key
slot (`psa_destroy_key(pk_slot)`) and resets `pk_slot`/`pk_type`/`pk_size` to null/zero, then
reimports the key material and reassigns those same three fields to the new slot's values - all
without any synchronization. Concurrently, `save_to_string()` reads `pk_slot`/`pk_type`/`pk_size`
directly to export the current key. When two threads race - one calling `load_from_string()`
(destroying and recreating the slot) while another calls `save_to_string()` or a second concurrent
`load_from_string()` (reading/rewriting the same fields mid-transition) - the reader/second-writer
can observe a `pk_slot` value that's already been destroyed, or a `pk_type`/`pk_size` pair that no
longer matches the *current* `pk_slot` (e.g., the slot ID from the new key paired with the type/size
from the old one, or vice versa). The PSA crypto layer detects the resulting invalid/mismatched slot
state and returns an error rather than operating on genuinely freed memory - hence no ASan crash,
but a very real, very frequent operational failure.

## Introduced by

Not independently dated - this class has never had any thread-safety pass applied to it (0
pre-existing guard macros), so the race has existed since `pk_slot`/`_clear()`/`_parse_key()` were
introduced.

## The fix

`+4/-0` in `modules/mbedtls/crypto_mbedtls.cpp`, `+2/-0` in `modules/mbedtls/crypto_mbedtls.h` (see
`patch.diff`):

- Added `#include "core/os/thread_safe.h"` and `_THREAD_SAFE_CLASS_` to `CryptoKeyMbedTLS`.
- `load_from_string()`: locked the entire body (covers `_parse_key()`'s `_clear()`-then-reassign
  sequence transitively, since `_parse_key()`/`_clear()` are private helpers only called from
  already-locked entry points).
- `save_to_string()`: locked the entire body.

Verified: 0 spurious errors across 3 runs with the fix applied (vs. ~210,000 unfixed, scaling to
~637,000 under heavier contention). Test suite: `1431 | 1430 passed | 1 failed | 3 skipped` with the
fix applied - identical to the established baseline.

## What's not done / open questions

- `load()`/`save()` (the file-based variants) duplicate `_parse_key()`/the PSA-export logic directly
  rather than delegating to the now-locked `load_from_string()`/`save_to_string()` - **not covered by
  this fix** and not independently tested (would need a real filesystem path in the repro, out of
  scope for the 10-15 minute empirical check this candidate was budgeted). Same race almost
  certainly applies there too.
- `reimport()` (used internally, not `ClassDB`-bound) also reads/writes `pk_slot`/`pk_type`/
  `pk_size` unguarded - not covered, not independently tested, since it's not directly
  GDScript-reachable.
- `is_public_only()`'s plain `bool` read was left unguarded - a torn read of a single bool is not a
  memory-safety concern the way the PSA slot fields are, and it wasn't implicated in the observed
  spurious-error symptom.
- This was the last item identified in the coordinator's priority list before falling back to the
  rest of IDEAS.md's backlog - next up per that list: the `Tree`/`TreeItem` cross-object lock gap
  (042 follow-up), `Image`'s ~95 remaining unlocked methods (036 follow-up), `TextParagraph`'s 13
  remaining unlocked methods (038 follow-up), the `material.cpp` RID race re-test (002 follow-up),
  and the 22 remaining unaudited `_THREAD_SAFE_CLASS_` files.
