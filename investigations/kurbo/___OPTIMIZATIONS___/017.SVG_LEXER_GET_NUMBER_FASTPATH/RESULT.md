# 017. SVG_LEXER_GET_NUMBER_FASTPATH

`SvgLexer::get_number` (`kurbo/src/svg.rs:327`) is where the `svg-parse` scene spends ~70% of its
time. It is a **two-pass** design: a byte scanner walks sign/digits/`.`/exponent one
`get_byte()`/`unget()` at a time to find the token's extent, then hands the resulting `&str` to
`str::parse::<f64>()`, whose own `dec2flt::parse_number` walks the same digits a second time. This
candidate makes the first pass accumulate the mantissa and decimal exponent it is already reading
past, and finishes the conversion itself whenever Clinger's exactly-rounded fast path applies
(mantissa <= 2^53, `|exp10| <= 22`), falling back to `str::parse` otherwise.

## What was tried

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)` / `cargo 1.98.0 (797e8a9bc 2026-08-05)`
(recorded because the evidence below includes callgrind instruction counts).

### Step (a): where does the 70% actually go?

`callgrind --cache-sim=no` on `kurbo_bench svg-parse 3`, grouped by function (41,562,742 `Ir`
whole-program; ~5 M of that is the one-time `to_svg` that builds the scene's input):

| region | `Ir` | share of the number-parsing cost |
|---|---|---|
| scanner (everything inlined into `SvgLexer::get_number`: `svg.rs` + `num/mod.rs` + `slice/index.rs` + `str/traits.rs` lines) | 10.72 M | ~43% |
| `core`'s `dec2flt` (`parse::parse_number` + `FromStr::from_str` + `lemire::compute_float`) | 13.94 M | ~57% |
| `get_number_pair` | 1.13 M | ~5% |

So neither half dominates: ~510 `Ir` per number, split roughly 215 scanner / 280 `dec2flt`. Both
sub-candidates from the backlog were therefore implemented and measured.

### Step (b), rejected: replace `get_byte`/`unget` with direct slice indexing

`variant_a_slice_index.diff` in this folder (+20/-15). Semantically identical, `CHECK` bit-exact.
It **regressed**: `+6.9%` single-core and `+7.2%` multi-core on `svg-parse` (0 of 9 paired wins in
both directions), and `+3.4%` whole-program `Ir` (41,562,742 -> 42,969,893). The existing
`get_byte`/`unget` form already compiles to a register-resident cursor; hoisting `self.data
.as_bytes()` into a local `bytes` slice adds a second live ptr/len pair without removing a bounds
check, and moving the `ix += 1` to the bottom of the scan loop cost the original's tighter
increment-then-test shape. Reproduce with `just test-variant-a`.

### Step (c), shipped: single-pass mantissa accumulation + exact fast path

`patch.diff` (+55/-9, one function plus a 23-entry `POW10` table).

Timings, paired alternating runs via `tools/paired.sh`, `RUNS=9`; medians in ms. Two harnesses:
`kurbo_bench svg-parse` (the workspace's canonical scene) and `svgbench` (this candidate's own
bench, same 200x8-cubic path re-emitted at a chosen number of decimals - see "Why the result looks
the way it does" for why that second axis is necessary).

| Variant | scene | single-core p50 (`taskset`) | multi-core p50 | paired | notes |
|---|---|---|---|---|---|
| baseline | `kurbo_bench svg-parse` | 0.2430 | 0.2430 | - | 17-significant-digit input |
| variant A (slice index) | `kurbo_bench svg-parse` | 0.2610 (+6.9%) | 0.2620 (+7.2%) | 0/9 | regression, rejected |
| **shipped (fast path)** | `kurbo_bench svg-parse` | **0.2080 (-14.5%)** | **0.2090 (-14.0%)** | 9/9 | bit-exact |
| baseline | `svgbench svg-parse-full` | 0.2390 | - | - | same input as `svg-parse`, cross-check |
| **shipped** | `svgbench svg-parse-full` | **0.2030 (-15.1%)** | - | 9/9 | bit-exact |
| baseline | `svgbench svg-parse-1` | 0.1521 | - | - | 1 decimal |
| **shipped** | `svgbench svg-parse-1` | **0.0868 (-42.9%)** | - | - | bit-exact |
| baseline | `svgbench svg-parse-2` | 0.1580 | 0.1576 | - | 2 decimals (typical real SVG) |
| **shipped** | `svgbench svg-parse-2` | **0.0937 (-40.7%)** | **0.0937 (-40.4%)** | 9/9, 8/9 | bit-exact |
| baseline | `svgbench svg-parse-3` | 0.1638 | - | - | 3 decimals |
| **shipped** | `svgbench svg-parse-3` | **0.0985 (-39.9%)** | - | - | bit-exact |
| baseline | `svgbench svg-parse-6` | 0.1927 | 0.2547 | - | 6 decimals |
| **shipped** | `svgbench svg-parse-6` | **0.1118 (-42.0%)** | **0.1374 (-46.2%)** | 8/9, 9/9 | bit-exact |

Confirmation pass at load average 16.5 (siblings building) reproduced every ratio: `-15.5%` /
`-15.5%` on `svg-parse` single/multi, `-43.3%` on `svg-parse-2`, `-15.6%` on `svg-parse-full`,
`-46.2%` on `svg-parse-6`, all 9/9. A `just test` run from a *fresh clone* of the pinned commit
reproduced `-19.3%` / `-17.2%` on `svg-parse` and `-45.4%` / `-43.1%` on `svg-parse-2`, all 9/9.

Callgrind (deterministic, immune to sibling load):

| scene | baseline `Ir` | patched `Ir` | delta |
|---|---|---|---|
| `kurbo_bench svg-parse 3` (whole program, incl. ~5 M one-time `to_svg`) | 41,562,742 | 36,471,984 | **-12.2%** |
| `svgbench svg-parse-2 3` (whole program, incl. the `format!`-based input build) | 35,791,997 | 27,446,272 | **-23.3%** |

Memory: `MEM allocs/iter 10.1 -> 10.0`, `bytes/iter 114696 -> 114696`, `peak_rss_kb` unchanged.
The patch allocates nothing; it is a pure-CPU change.

## Result

A consistent wall-clock win on every input measured, in both single-core and multi-core mode, at
9/9 (twice 8/9, where the one lost pair was a visible host-noise spike, not a close call):

- **-14% to -19%** on the workspace's existing `svg-parse` scene (full-precision, ~17-significant-
  digit numbers - 71.3% of its 10 000 numbers still qualify for the exact fast path).
- **-40% to -46%** on the same path re-emitted with 1/2/3/6 decimals, i.e. what real SVG path data
  from Inkscape/Illustrator/web tooling actually looks like.

I am leaning on the **wall clock**, with callgrind as direction confirmation. Per
`projects/kurbo.md`, `Ir` must not be the ranking metric here, and this candidate shows the usual
kurbo gap in the usual direction: `-23.3%` whole-program `Ir` against `-43%` wall clock on
`svg-parse-2` (the removed work is a branchy second scan plus `dec2flt`'s serial 128-bit
Eisel-Lemire chain, which retires fewer instructions than it costs in latency). The effect is far
larger than the measured noise floor (baseline re-runs drifted 0.2433/0.2439/0.2434 on an idle
host; the outlier runs the paired script exposes are 2x spikes from sibling candidates, and the
alternating A/B ordering cancels them).

Dimensions covered: **time** (primary) and **allocation count / bytes / peak RSS** (unchanged, as
expected - nothing about the patch touches allocation).

## Does it change behavior/output?

- **Bit-exact?** **yes**, on every input measured and on a dedicated 400 032-case cross-check.
- `CHECK` is byte-identical between the two builds on all six scenes (`kurbo_bench svg-parse` =
  `0x1aac21969db21d82`, and the five `svgbench` precisions).
- The fast path is guarded by exactly the same conditions `core`'s own `dec2flt` uses for its
  fast path (`RawFloat::MAX_MANTISSA_FAST_PATH == 2^53`, `MAX_EXPONENT_FAST_PATH == 22`) and
  performs the same single IEEE operation (`m * 10^e` for `e >= 0`, `m / 10^-e` for `e < 0`).
  Both operands are exactly representable, so one correctly-rounded multiply/divide is the
  correctly-rounded result of the exact decimal - there is no last-ulp freedom being exercised.
  Anything outside those guards (>19 digits, mantissa > 2^53, `|exp10| > 22`, exponents with more
  than 3 digits) falls back to `str::parse` unchanged.
- `bench/svgbench`'s `verify` scene independently confirms this: it feeds 400 032 generated
  number tokens (1-22 digits, decimal point at every position, all three sign forms, `e`/`E`
  exponents with optional sign and 1-4 digits, plus 32 hand-picked edge cases - `-0`, `1.`, `.5`,
  `9007199254740992`, `9007199254740993`, `1e22`/`1e23`, `5e-324`, `1e999`, 20+ digit mantissas,
  leading-zero runs) through `BezPath::from_svg` and compares the resulting `f64` **bit pattern**
  against `str::parse::<f64>()`. Baseline: `checked 400032 mismatches 0`. Patched: `checked 400032
  mismatches 0`.
- Sign of zero is preserved (`-0.0` stays `-0.0`); the fast path negates after computing the
  magnitude.
- Error behavior is unchanged: the same `SvgParseError::{UnexpectedEof, Wrong}` are returned from
  the same positions. On an `Err` return `self.ix` is left where the original left it in every
  case except the two `ok_or(Wrong)?` exponent paths, which is unobservable - every `get_number`
  error propagates out of `from_svg` with `?`, so the lexer is dropped without `ix` being read
  again.
- **Is this a breaking change?** no. `get_number` is private, `from_svg`'s signature and results
  are unchanged.
- Project test suite: `cargo test --workspace --all-features`
  - baseline: **252 passed, 0 failed** (187 + 10 + 55 + 0 across 4 `test result` lines, 1 ignored),
    no `^error` lines.
  - patched: **252 passed, 0 failed** (same 4 lines, same split), no `^error` lines.
  - No new failures; no test changed state in either direction.
- `cargo clippy --workspace --all-targets --all-features`: the only warning is a pre-existing
  `needless_late_init` in `kurbo/src/common.rs:389`, present before the patch too. No new lint.
- `cargo build -p kurbo --no-default-features --features libm` (the `no_std` path): builds clean.
  `cargo clippy -p kurbo --no-default-features --features libm`: same single pre-existing warning.
  The patch uses no float library functions - only integer arithmetic and one `f64`
  multiply/divide - so it is `no_std`-neutral by construction.
  (`--no-default-features` with *neither* `std` nor `libm` fails to compile `polycool`; verified
  that this is pre-existing on the unpatched tree, not caused by the patch.)
- MSRV: `unsigned_abs`, `saturating_add`, `u64::from`, `i32::from` are all long stable; nothing
  newer than 1.85 is used. No `unsafe`.

## Why the result looks the way it does

**Why the win is much bigger on short decimals.** The shipped patch removes `core`'s
`dec2flt::parse_number` second pass *and* its Eisel-Lemire conversion, replacing both with one
`f64` multiply or divide against a table entry. Whether that fires depends entirely on the input's
digit count, and the canonical `svg-parse` scene is unrepresentative here: it builds its input with
`BezPath::to_svg`, which formats every coordinate with `{}` (shortest round-trip), producing
15-17 significant digits (`315.57275592606555`). Counting the actual tokens of that exact input:

| input | numbers | take the exact fast path |
|---|---|---|
| `to_svg` full precision (the `svg-parse` scene) | 10 000 | 7 131 (71.3%) |
| 1 decimal | 10 000 | 10 000 (100%) |
| 2 decimals | 10 000 | 10 000 (100%) |
| 3 decimals | 10 000 | 10 000 (100%) |
| 6 decimals | 10 000 | 10 000 (100%) |

71.3% rather than 100% because a 16-digit mantissa is under 2^53 about nine times out of ten while
a 17-digit one never is. That gap - 71% vs 100% hit rate - is most of the difference between
`-15%` and `-43%`; the rest is that the short-decimal cases also have fewer bytes to scan. **Real
SVG path data is the 100% column**, which is why `bench/svgbench` exists: measuring only the
canonical scene would have under-reported this candidate by ~3x.

**Why the pure-scanner rewrite (variant A) lost.** The `get_byte`/`unget` pair looks like an
`Option` round-trip plus a redundant bounds check per byte, but `get_byte` is a small private fn in
the same module and gets inlined into a plain `cmp ix, len` / `movzx` / `inc ix`. Rewriting it
around a local `bytes` slice and a local `ix` does not delete a bounds check - it adds a live
ptr/len pair and restructures the loop's increment. The `Ir` count went *up* (+3.4%), so this
isn't a code-layout accident. Lesson for `svg.rs`: the byte cursor was not the problem; the
**second traversal** was.

**Why the wall-clock win exceeds the `Ir` win** (`-43%` vs `-23%` on `svg-parse-2`): consistent
with the pattern `projects/kurbo.md` records for candidates 001 and 006. `dec2flt` is a long
serial dependency chain (byte accumulate -> 128-bit multiply -> normalize -> assemble bits) with
several hard-to-predict branches; replacing it with `mantissa as f64` plus one table-indexed
multiply retires far fewer *latency-critical* instructions than the raw `Ir` delta suggests.

## What's not done / open questions

- The `dec2flt` fallback still re-scans the digits for the ~29% of full-precision numbers that
  miss the fast path. `core` also has an "extended" fast path (mantissa shifted left so
  `22 < exp10 <= 22+16` still resolves exactly) that this patch does not replicate; adding it
  would raise the 71.3% hit rate somewhat on `to_svg`-style input, at the cost of more code in
  `get_number`.
- Nothing was done about the *other* half of the profile that this patch does not touch:
  `get_number` still costs ~215 `Ir` per number in the scan itself, and variant A shows that
  cannot be fixed by a code-shape change. A genuinely faster scan would need to consume several
  bytes per step (SWAR over 8 bytes at a time), which is a bigger, harder-to-verify change.
- `bench/svgbench` is this candidate's own crate rather than a new scene on the canonical
  `harness/kurbo_bench`. It was kept separate because 5 sibling candidates were running against
  that shared harness concurrently. A future session should fold `svg-parse-<N-decimals>` into
  `kurbo_bench` proper and delete the duplicate generator - the `verify` scene is worth keeping
  either way.
- `to_svg`'s own cost is untouched and is still the larger of the two SVG numbers
  (`svg-write` 0.68 ms vs `svg-parse` 0.24 ms) - see the existing `IDEAS.md` row.
