# 45 - what a property read costs, measured instead of divided

Ported from `perf_test/___OPTIMIZATIONS___/45_property_read_cost/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

Every traversal figure in this workspace is quoted in nanoseconds per item - ~86 ns (candidate 018),
~95-98 ns (candidate 015), ~40 ns for a geometry read (candidate 015's correction). All of them were
obtained by dividing a frame time by an item count. This measures the underlying unit directly:
`src/bin/property_probe.rs`.

## The numbers

2 000 000 reads each, `rdebug` profile:

| operation | ns |
| --- | --- |
| plain `get()`, no binding, no tracker | **4.13** |
| `get()` on a clean binding, no tracker | 4.13 |
| 100 000 distinct properties, no tracker | 4.20 |
| the same property re-read inside a tracker | 4.68 |
| **100 000 distinct properties, inside a tracker** | **9.01** |
| one `PropertyTracker::evaluate` per read | 9.38 |
| `get()` after dirtying, so the binding re-evaluates | **18.94** |

So: **a property read is ~4.2 ns, registering a dependency roughly doubles it to ~9 ns, and
re-evaluating a dirty binding is ~19 ns** - 4.5x a clean read.

The distinction between rows 4 and 5 matters and is easy to get wrong: reading the *same* property a
million times inside one tracker registers the dependency once, so it measures candidate 008's
short-circuit rather than the registration. A traversal reads a different property every time. The
first version of this probe made exactly that mistake and reported 4.5 ns for the tracked case.

## Strings and callbacks

Added afterwards, because both are everywhere in a real UI and neither had ever been measured here.
Medians of three runs, all from the same process mode (the absolute level of a run varies by ~1.5x -
candidate 035's bimodality - so only within-run comparisons are meaningful):

| operation | ns |
| --- | --- |
| `SharedString` get | **4.00** |
| formatted string binding (`format!("item {:06}", n)`), clean | 4.63 |
| **formatted string binding, re-evaluated** | **65.13** |
| numeric binding, re-evaluated | 22.71 |
| **`Callback::call`** | **1.58** |
| plain boxed closure call (control) | 0.99 |

Three things follow:

- **Reading a string property is as cheap as reading a float** (4.00 vs 4.21 ns). `SharedString` is
  refcounted, so a read is a refcount bump, not a copy.
- **A callback call is 1.58 ns**, against 0.99 ns for a bare boxed closure - so Slint's `Callback`
  costs about **0.6 ns** over the closure it wraps, for taking the handler out of its `Cell` and
  putting it back. A UI with a callback on every row pays nothing for them.
- **Building a string costs 65 ns**, three times a numeric binding. At 20 000 rows that is 1.3 ms if
  every label is rebuilt - real, but **250x less than shaping the result** (17-30 us per `Text`,
  candidate 044). Formatting is not what makes text expensive.

The control needed `black_box`: the first version measured the plain closure at **0.00 ns** because
the optimizer folded it away entirely. A micro-benchmark that reports zero has measured nothing.

## It reconciles with the traversal figures

`ItemRc::geometry()` is a `match` on the item index returning four property reads
(`item_geometry` in the generated code). Four tracked reads = **36 ns**, and candidate 015's
correction measured the geometry read in the input pass at **~40 ns** by ablation. Two independent
routes to the same number.

The rest of a ~90-124 ns item visit is therefore not property reads: it is the vtable dispatch, the
`ItemRc` construction, and the visitor machinery.

## What candidates 008 and 014 are worth at this level

Both fixes act exactly here - 08 skips a duplicate registration, 14 recycles the node a registration
needs. Disabling both (`tools/isolate.py off 14-node-pooling`, `off 08-skip-duplicates`):

| operation | without 08 + 14 | with | |
| --- | --- | --- | --- |
| distinct properties, inside a tracker | 28.89 ns | **8.90 ns** | **-69%** |
| same property re-read inside a tracker | 28.19 ns | **4.68 ns** | **-83%** |
| one tracker evaluation per read | 20.36 ns | **9.43 ns** | -54% |
| plain get, no tracker | 4.09 ns | 4.13 ns | unchanged, as expected |
| get after dirtying | 21.92 ns | 19.70 ns | -10% |

**A tracked property read was 28.9 ns and is now 8.9 ns - 3.2x.** At the frame level those two fixes
measured -8% to -25%, because a frame is full of work that is not property reads. The micro figure is
the honest size of what they changed, and the frame figure is the honest size of what it was worth.

(One oddity left as measured: "distinct properties, no tracker" reads 2.93 ns in the disabled build
against 4.20 in the enabled one. Both are the untracked path, which neither fix touches, so this is
codegen or layout noise rather than an effect.)

## Why this is worth having written down

It converts the workspace's central claim - "the engine's unit of currency is a property read" - from
an inference into a measurement, and it gives a ceiling for any future idea of the form "avoid N
property reads per item": **at 4.2 ns untracked and 9 ns tracked, saving one read per item saves
0.9 ms per 100 000 items.** Candidate 035 saved four of them per culled item and measured -12%; that
is consistent, and any proposal promising much more from the same lever is wrong before it is built.

## Reproduce

```sh
cargo run --profile rdebug --bin property_probe

tools/isolate.py off 14-node-pooling && tools/isolate.py off 08-skip-duplicates
cargo run --profile rdebug --bin property_probe
tools/isolate.py restore
```

## Does it change behavior/output?

N/A - measured/quantified only, no patch applied to real source in the original session. The
08/14 disable comparison used `tools/isolate.py`, which toggles those two already-existing fixes
off and back on for measurement purposes; nothing was changed in real source by this session.
