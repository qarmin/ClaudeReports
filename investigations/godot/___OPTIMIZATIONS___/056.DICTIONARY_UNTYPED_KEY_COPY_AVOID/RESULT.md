# `Dictionary` accessors skip the key copy + no-op `validate()` call for untyped dictionaries

Every `Dictionary` accessor (`operator[]`, `getptr`, `get`, `get_valid`, `get_or_add`, `set`, `has`,
`has_all`, `erase`) starts with `Variant key = p_key;` (a full `Variant` copy) so the copy can be
passed to `typed_key.validate(key, ...)`, which takes a non-const reference and may coerce the key
for *typed* dictionaries (`Dictionary[int, String]` etc.). For the common **untyped** dictionary
case (`typed_key.type == Variant::NIL`), `validate()` returns immediately without touching the key
at all - so the copy is pure overhead, and when the key is a `String`/`StringName`/`Array`/any
`RefCounted`, that copy is an atomic refcount increment+decrement on every single access. Open PR
**godotengine/godot#121081** (`MERGEABLE` against current `HEAD`, applies clean, single file) adds
an untyped fast path to each accessor that uses `p_key` directly, skipping both the copy and the
no-op validation call. Typed dictionaries are entirely unaffected - they still take the existing
coercing path.

Toolchain: `g++` (GCC) 15, SCons 4.11.1, `scons platform=linuxbsd target=editor tests=yes
debug_symbols=yes`.

## What was tried

Benchmark: three new `bench.gd` modes matching the PR author's own benchmark shape (string-keyed
dictionary, fixed key, tight loop) - `dict_string_key_read` (`d["the_key_name"]`, read),
`dict_string_key_write` (`d["the_key_name"] = i`), `dict_string_key_contains` (`d.has("the_key_name")`),
each 5,000,000 iterations on a 3-entry untyped `Dictionary`. Measured via **5 interleaved
baseline/patched pairs**, `uptime` checked before starting (load average 0.70-1.43 on a 24-core
box).

| Pair | read: baseline us/iter | read: patched us/iter | write: baseline us/iter | write: patched us/iter | contains: baseline us/iter | contains: patched us/iter |
|---|---|---|---|---|---|---|
| 1 | 0.037066 | 0.033308 | 0.028829 | 0.025098 | 0.051014 | 0.041971 |
| 2 | 0.037354 | 0.035040 | 0.028815 | 0.025166 | 0.049926 | 0.042684 |
| 3 | 0.038290 | 0.034355 | 0.028641 | 0.028876 | 0.053458 | 0.043497 |
| 4 | 0.045500 | 0.034195 | 0.036535 | 0.025440 | 0.055156 | 0.042964 |
| 5 | 0.036952 | 0.034213 | 0.029096 | 0.025492 | 0.050063 | 0.042591 |
| **mean** | **0.039032** | **0.034222** | **0.030383** | **0.026014** | **0.051923** | **0.042741** |

## Result

**-12.3% for string-key read, -14.4% for string-key write, -17.7% for string-key `has()`**
(wall time, means above). 4/5 pairs non-overlapping per operation (pair 3's write measurement is
the one exception - baseline 0.028641 vs patched 0.028876, patched marginally *higher* that one
time, a single noisy outlier against 4 other pairs all showing a clear, consistent win in the same
direction). Real, larger in relative terms than the PR author's own reported numbers (their
Windows/release-editor benchmark: -6.9% read, -9.4% write, -6.7% `in` check) - consistent with this
session's general pattern of PR-claimed numbers being a lower bound rather than an exact match
across different hardware/toolchains (see `projects/godot.md`). Time only; this session did not
measure allocation count directly, though the mechanism (removing one `Variant` copy - an atomic
refcount round-trip for `RefCounted`-backed key types like `String`) plausibly reduces atomic
operation count, which isn't quite the same axis as heap allocation count and wasn't separately
instrumented here.

## Does it change behavior/output?

- **Bit-exact.** Dedicated correctness script (`bench/dict_correctness_test.gd`) covers: untyped
  dictionary read/write/`has`/`has_all`/`erase`/`get`-with-default/`get_or_add`, a **typed**
  dictionary (`Dictionary[int, String]`) including key coercion (accessing an int-typed dict with a
  `float` key, confirming the *typed* path - untouched by this patch - still coerces correctly), a
  typed `String`-key dictionary with erase, an `Array`-typed (reference type) dictionary key, and a
  1000-entry stress dictionary with string keys - all `ALL_TESTS_PASS` on both arms.
- **Is this a breaking change?** No public API/file format change - internal accessor
  implementation only, typed-dictionary coercion semantics are explicitly preserved (verified
  above, not just assumed from reading the diff).
- Project test suite: **1427 cases, 1426 passed, 1 failed, 3 skipped both before and after** (same
  known pre-existing `test_ip.cpp` IPv6 failure) - identical.

## Why the result looks the way it does

`Dictionary` is one of the two most-used container types in GDScript (alongside `Array`), and
almost all real-world usage is untyped (typed dictionaries, `Dictionary[K, V]`, are a newer,
less-adopted GDScript 2.x feature). Removing one `Variant` copy - which for a `String`/`StringName`
key means an atomic refcount increment then decrement, since `String` is `CowData`-backed and
refcounted - from every single dictionary access is a small, fixed per-call saving, but `has()`
benefiting the most (-17.7%) makes sense: `has()`'s own useful work (a single hash lookup, no
mutation) is proportionally smaller than `operator[]`'s, so the removed copy is a *larger* fraction
of `has()`'s total cost than of a full read or write.

## What's not done / open questions

- Only string keys were benchmarked (the PR's own table also covers int keys, showing a smaller
  effect there since `int` isn't refcounted - no atomic traffic to remove, just the smaller
  `Variant` copy-construct/destruct cost) - not separately measured in this session.
- Allocation *count* (as opposed to atomic-op count) was not separately instrumented - the two are
  related but not identical, and this session didn't build the allocator-wrapper instrumentation
  needed to distinguish them precisely.
- `set()`, `has_all()`, `erase()` and `get_or_add()` are all touched by the same patch but were not
  individually timed - only `operator[]` (read/write) and `has()` were benchmarked; the correctness
  script does exercise all of them, just not for timing.
