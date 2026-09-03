# What is under test, and how it is measured

Read this first if picking the work up cold - it explains the harness, `SUMMARY.md` lists the
candidates measured with it.

kurbo is a pure-safe-Rust 2D curves/geometry library (`rust.unsafe_code = "forbid"` at workspace
level, MSRV 1.85, edition 2024, `no_std`-capable via the `libm` feature). A sibling
`___BUGHUNT___/` workspace exists at the repo root with 60+ correctness/panic findings; this
workspace is the performance counterpart and is numbered independently.

## Workload

kurbo ships six criterion micro-benches (`kurbo/benches/`: `cubic`, `cubic_arclen`, `quad_arclen`,
`quartic`, `nearest`, `rect_expand`) and they cover almost none of the surface area that matters to
downstream users - no flattening, no stroking, no dashing, no offsetting, no path building or
iteration, no affine transforms, no SVG parse/write. So this workspace builds its own harness
(`harness/kurbo_bench/`) covering those, and keeps the criterion benches for cross-checking a
solver-level change.

| scene | what it stresses |
|---|---|
| `flatten` | `kurbo::flatten` of a 200x8-cubic path at tolerance 0.25 - the single hottest kurbo entry point for any renderer |
| `flatten-fine` | same path at tolerance 0.001; ~7x the output per curve, so it separates per-curve setup cost from per-output-point cost |
| `flatten-small` | 20x8-cubic path at tolerance 0.25 - the second problem size required by MEASUREMENT.md section 6 |
| `mixed-flatten` | flatten of a path mixing `LineTo`/`QuadTo`/`CurveTo`, not cubics only - reaches the quad arm of `flatten` |
| `stroke` | `stroke()` expansion, width 4, round joins/caps, tolerance 1e-4 |
| `stroke-miter` | same, miter joins + butt caps - a different join arm of the stroker |
| `dash` | `stroke()` with a 4-entry dash pattern, i.e. `dash_iter` + stroker |
| `offset` | `offset::offset_cubic` over 2000 random cubics, d=5, tol=1e-3 |
| `simplify` | `simplify::simplify_bezpath` of a flattened path, accuracy 1.0 |
| `arclen` / `arclen-coarse` | `ParamCurveArclen::arclen` over 2000 cubics at 1e-9 / 1e-3 - accuracy is the knob that changes the subdivision depth |
| `area` | `Shape::area` + `Shape::perimeter` over a 200x8-cubic path |
| `bbox` | `Shape::bounding_box` (goes through `extrema`, so it reaches the quadratic solver) |
| `affine` | `BezPath::apply_affine` over a 200x8-cubic path |
| `svg-parse` / `svg-write` | `BezPath::from_svg` / `BezPath::to_svg` on a ~1600-element path |
| `nearest` | `ParamCurveNearest::nearest` over 2000 cubics, accuracy 1e-6 |
| `intersect` | `PathSeg::intersect_line`, 2000 cubics x 4 lines |
| `segments` | `BezPath::segments` iteration + `eval(0.5)`/`start()`/`end()` |
| `fit` / `fit-opt` | `fit_to_bezpath` / `fit_to_bezpath_opt` over offset-curve sources |
| `solve-cubic` / `solve-quartic` | `common::solve_cubic` / `solve_quartic`, 200k calls (these delegate to `polycool`) |
| `deriv` | `ParamCurveDeriv::deriv` + 8 evals over 2000 cubics |
| `subdivide` | `ParamCurve::subsegment` over 2000 cubics x 8 ranges |

Every scene builds its input **once**, outside the timed loop, from a fixed-seed xorshift PRNG, so
there is no test-data file to hunt for and no run-to-run input variation.

## Measurement harness

```
cargo build --release                                    # in harness/kurbo_bench/
taskset -c 2 target/release/kurbo_bench <scene> [iters]   # iters default 50
target/release/kurbo_bench --list                         # every scene + one-line description
target/release/kurbo_bench --dump <offset|stroke>         # not timed - see below
```

Three output lines per run:

```
scene flatten iters 50 warmup 5
RESULT median 0.9320 ms  mean 0.9326  p95 0.9449  spread 0.9239..0.9449
CHECK  0x9b1c...
MEM allocs/iter 2.1  bytes/iter 1064  peak_rss_kb 3216
```

- `RESULT` is a median with the spread printed, per MEASUREMENT.md section 4. Warmup is `max(2,
  iters/10)` iterations, discarded and not counted toward allocations.
- `CHECK` is an order-sensitive FNV-style mix of every f64 the scene produces (path elements,
  root sets, arclens, bounding boxes). It is **recomputed every iteration and compared against the
  first**, so a nondeterministic scene exits 3 with a loud message instead of quietly averaging
  two behaviours. It is also the bit-exactness check for a candidate: baseline `CHECK` == patched
  `CHECK` means the patch changed nothing observable in that scene.
