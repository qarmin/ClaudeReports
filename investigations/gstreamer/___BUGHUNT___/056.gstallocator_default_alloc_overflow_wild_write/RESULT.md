# gstallocator_default_alloc_overflow_wild_write

`subprojects/gstreamer/gst/gstallocator.c` `default_alloc()` computes `gsize maxsize = size +
params->prefix + params->padding;` with no overflow check before passing it into
`_sysmem_new_block()`. Flagged by `findings.toml` (`OVF_25`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Standalone C program calling only the public `gst_allocator_alloc()` API with `size = G_MAXSIZE - 100`, `prefix = 0`, `padding = 200`, `flags = GST_MEMORY_FLAG_ZERO_PADDED` | Core-only meson build, plain debug, run directly and under `valgrind` | No crash. Added temporary debug prints directly into a local build of `gstallocator.c` to inspect the actual runtime values: `maxsize` did wrap to a tiny number (106) as expected, and the later `padding = maxsize - (offset + size)` computation in `_sysmem_new_block()` algebraically simplifies (mod 2^64) to exactly `prefix + padding_param - offset` regardless of `size`'s value (the huge `size` terms cancel out) - so the *padding byte count* itself stays small (207) and the vulnerable `memset` target pointer (`data + offset + size`) wraps around 64-bit pointer arithmetic to land only ~100 bytes *before* `data` - close enough to still be inside other legitimately-allocated heap memory, so nothing crashed and even `valgrind` reported 0 errors (memcheck doesn't flag a write into a *different* valid allocation as invalid). |
| Same harness, refined: `padding` raised to 16 MiB (`size` adjusted to keep the same overflow relationship) so the wrapped-around target pointer lands ~16 MiB before `data` - almost certainly outside any single small heap allocation's mapped region | Same build/tooling, both a raw run and under `valgrind` | **Reproduces**: crashes with `SIGSEGV` (exit 139) on every plain run. Under `valgrind`: "Process terminating with default action of signal 11 (SIGSEGV) ... Bad permissions for mapped region at address 0x4C01518", stack trace pinpointing `memset (vg_replace_strmem.c:1391)` called from `_sysmem_new_block (gstallocator.c:464)` called from `default_alloc (gstallocator.c:547)` called from the public `gst_allocator_alloc (gstallocator.c:336)`. |

## Status

**Confirmed** - reproduces 3/3 runs (all three plain runs segfaulted; a fourth run captured under
valgrind also crashed identically with the same stack trace).

**Fully reproducible**: **Yes**. The repro calls only the real, unmodified, public
`gst_allocator_alloc()` API with caller-controllable `size`/`GstAllocationParams` - no internal
function is called directly, and the crash occurs entirely inside real, unmodified GStreamer code.

## Class

crash (wild out-of-bounds write, SIGSEGV)

## Root cause

`default_alloc()` computes `maxsize = size + params->prefix + params->padding` without checking
for overflow; for a `size` close to `G_MAXSIZE`, this silently wraps to a small value. That small,
wrapped `maxsize` is what `_sysmem_new_block()` uses to allocate the actual backing memory (via
`g_try_malloc`), so only a tiny real buffer gets allocated - but the *original*, huge `size` is
passed through separately and used again inside `_sysmem_new_block()` to compute `padding =
maxsize - (offset + size)`. Because `maxsize` was derived from `size + prefix + padding_param` in
the first place, this second computation algebraically cancels the `size` term back out (mod
2^64), so `padding` itself ends up as a small, plausible-looking number - but the actual `memset`
*target pointer*, `data + offset + size`, uses the raw huge `size` value directly in pointer
arithmetic. Since 64-bit pointer arithmetic wraps modulo 2^64 in practice, `data + (huge size)` is
equivalent to `data - (a value close to prefix+padding)` - i.e. a pointer landing some distance
*before* the tiny real allocation, not a "wildly" random address. Whether this crashes immediately
depends entirely on how far before the allocation that computed distance is: a small `padding`
value (as `findings.toml`'s description alone would suggest testing with) often still lands inside
some other legitimately-mapped heap memory (silent heap corruption, not immediately observable);
a larger `padding` value reliably lands outside any mapped region and crashes immediately, which is
what this repro uses to get a clean, deterministic reproduction. The underlying bug is one
missing overflow check in `default_alloc()` regardless of which magnitude of `padding` is used to
demonstrate it.

## Introduced by

e52aa65ab77f201fffe1639876db2a5e260d729b "allocator: small internal cleanups" (2013-02-26) - this
is when `default_alloc()` first took its current form computing `maxsize` this way; the pattern
appears to predate this specific commit in an equivalent shape further back, but this is the
earliest point in this file's tracked history where `git log -S` cleanly isolates the exact
current expression.

## The fix

Add an explicit overflow check in `default_alloc()` before computing `maxsize` - reject (return
`NULL`, with a `GST_CAT_WARNING` explaining why) if `params->prefix + params->padding` would
themselves overflow, or if adding `size` on top would overflow, rather than silently wrapping and
handing a corrupted, undersized `maxsize` to `_sysmem_new_block()`.

Verified: rebuilt with the fix and re-ran the same repro - `gst_allocator_alloc()` now returns
`NULL` cleanly (no crash, confirmed both via a plain run and under `valgrind`) for the exact same
input that previously crashed.

- Project test suite: `meson test -C build-core-repro gstreamer:gst_gstmemory` - 1/1 passed both
  before and after the fix.

## What's not done / open questions

- Only `default_alloc()` (the default system-memory allocator's `alloc` vfunc) was checked; other
  custom `GstAllocator` implementations elsewhere in the codebase (e.g. DMA-BUF, GL memory
  allocators) that compute their own `maxsize`-style values from caller-supplied
  `size`/`GstAllocationParams` were not audited for the same pattern.
- The "lands inside another live allocation instead of crashing" case (the original, smaller-
  padding construction) is itself worth a follow-up: it's a real silent heap-corruption primitive
  that this session's repro doesn't specifically demonstrate corrupting *observable* data in a
  neighboring allocation (it only demonstrates the crash variant, which was sufficient to confirm
  and root-cause the bug).
