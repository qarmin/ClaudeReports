# 047. PGO: -40MB of startup RSS and -4% startup time, with no runtime cost (and one overfitting trap)

- **Project**: godotengine/godot
- **Base commit**: `9552dfb6859a1aaba1e570b8e0ef5c599b830f19`
- **Metric(s) measured**: binary size, per-mapping resident memory (`smaps`), peak RSS, Godot's
  tracked peak heap, editor startup wall time, and 6 runtime benches - measured both with the
  benches **inside** the PGO training set and with them **held out**
- **Workload**: empty-project editor startup + `harness/gdbench` modes
- **No `patch.diff`**: like candidate 045 this is a **build-configuration** result, not a source
  change. Godot's SCons has no `use_pgo` option; the arms were produced with `ccflags`/`linkflags`
  (exact invocations in `bench/measurements.txt`).

## What was tried

Candidate 045 established that ~102MB of the editor's startup RSS is the binary's own resident pages,
and that `optimize=size` recovers 25MB of it but costs +8.6% startup and **+17-42% on
computation-heavy runtime paths**. It flagged PGO as the more promising version of the same idea,
because better code layout could reduce the *touched fraction* of the binary instead of trading size
for speed.

This candidate builds that: `-fprofile-generate` build -> run a training workload -> `-fprofile-use`
rebuild. Crucially it was done **twice with different training sets**, because the first run's
results looked too good:

- `pgo(all)`: trained on editor startup **and** all 7 gdbench modes.
- `pgo(startup)`: trained on editor startup **only**, leaving the benches held out.

## Result

**Memory - robust, and independent of the training set:**

| | default | pgo(all) | pgo(startup) |
|---|---|---|---|
| binary on disk | 175.4 MB | 141.1 MB (-19.5%) | 141.1 MB (-19.5%) |
| resident binary pages | ~103.7 MB | ~66.7 MB (**-35.7%**) | - |
| peak RSS | 702.5 MB | 662.2 MB (**-40.28 MB, -5.73%**, 6/6 pairs) | **-36.48 MB (-5.23%)**, 4/4 pairs |

Godot's tracked counters are identical within **32 bytes** across arms, and `[heap]`/`[anon]` are
unchanged, so every megabyte of this is binary pages - the component candidate 039 identified and
no source-level candidate can touch.

**Startup time: -4.0% to -5.05%** (2529 -> 2401 ms for `pgo(all)`, 2430 -> 2334 ms for
`pgo(startup)`; faster in 6/6 and 5/5 pairs respectively). Startup is in the training set for both
arms, which is the realistic deployment case - you would train on startup if you shipped this.

**Runtime: neutral to marginally better, once measured honestly.** This is where the two training
sets matter:

| bench | pgo with bench **in** training | pgo with bench **held out** |
|---|---|---|
| `variant_call` | **-17.8%** | **-0.5%** |
| `vector_math` | -0.8% | -1.1% |
| `regex` | -2.0% | -0.7% |
| `string_concat` | -0.7% | -1.3% |

**The -17.8% on `variant_call` is profile overfitting, not a transferable speedup.** Trained on the
very benchmark it was then measured on, GCC laid out `Variant` dispatch for that exact call mix;
with the benches held out, the same path reads -0.5%. Had this candidate stopped after the first
build - which is what the plan called for - it would have reported "PGO: -40MB **and** -17.8% on the
GDScript hot path", which is wrong in a way no amount of extra runs would have revealed.

The honest summary: **PGO buys ~36-40MB of RSS and ~4% startup for free**, and its runtime effect on
untrained workloads is around -1%, i.e. real but small.

## Does it change behavior/output?

No source difference exists between arms, so no behavioral change is possible by construction.

**Test suite**: 1427 cases, 1426 passed, 1 failed, 3 skipped on all three arms (default, both PGO
builds) - the known `test_ip.cpp` IPv6 failure.

## Why the result looks the way it does

PGO gives GCC real branch and call-frequency data, so it (a) moves cold code out of hot pages,
shrinking the *touched* text footprint - which is why resident binary pages fall 35.7% while the
binary itself only shrinks 19.5% - and (b) inlines and lays out the hot paths better, unlike `-Os`
which gives up inlining across the board. That is exactly why this succeeds where candidate 045
failed: `optimize=size` made `variant_call` **+17.6% worse**, PGO leaves it unchanged (held out) and
gets more memory back besides.

## What's not done / open questions

- **The real cost is process, not performance.** PGO needs a two-phase build (train, then rebuild),
  which roughly doubles release build time (6m13s + 4m06s here versus ~4m48s for a plain build), and
  a profile that goes stale as the code changes. Whether Godot's release pipeline wants that is an
  engineering-process question this file does not answer.
- **The training workload used here is trivial** (empty-project editor startup). A real deployment
  would train on opening a substantial project, editing scenes and scripts, and running a game -
  which would plausibly move the runtime numbers further than -1%, and could change the RSS figure
  in either direction.
- **`lto` was not combined with PGO.** Godot's SConstruct has an `lto` option (`auto`/`thin`/`full`)
  which is off for this target; LTO+PGO together is the configuration a release build would
  realistically use, and was not measured.
- **Only gcc was tested**; clang's PGO (`-fprofile-instr-generate`/`-fprofile-instr-use`) may differ.
- No `perf` profile was taken to confirm *which* layout decisions produce the page-footprint drop.