- `MEM` comes from a counting global allocator wrapping `System` (count + bytes requested, over the
  timed region only), plus `VmHWM` from `/proc/self/status` for whole-process peak RSS. Allocation
  counts are deterministic and hardware-independent, so on a noisy host they are usable evidence
  when wall clock isn't.
- Everything passed into and out of a scene goes through `black_box`.

`--dump <offset|stroke>` is a separate, untimed mode: it prints **every coordinate the scene
produces**, one per line at `{:.17e}`, so two builds can be compared numerically instead of only by
checksum. It exists because `CHECK` answers "did anything change?" but not "by how much", and an
output-changing candidate has to answer the second (SKILL.md's characterized-difference rule).
Usage:

```
./bin_baseline --dump offset > base.txt
./bin_patched  --dump offset > patched.txt
paste base.txt patched.txt | awk '{d=$1-$2; if(d<0)d=-d; if(d>m)m=d} END {print "max abs diff", m}'
```

**Check the line counts match before trusting a paste-based diff.** If the two builds emit
different numbers of coordinates, the path *structure* changed (a different subdivision count) and
a coordinate-wise diff is misaligned past the first divergence - its numbers are then meaningless,
not merely large. Candidate 004 hit exactly this on the `stroke` scene (113 178 vs 113 088 values)
while the `offset` scene stayed aligned; its `RESULT.md` shows how to report that honestly.

## Instrumentation added for this workspace

| what | where | why |
|---|---|---|
| the whole `kurbo_bench` harness | `harness/kurbo_bench/` | kurbo's own benches only cover 4 solver/arclen micro-cases; nothing exercised flatten/stroke/offset/SVG, which is where downstream time actually goes |
| counting global allocator | `harness/kurbo_bench/src/main.rs` | `stroke`/`fit`/`simplify` allocate heavily (`stroke` requests ~1.36 MB and ~25 allocations per iteration); allocation count is the one deterministic column on a noisy host |
| per-iteration checksum comparison | same | catches a nondeterministic scene at the source instead of letting it become an unexplained spread |
| `OffsetCurve` (a local `ParamCurveFit` impl) | same | kurbo's own `CubicOffset` is **private**, and the only public `ParamCurveFit` impl is `SimplifyBezPath`, which the `simplify` scene already covers - so the `fit` scenes supply an offset curve of a cubic (the motivating use case in `fit.rs`'s own docs) computed analytically |
| `--dump` mode | same | added during candidate 001; `CHECK` says *whether* output changed, this says *by how much*, which is what a characterized-difference write-up needs |

The harness is **untracked in the kurbo repo** - a fresh `git clone` of `repo_url` will never
contain it. Per `SKILL.md`, exactly one canonical copy lives at `harness/kurbo_bench/`, and each
candidate's own `justfile` copies it into its `./clone` and rewrites the `kurbo` dependency path
(`../../../kurbo` -> `../kurbo`) as part of its `clone` recipe. That makes these candidates
self-contained *given the whole `___OPTIMIZATIONS___` folder kept together*, not as a single
isolated subfolder.

## Environment

- 24 cores; load average was drifting between ~2.5 and ~6 while otherwise idle at session start, so
  **use `tools/paired.sh`, not a naive twice-per-variant comparison** (MEASUREMENT.md section 2c).
- `perf_event_paranoid = -1` - full `perf record`/`perf stat` works, no sudo needed.
- `valgrind` (callgrind + cachegrind), `flock`, `just`, `hyperfine` all installed.
- rustc 1.98.0 / cargo 1.98.0.
- kurbo is pure computation with almost no I/O, so **callgrind's deterministic instruction count
  (`Ir`) is unusually well suited here** and sidesteps the wall-clock noise floor entirely for
  ranking a micro-change. Use it alongside wall clock, not instead of it - a cache-locality win
  won't show up in `Ir`.
- A patch must keep building under `--no-default-features --features libm` (the `no_std` path) and
  must not trip the workspace's strict lint set (`missing_docs`, `clippy::cast_possible_truncation`,
  ...), or it fails CI even when it's faster.

## Ground rules used in this workspace

1. No claim without a command that reproduces it - every candidate folder has the numbers and the
   exact command.
2. Every candidate that touches real source gets a real `patch.diff`, applied and built for real,
   plus `cargo test --workspace --all-features` before and after.
3. Null results are written down with the same care as wins - a null result with no file is
   indistinguishable from "never checked".
4. If a later candidate changes the baseline enough that an earlier candidate's numbers no longer
   hold, don't silently rewrite that candidate's `RESULT.md` - add a `CORRECTION.md` in its folder
   explaining what changed, and update its `SUMMARY.md` row to point at it.
