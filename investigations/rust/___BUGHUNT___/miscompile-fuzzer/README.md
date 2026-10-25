# miscompile-fuzzer

A reusable, parameterized Rust-program generator plus a differential-testing
harness for finding **new** rustc miscompilations. Built as real, extensible
code (not one-off programs re-authored by hand each session) so a future
session can just run it again, or add a new template category.

It does not touch the rust-lang/rust checkout, never builds rustc from
source, and never attempts a fix - see `../README.md` and `../AGENTS.md` for
why. It only generates small standalone `.rs` files, compiles them with a
prebuilt `rustc` (via `rustup`), runs the binaries, and diffs stdout/exit code.

## Two generator modes: random (`generator.py`) and static (`static_generator.py`)

This tool has two complementary, independent ways to produce a corpus - same
UB-freedom bar, same `diff_runner.py` for diffing either one, different
coverage strategy:

| | `generator.py` (random) | `static_generator.py` (static) |
|---|---|---|
| Templates live in | `templates/` | `templates_static/` |
| How a program is chosen | `random.Random` seeded from `(--seed, index)` | none - every declared combination, always |
| Coverage model | broad, varied, unbounded (raise `--count`) | an explicit, bounded, named scenario matrix |
| Re-running | same `(seed, index)` reproduces one program | same input always reproduces the *entire* corpus, byte for byte |
| "Add more coverage" means | usually nothing (templates already vary every run) | add one value to one axis list in a `templates_static/*.py` module |
| Default output dir | `corpus/` / `findings/` | `corpus_static/` / `findings_static/` |

Neither mode writes into the other's output directory, so they can be run
side by side (including while a long random campaign is already in
progress) without interfering with each other. Pick random mode for broad
exploration of a category's input space; pick static mode when you want a
small, fully-enumerated, easy-to-reason-about regression matrix for a
specific pattern (e.g. "every combination of these 3 field-type sets × these
4 mutate patterns") - the static mode is the "more certain" one in the sense
that every generated program is an intentional, named point in a declared
matrix, not a sampled one, and the matrix is trivial to widen or narrow by
editing a plain Python list. This mirrors the declarative-spec-driven
generator pattern in `~/Projekty/Rust/hodow`'s `scripts/cpu/generate_tests.py`
(a signatures/policy table mechanically expanded into one Rust test per
declared combination) rather than cargo-fuzz-style randomized fuzzing.

### How it works (random mode)

1. **`generator.py`** picks a template category, seeds a `random.Random`
   deterministically from `(--seed, index)`, and asks that template to build
   one `.rs` program plus the stdout it should produce. Both the `.rs` and a
   companion `.expected` file land in the output directory (default
   `corpus/`). Same `(seed, index)` always reproduces the exact same program -
   useful for re-generating a specific finding later without storing it.

2. **`diff_runner.py`** compiles every `corpus/*.rs` under two (or more)
   rustc configurations that *must* agree for any UB-free program - by
   default the same `rustc` at `-C opt-level=0` vs `-C opt-level=3`, with
   `-C overflow-checks=off -C debug-assertions=off` pinned identically on
   both sides so a difference can only come from the flag you're deliberately
   varying. It runs everything through a `ProcessPoolExecutor` sized to
   `nproc` by default, so a few hundred small single-file programs take a
   couple of seconds wall-clock on a 24-thread host.

   For each program it checks two independent things:
   - **Cross-config divergence** (the real signal): configs disagree with
     each other on compile-success, stdout, or exit code. This is copied into
     `findings/<program>/` with every config's exact compile command, stderr,
     and stdout, plus the generator's predicted stdout, so nothing has to be
     re-derived by hand to start triaging it.
   - **Self-check mismatch** (a harness/template bug, *not* a finding): every
     config agrees with every other config, but not with the generator's own
     `.expected` prediction. This means the Python oracle in the template is
     wrong, not that rustc is wrong - fix the template, don't file it.

3. Why the third oracle matters: a real compiler bug shows up as
   `O0 != O3` directly. A template bug (the Python mirror of the Rust logic
   computed the wrong value) shows up as `O0 == O3 != expected` instead,
   because both configs correctly execute whatever the source actually says -
   so the two failure modes are structurally distinguishable, which is what
   let step 3 below get to a *clean* baseline instead of silently trusting an
   unverified harness.

## UB-freedom, by construction

Every template's docstring in `templates/*.py` states the specific
UB-avoidance argument for that category (e.g. "we only copy packed fields out,
never take `&field`", "both union fields always hold a valid pointer into a
live `'static` array", "two live locals' addresses are always distinct").
Rules followed throughout:

- No reference/value is ever read before being written.
- No array/slice/pointer access goes out of bounds; pointer arithmetic via
  `.add()` always stays within `[0, len)` of the same allocation.
- Packed-struct fields are only ever copied out (`let v = s.field;`), never
  referenced (`&s.field` on an unaligned field is instant UB).
- Union fields only ever hold bit patterns valid for *every* field type they
  might be read back as (e.g. both fields are always "a valid pointer into
  live memory", never an arbitrary/partial bit pattern).
- Arithmetic that could overflow always goes through `wrapping_*` /
  `checked_*` / `saturating_*`, never a bare `+`/`-`/`*` - so behavior doesn't
  depend on `-C overflow-checks` (which the diff runner pins off either way).
- Inputs that an optimizer could otherwise constant-fold away entirely are
  wrapped in `std::hint::black_box(..)`, so the generated control flow is
  real work for the optimizer to reason about, not dead code it can prove
  unreachable.
- Every program is self-contained, single-threaded, and never prints anything
  address-, allocator-, or timing-dependent (no raw pointer values, no
  `HashMap` iteration order, no wall-clock reads).

If you add a template and can't write a one-paragraph UB-freedom argument for
it that actually holds up, don't add it - a generator that produces UB finds
nothing but noise (see the task brief this tool was built against).

## A second generation paradigm: identity chains

Every category above shares one structural weakness: it computes a value
FORWARD in Rust and predicts the same value independently in Python, so a
self-check mismatch is ambiguous between "the template's Python oracle is
wrong" and "rustc is wrong" (see "why the third oracle matters" above) - the
Python mirror is itself a real source of bugs, as several of the "Validation
result" sections below document (oracle bugs found and fixed in
`packed_struct_fields` and `static_merge_packed_const`).

`templates/identity_chain.py` (`identity_chain_safe`) and
`templates/identity_chain_unsafe.py` (`identity_chain_unsafe`) use a
different paradigm instead, one where the oracle structurally cannot drift
from the generated code no matter how deep or complex that code gets:

1. Start from a single `std::hint::black_box`'d constant of some type `T`
   (varied across `u8`/`u16`/`u32`/`u64`/`i32`/`bool`/`char` and a few small
   `Copy` tuples/arrays/structs).
2. Build an arbitrarily deep/wide tree of `if`/`match`/loop/function-call/
   recursion structures around it, where EVERY node is drawn from a small
   library of "identity transforms" - each one provably `v -> v` for every
   possible input, by a short, self-evident argument that never needs
   Miri-level stacked-borrows reasoning (every transform's one-paragraph
   argument is in `identity_chain.py`'s module docstring, the same
   convention every other template already follows for UB-freedom).
3. Because every node is individually an identity, the WHOLE tree is an
   identity regardless of how deep `--depth` or wide `--breadth` go - so the
   oracle never has to mirror the generated control flow. It only ever
   asserts `expected_stdout == the one starting constant, formatted`. A
   self-check mismatch here can only mean one specific thing: a transform in
   the library isn't actually an identity (a bug in `identity_chain.py` to
   fix), never "the Python mirror drifted from the Rust" - there is no
   mirror to drift.

`--depth`/`--breadth` (random mode only, `generator.py`; module defaults 8
and 3) are therefore a pure lever for arbitrarily complex nesting with no
corresponding growth in oracle complexity - the stated goal of this
paradigm. A shared, mutable node-count budget inside `ChainCtx` (see
`identity_chain.py`'s "Code-size safety valve") keeps source size and
compile time bounded even at large `--depth`/`--breadth` values, by having
`build_chain` degrade to a plain leaf once the budget is exhausted rather
than letting a wide `match` node multiply the tree exponentially.

`identity_chain_unsafe` is a separate, deliberately small, OPT-IN extension
(registered `default=False` - see "Opt-in categories" below) adding exactly
two patterns on top of the same safe library: a pointer round-trip
(`unsafe { *(&v as *const T) }`, mixed into the normal transform pool since
it's a true identity too) and, as one final step after the identity chain,
a union byte-reinterpretation (`union { whole: uN, bytes: [u8; N] }`) -
which is NOT an identity (it's a real bit-level reinterpretation), so its
oracle is computed mechanically via Python's `struct.pack`/`unpack` (native
byte order) rather than by mirroring any generated logic. See that module's
docstring for the full soundness argument for both patterns.

### Opt-in categories

`identity_chain_unsafe` is excluded from the default (no `--categories`)
rotation in both `generator.py` and `--list-categories`'s output, via
`templates/__init__.py`'s `register(name, default=False)` /
`NOT_DEFAULT` set - it remains fully usable with an explicit
`--categories identity_chain_unsafe` (or combined with other names). This
keeps the bulk of this tool's coverage on the safe path, per its intended
scope, while still making the small unsafe extension easy to reach
on demand. No other category currently uses `default=False`.

## Template categories (19)

| Category | File | What it targets |
|---|---|---|
| `identity_chain_safe` | `templates/identity_chain.py` | A structurally different paradigm from every other category below - see "A second generation paradigm" above. Deeply nested `if`/`match`/loop/call/recursion trees built entirely from provable `v -> v` identity transforms around a `black_box`'d constant, so the oracle stays `expected == starting constant` no matter how deep `--depth`/`--breadth` go |
| `identity_chain_unsafe` | `templates/identity_chain_unsafe.py` | Opt-in only (`--categories identity_chain_unsafe`, not in the default rotation) - adds a pointer round-trip identity transform plus a single final union byte-reinterpretation step (oracle via Python `struct.pack`/`unpack`, not a trivial-equals-constant check) on top of `identity_chain_safe`'s library |
| `union_ref_fields` | `templates/unions.py` | GVN/JumpThreading/DataflowConstProp failing to see a union sibling-field write invalidate a cached read (rust-lang/rust#160004, #161898, #163825) |
| `packed_struct_fields` | `templates/packed_structs.py` | GVN folding a `#[repr(packed)]` field projection (including through a `const`) into a misaligned literal (#163782) |
| `reborrow_chain` | `templates/reborrow.py` | ReferencePropagation mishandling a reborrow chain through nested field projections (#163777, #132898) |
| `pointer_provenance_roundtrip` | `templates/provenance.py` | Pointer-identity/exposed-provenance round-trip folding (#107975, #130388, #147538, #160267, #160722, #162078, #163541) |
| `match_shared_condition` | `templates/match_branch.py` | EarlyOtherwiseBranch/JumpThreading re-deriving a shared discriminant/condition incorrectly (#159618 family) |
| `enum_niche_layout` | `templates/enum_niche.py` | `Option<T>`/custom niche-enum layout correctness under branching |
| `static_merge_packed_const` | `templates/static_merge.py` | LLVM/GVN assuming distinct globals can alias or merge, when Rust guarantees distinct `static` addresses (#162078, #163541) |
| `checked_wrapping_arithmetic` | `templates/arithmetic.py` | checked/wrapping/saturating arithmetic across `#[inline(always/never)]` boundaries (#149294/#155952 family) |
| `nested_loop_mutable_state` | `templates/control_flow.py` | Labeled `break`/`continue` out of nested loops with live mutable state (#163311) |
| `const_eval_array_ops` | `templates/const_eval.py` | Const-eval (CTFE) of a `while`-loop/branching `const fn` building a `const` array vs. the identical function executed at runtime - GVN/const-prop folding a `TABLE[idx]` read to a value that disagrees with its own runtime recomputation |
| `trait_object_devirt` | `templates/trait_objects.py` | Default-vs-override trait methods called through a concrete type, a generic (`T: Trait`), a `&dyn Trait`, and a `Vec<Box<dyn Trait>>` all disagreeing - the DeduceParamAttrs/devirtualization family (#163856) |
| `closure_capture_boundary` | `templates/closures.py` | A closure mutating a `&mut`-captured local vs. a `move` closure's by-value snapshot vs. `Box<dyn FnMut>`, each threaded through randomly-assigned `#[inline(always)]`/`#[inline(never)]` wrapper functions |
| `drop_order_sensitive` | `templates/drop_order.py` | A randomly nested tree of scopes/structs/tuples/early explicit `drop()`s - optimizations reordering or eliding `Drop` side effects relative to the Reference's guaranteed order (reverse-declaration for locals, forward-declaration for struct/tuple fields; sibling of the #156008 "AsyncDrop broke normal Drop" family) |
| `simd_autovec_loops` | `templates/simd_autovec.py` | Fixed-length `[T; N]` elementwise loops (`wrapping_add`, `^`, `saturating_add`, a branchless `if a[i] > b[i]` select) plus a `wrapping_add` horizontal reduction, at `N` chosen to trigger LLVM's loop vectorizer at `O3` but never at `O0` - a vectorizer correctness bug (wrong lane shuffle, wrong predication/blend, a reassociated reduction that isn't actually equivalent) shows up as `O0 != O3` directly |
| `niche_generic_layout` | `templates/niche_generics.py` | Niche-filling layout correctness when the niche-bearing payload is a *generic* type parameter (`Holder<NonZeroU32>`, `Holder<Holder<NonZeroU16>>`, `Option<Holder<&'static i64>>`, `Either<NonZeroU8, bool>`) rather than a concrete type written directly in the enum body - a layout bug here could be specific to a monomorphized instantiation, not visible from the generic definition alone |
| `cell_shared_ref_aliasing` | `templates/cell_aliasing.py` | Whether LLVM ever applies `noalias`/invariant-load reasoning to a `&Holder` parameter it isn't allowed to, because `Holder` contains a `Cell<T>` field - two copies of the *same* `&Holder` (`h`/`h2`, both `Copy`s of one reference) alternate `.get()`/`.set()` calls through a chain of `#[inline(always)]`/`#[inline(never)]` helper functions, so a correct compiler must treat every read after any `.set()` (through either name) as potentially changed regardless of inlining |
| `bit_intrinsics_chain` | `templates/bit_intrinsics.py` | LLVM's bit-twiddling combine/known-bits lowering for `rotate_left`/`rotate_right`/`swap_bytes`/`reverse_bits`/`count_ones`/`leading_zeros`/`trailing_zeros` (the `fshl`/`fshr`/`bswap`/`bitreverse`/`ctpop`/`ctlz`/`cttz` intrinsics), chained through randomly-inlined helper functions and interleaved with `wrapping_add`/`^` - including rotate amounts well past the type's own bit width to exercise the documented modulo-bit-width wraparound behavior |

## Is the detector actually detecting anything? (`canary/`)

Many rounds in a row with "0 divergences" is the *expected* honest outcome (real rustc
miscompilations are rare), but it's indistinguishable from "the detector is silently broken"
without independent proof. `canary/` holds known, already-confirmed-reproducing MCVEs from
`../_reference/known_miscompile_issues.md` planted deliberately so `./check_canaries.sh` can
prove `diff_runner.py`'s comparison logic genuinely fires on a real divergence:

```bash
./check_canaries.sh                         # PASS verified 2026-10-07 against stable (1.99.0)
./check_canaries.sh --rustc "rustc +nightly" # PASS verified 2026-10-07 against nightly too
```

Unlike every other use of this tool, a divergence in `canary/` is success and a clean run is
failure - see `canary/README.md`. Run this whenever a long clean streak feels suspicious, or after
changing `diff_runner.py`'s comparison logic.

**Toolchain coverage gap found 2026-10-07**: `ROUNDS.log` shows nearly all large-batch rounds
(the 25k-50k-program ones) used `diff_runner.py`'s plain `--rustc` default, which resolves to
whatever `rustup default` is on this host - `stable` (1.99.0), not `nightly`. The `case_163311`
canary happens to reproduce on both, so this specific bug wouldn't have been missed by that choice
- but several of the highest-value open-issue clusters this generator targets (GVN/
ReferencePropagation fixes, e.g. #163777/#163782/#163825, all filed within the last few days as of
2026-10-07) are exactly the kind of bug that lands on nightly weeks before reaching stable. A large
batch run with `--rustc "rustc +nightly"` has not yet been done (only small 60-200-program
stable-vs-nightly self-check rounds so far) - worth prioritizing for the next big round rather than
repeating another large stable-only batch.

## Running a campaign

**Preferred: `./run_round.sh`** (clean corpus -> generate -> diff, one command). Use this instead
of typing your own `rm -rf corpus/*`-style cleanup before regenerating: a literal `rm -rf <dir>/*`
written directly into a shell command gets flagged as a "dangerous rm on a statically-unresolvable
target" and prompts for manual approval EVERY round, which makes an unattended multi-round
campaign require someone to sit and click approve repeatedly. `run_round.sh`'s own cleanup only
ever targets its own fixed, script-relative `corpus/`/`corpus_static/`/`findings/`/`findings_static/`
(anything else passed to `--corpus-dir`/`--findings-dir` is refused), so approving the script once
covers every future round.

```bash
# Random/seeded mode (default):
./run_round.sh --count 500 --seed "$(date +%s)"
./run_round.sh --count 100 --seed 1 --categories union_ref_fields,packed_struct_fields

# Static/combinatorial mode (regenerates the full declared matrix - no --count/--seed):
./run_round.sh --static
./run_round.sh --static --categories union_ref_fields_static

# Anything after run_round.sh's own flags passes straight through to diff_runner.py:
./run_round.sh --count 200 --seed 1 --rustc "rustc +nightly"
./run_round.sh --count 200 --seed 1 --configs "O0=-C opt-level=0" "O3=-C opt-level=3"

# Batching + noise injection (see "Batching for throughput" / "Noise injection" below) -
# --cases-per-file/--noise-level are forwarded to generator.py/static_generator.py:
./run_round.sh --count 5000 --seed "$(date +%s)" --cases-per-file 100 --noise-level 2
./run_round.sh --static --cases-per-file 15 --noise-level 2

# Identity-chain categories (see "A second generation paradigm" above) - --depth/--breadth are
# forwarded to generator.py only (random mode only; identity_chain_unsafe is opt-in, so name it
# explicitly via --categories to include it):
./run_round.sh --count 400 --seed 1 --categories identity_chain_safe --depth 10 --breadth 4
./run_round.sh --count 200 --seed 1 --categories identity_chain_safe,identity_chain_unsafe --depth 8 --breadth 3
```

Exit code is 1 if `diff_runner.py` found a real divergence (written under `findings/<program>/`
or `findings_static/<program>/`), 0 otherwise.

### Manual/low-level invocation (no cleanup - appends/overwrites by filename)

```bash
# From miscompile-fuzzer/:

# 1. Generate N programs (deterministic given --seed; re-run with a new
#    --seed, or a higher --start-index into the same corpus dir, for a fresh
#    batch without colliding file names).
python3 generator.py --count 500 --seed "$(date +%s)" --out corpus/

# 2. Diff every program under the default O0-vs-O3 config, in parallel.
python3 diff_runner.py --corpus corpus/ --findings findings/ --jobs "$(nproc)"

# Exit code is 1 if any real divergence was found (and written under
# findings/<program>/), 0 otherwise.
```

Prefer `run_round.sh` for a real campaign round instead - this form doesn't clean up first, so
stale files from a previous round's different category mix can linger.

Useful variations:

```bash
# Only specific categories
python3 generator.py --count 100 --seed 1 --out corpus/ --categories union_ref_fields,packed_struct_fields

# Different config axis: stable vs nightly (no unstable features are ever
# used by any template, so this must also agree)
python3 diff_runner.py --corpus corpus/ --findings findings/ --rustc "rustc" \
    --configs "stable=-C opt-level=3"
# then separately with --rustc "rustc +nightly" and diff the two findings/
# directories' stdout.*.txt files - or extend diff_runner.py to take two
# --rustc values directly (see "Extending" below) if you do this often.

# Custom config set (e.g. default vs -Z mir-opt-level=0 on nightly)
python3 diff_runner.py --corpus corpus/ --findings findings/ --rustc "rustc +nightly" \
    --configs "default=-C opt-level=3" "mir0=-C opt-level=3 -Z mir-opt-level=0"

# List available template categories
python3 generator.py --list-categories
```

If `diff_runner.py` reports any `self-check mismatches`, that's a bug in a
template's Python oracle, not evidence of a compiler bug - rerun with
`--report-self-check-failures`, find the offending template in `templates/`,
and fix its expected-value computation to match what the Rust code it emits
actually does. Do not file these.

If it reports a real `DIVERGENCE`, go to `findings/<program>/REPORT.md`: it
has every config's exact compile command, stderr, stdout, and exit code,
plus the generator's predicted stdout. Before treating it as a candidate:

1. Re-run the exact compile commands from `REPORT.md` by hand to confirm it's
   not a one-off (timeout, flaky filesystem, etc).
2. Re-read the originating template's UB-freedom argument against the actual
   generated `.rs` file (not just the template source) - double check the
   specific random values/branches chosen didn't accidentally violate an
   invariant the template assumed (e.g. two "distinct" indices turning out
   equal due to a template bug, two union field writes racing in a way the
   template didn't anticipate). If it holds up, it's a real new finding -
   follow the candidate workflow in `../README.md` (bisect with
   `cargo-bisect-rustc`, write `RESULT.md`, etc. - never a `patch.diff`).

## Batching for throughput

Each generated `.rs` file used to test exactly one scenario, so most
wall-clock time in a large campaign went to rustc process-startup overhead,
not actual compilation work. `--cases-per-file N` (both `generator.py` and
`static_generator.py`) packs `N` independent sub-cases into ONE `.rs` file
instead: each sub-case is generated exactly the way it always was (same
template, same per-category parameter generation - a batched sub-case's
underlying `(category, Program)` is derived identically to what a
non-batched run at the same `(--seed, index)` would produce), then wrapped in
its own `mod case_<n> { ... }` and called in sequence from one shared
`main()`. `mod` isolation means no category template needs to change for
this to be safe, even the handful that reuse a fixed (non-`idx`-qualified)
name like `cell_aliasing.py`'s `struct Holder` - two sub-cases of the same
category in one file would otherwise collide.

`--cases-per-file 1` (the default) reproduces today's one-program-per-file
output byte-for-byte - nothing existing changes unless you pass a higher
value (or `--noise-level` > 0, see below).

**Locating a divergence inside a batch:** every sub-case's `println!(...)`
calls get a `print!("case_<n>:")` spliced in front of them (a verbatim text
substitution, not a reinterpretation of the format string), and the matching
`.expected` lines get the identical `case_<n>:` prefix. Since sub-cases run
strictly in sequence, every line of stdout is self-identifying. On top of
that, `diff_runner.py` now writes a plain `difflib.unified_diff` between the
baseline config's stdout and every other config's stdout into
`findings/<program>/REPORT.md` (see "## stdout diff" there) - for a batched
file this immediately shows which `case_<n>:` line actually differs, with no
need to re-derive it by hand.

### Measured throughput (this session, 24-thread host, `rustc 1.99.0` O0 vs O3)

5000 random-mode sub-cases end to end (`generator.py` + `diff_runner.py`,
`--noise-level 0` both runs, `--jobs 24`):

| `--cases-per-file` | rustc invocations (2 configs) | wall-clock (`diff_runner.py`) | sub-cases/sec |
|---|---|---|---|
| 1 (today's default) | 10000 | 51.4s | ~97 |
| 50 | 200 | 4.5s | ~1103 |
| 100 | 100 | 4.0s | ~1247 |

That's roughly a **12.8x** wall-clock speedup at `--cases-per-file 100` over
one-case-per-file, for identical total test coverage (same 5000 sub-cases,
same templates, same per-category parameter generation - only how many share
a rustc invocation changed). Generation itself (`generator.py` alone) is not
the bottleneck either way (well under a second for 5000 sub-cases); the win
is entirely in `diff_runner.py`'s compile+run phase, as expected since that's
where rustc's own process-startup cost is paid once per file instead of once
per sub-case.

## Noise injection

A real rustc miscompile class only triggers when unrelated-looking code sits
near the actual trigger - this matches how several LLVM/MIR optimization
passes gate on function size, instruction count, or inlining-budget
heuristics, so padding a function's neighborhood with inert filler can flip
which optimization path gets taken. `--noise-level {0,1,2}` (both
`generator.py` and `static_generator.py`, combinable with `--cases-per-file`
- noise applies per sub-case within a batched file too) injects this filler
via the shared `templates/noise.py` module, reused unchanged by both
`templates/` and `templates_static/`:

- **0** (default): no noise, today's behavior, byte-for-byte.
- **1** (light): a couple of small unrelated helper functions, one extra
  struct/enum type, a couple of tautological `assert!`/`debug_assert!`
  calls, a couple of dead-but-`black_box`ed computations.
- **2** (heavy): more and larger helper functions - including a couple of
  "big" ones with 60-120 chained sequential wrapping/bitwise operations,
  deliberately sized to inflate that function's own emitted instruction
  count (aiming at, not guaranteeing - this is a version-independent
  heuristic, not a promise tied to any specific rustc/LLVM inlining-cost
  constant - crossing a typical inlining-budget threshold), several extra
  types, more tautologies, more dead calls.

Noise is spliced in two places: extra top-level items (functions,
structs/enums) land among the sub-case's own top-level items (always valid -
Rust item order is irrelevant), and two statement groups (`pre`/`post`) are
spliced immediately before and after the real test logic, which is itself
wrapped in its own nested `{ }` block so the splice point is always
syntactically valid regardless of whether the real logic's last statement
happens to be a tail expression.

### The inertness invariant (critical - stated here and in `templates/noise.py`)

Noise must be **provably inert**, so that any divergence a diff run finds is
always attributable to the real tested pattern, never to the noise:

1. **No shared state.** Noise never reads or writes any variable, static, or
   field that the real test logic reads or writes - it only ever touches its
   own freshly-declared locals and the extra types/functions it declared
   itself.
2. **Fully deterministic, baked in at generation time.** Every "random"
   choice a noise helper makes happens in Python, via the same
   `random.Random` seeding convention the rest of this tool already uses
   (an independently-seeded stream per sub-case - `noise:<seed>:<index>` in
   random mode, `static-noise:<category>:<idx>` in static mode - so noise
   draws can never perturb, or be perturbed by, the real template's own RNG
   stream). The emitted Rust source contains only literal values; nothing it
   emits calls an RNG, reads the clock, or reads an address at *program*
   runtime.
3. **Never printed, never compared.** Every noise value is threaded through
   `std::hint::black_box(..)` and then discarded (`let _ = ...`) - it can
   still be real work for the optimizer to reason about without ever
   becoming part of a program's observable stdout.

If you extend `templates/noise.py`, any new noise shape must keep all three
properties - see the module's docstring for the same statement next to the
code it governs.

## Extending with a new template category

1. Create `templates/<name>.py` with a `generate(rng, idx) -> Program`
   function, decorated `@register("category_name")` (see any existing module
   for the shape - `Program` is defined in `templates/common.py`).
2. Write a module docstring stating: (a) which MIR/LLVM pass or bug family
   this targets, and (b) the specific UB-freedom argument for why every
   program this generates is guaranteed well-defined Rust, no matter what the
   RNG picks.
3. Add the module to the import list at the bottom of `templates/__init__.py`.
4. Generate a batch with just your new category
   (`--categories your_category_name`) and diff it. Fix any self-check
   mismatches until it's clean, the same way the existing categories were
   validated (see below) - only then fold it into the full rotation.

## Static mode: categories, usage, and extending

Four categories so far, covering the two biggest open-issue clusters
(unions/packed-struct GVN bugs, pointer provenance) plus two more from the
random generator's set (reborrow chains, which are pure-safe-Rust and
especially amenable to an exhaustive small matrix). Each one's combination
count is exactly the product of its declared axis lengths - see the
`SCENARIO_AXES` dict at the top of each module, this IS the entire spec:

| Category | File | Axes (combination count) |
|---|---|---|
| `union_ref_fields_static` | `templates_static/unions_static.py` | `n_vals` (3) × `sequence` (4) × `start_field` (2) = 24 |
| `packed_struct_fields_static` | `templates_static/packed_structs_static.py` | `field_types` (3) × `cond1` (2) × `cond2` (2) × `mut_field_offset` (2) = 24 |
| `pointer_provenance_roundtrip_static` | `templates_static/provenance_static.py` | `elem_type` (2) × `n` (2) × `index_pattern` (2) = 8 |
| `reborrow_chain_static` | `templates_static/reborrow_static.py` | `depth` (3) × `leaf_type` (2) = 6 |

62 programs total today. Field values, deltas, and tags inside each scenario
are computed by a fixed formula from `idx` (e.g. `wrap(1000 + 37*i + idx*7,
field_type)`) rather than sampled - deterministic but still varied across the
declared index space, so two different scenarios don't happen to generate
byte-identical programs.

```bash
# Generate the full static corpus (always the same 62 programs, every time):
python3 static_generator.py --out corpus_static/

# Diff it - same diff_runner.py as random mode, just pointed at the other dirs:
python3 diff_runner.py --corpus corpus_static/ --findings findings_static/ --jobs "$(nproc)"

# Only one category:
python3 static_generator.py --out corpus_static/ --categories union_ref_fields_static

# List available static categories:
python3 static_generator.py --list-categories
```

### Extending a static category

Add one value to an axis list in the relevant `templates_static/<name>.py`
module - nothing else needs to change. Verified directly this session:
adding `12` to `unions_static.py`'s `n_vals` axis (`[4, 6, 8]` →
`[4, 6, 8, 12]`) took that category from 24 to 32 programs on the next
`static_generator.py` run (exactly `+1 n_vals × 4 sequences × 2
start_fields = +8`, as the combination-count formula predicts), diffed clean,
and was reverted afterwards (that value isn't part of the committed matrix).

To add a whole new static category: same shape as `templates/`'s "Extending"
section above, but write a `generate_all() -> list[Program]` (no `rng`
argument) decorated `@register_static("category_name")`, declare its
`SCENARIO_AXES` dict, and add the module to `templates_static/__init__.py`'s
import list.

### Validation result (static mode, this session)

Ran the full 62-program static corpus against three independent config
pairs - the default `O0` vs `O3`, a narrower `-C opt-level=1` vs `=2`, and
`rustc` (stable) vs `rustc +nightly` both at `-C opt-level=3` - all three
clean on the first attempt, no template fixes needed:

```
diffing 62 programs across configs ['O0', 'O3'] with 24 workers...
done. 62 programs, 0 failed to compile (same on all configs),
0 self-check mismatches (generator-oracle bugs, not findings),
0 real cross-config divergences.
```

## Validation result (this session)

Ran `generator.py --count 180 --seed 1` (20 programs per category across all
9 categories) followed by `diff_runner.py` at the default O0-vs-O3
configuration. Also separately re-validated a 200-program batch at a
different seed, a 30-program subset at `-C opt-level=2`, and the same
30-program subset compiled with `rustc` (stable 1.99.0) vs `rustc +nightly`
(1.101.0-nightly) at `-C opt-level=3` - all clean.

First pass surfaced real harness/template bugs, all fixed before the clean
run below (not real compiler divergences - each was root-caused as a
generator mistake):

- **10/90 compile failures** in `nested_loop_mutable_state`: a literal like
  `1912179534.wrapping_mul(...)` is syntactically ambiguous to rustc (it
  can't tell the literal's type without a suffix). Fixed by adding an
  explicit type suffix/parens: `(1912179534{ty}).wrapping_mul(...)`.
- **3/90 self-check mismatches** in `packed_struct_fields`: the Python oracle
  computed which struct field got mutated based on which *variable name*
  (`mutate_branch`/`dead_branch`) held the code, but the template then always
  placed `mutate_branch`'s text in the `if` slot and `dead_branch`'s in the
  `else` slot regardless of which one `cond2` would actually execute - so for
  `cond2 == false` the oracle predicted the wrong field's update. Fixed by
  computing the executed branch directly from `cond2` instead of threading it
  through two confusingly-named variables.
- **10/90 self-check mismatches** in `static_merge_packed_const`: the
  template asserted `&C1 != &C2` for two distinct `const` items with
  identical values. That's not a guaranteed Rust invariant - only `static`
  items (which the same template already tests separately) are guaranteed
  distinct addresses; `const` items may legally share backing storage. Fixed
  by removing that assertion (kept only the correct `static`-address checks
  and the const field-value reads, which remain exactly predictable).

Final clean run:

```
diffing 180 programs across configs ['O0', 'O3'] with 24 workers...

done. 180 programs, 0 failed to compile (same on all configs),
0 self-check mismatches (generator-oracle bugs, not findings),
0 real cross-config divergences.
```

## Validation result (round 3 - 4 new categories)

After two full rounds with no new signal, added 4 genuinely new template
categories (`const_eval_array_ops`, `trait_object_devirt`,
`closure_capture_boundary`, `drop_order_sensitive` - see the table above),
chosen to cover pattern families absent from the existing 9: const-eval of a
looping/branching `const fn`, default-vs-override trait dispatch through
concrete/generic/`dyn`/boxed-collection call sites, closure capture-by-move
vs capture-by-reference across inlining boundaries, and drop-order guarantees
across nested scopes/structs/tuples. Each was spot-checked by hand against
real `rustc` output before writing its Python oracle (e.g. empirically
confirmed struct/tuple-field forward drop order vs. local reverse drop order
with a throwaway program - see `../tmp_recon/` - rather than relying on
memory of the Reference text).

Ran the new categories alone (40 programs, seed 1; 200 programs, seed 777;
both at the default O0-vs-O3 config), then at a 3-way `-C opt-level=1` vs `=2`
vs `=3` config (80 programs, seed 555), then that same 80-program batch's
self-check alone against `rustc +nightly -C opt-level=3`, and finally the
full 13-category rotation together (260 programs, seed 42, default config) to
confirm the new imports didn't disturb the existing categories. All four
passes were clean on the first attempt - no self-check mismatches, no
compile failures, no cross-config divergences - so no template fixes were
needed this round:

```
done. 200 programs, 0 failed to compile (same on all configs),
0 self-check mismatches (generator-oracle bugs, not findings),
0 real cross-config divergences.
```

```
done. 260 programs, 0 failed to compile (same on all configs),
0 self-check mismatches (generator-oracle bugs, not findings),
0 real cross-config divergences.
```

## Validation result (round 4 - 2 new categories)

After round 3's 13 categories went through several large unattended batches
(see `ROUNDS.log`: 40000, 25000, and 8000 programs, all 0 divergences) with no
new signal, added 2 more genuinely new template categories chosen to cover
pattern families absent from all 13 existing ones (checked against
`../_reference/known_miscompile_issues.md`'s pattern column):

- `simd_autovec_loops` (`templates/simd_autovec.py`) - fixed-length array
  elementwise loops shaped to trigger LLVM's loop vectorizer at `O3` (but
  never at `O0`), covering the loop-vectorizer/SLP correctness surface that
  no existing category touched (the closest existing categories,
  `checked_wrapping_arithmetic` and `const_eval_array_ops`, use scalar
  control flow, not fixed-trip-count array loops at a vectorizable width).
- `niche_generic_layout` (`templates/niche_generics.py`) - niche-filling
  enum layout where the niche-bearing type is a *generic parameter*
  (`Holder<NonZeroU32>`, nested `Holder<Holder<T>>`, `Option<Holder<&i64>>`,
  a two-parameter `Either<A, B>`), distinct from the existing
  `enum_niche_layout` category which only ever matches on concrete types
  written directly into the enum body, never a monomorphized type
  parameter.

Both pass the same UB-freedom bar as every other category (see their module
docstrings for the specific argument - safe constructors only, no
transmutes, every reference a `'static` borrow, every array index a literal
`0..N` loop bound, all arithmetic through `wrapping_*`/`saturating_*`).

Ran the two new categories alone (60 programs, seed 1; 400 programs, seed
777; both at the default O0-vs-O3 config), then a 3-way `-C opt-level=1` vs
`=2` vs `=3` config (200 programs, seed 555), then a 60-program batch's
self-check alone against both `rustc` (stable 1.99.0) and `rustc +nightly`
each at `-C opt-level=3`, and finally the full 15-category rotation together
(300 programs, seed 42, default config) to confirm the new imports didn't
disturb the existing 13. All passes were clean on the first attempt - no
self-check mismatches, no compile failures, no cross-config divergences - so
no template fixes were needed this round:

```
done. 400 programs, 0 failed to compile (same on all configs),
0 self-check mismatches (generator-oracle bugs, not findings),
0 real cross-config divergences.
```

```
done. 300 programs, 0 failed to compile (same on all configs),
0 self-check mismatches (generator-oracle bugs, not findings),
0 real cross-config divergences.
```

## Validation result (round 7 - 2 new categories)

After round 6's 15 categories went through large unattended batches (see
`ROUNDS.log`: 40000 programs clean at O0-vs-O3, plus a 3000-program
`codegen-units=1` vs `=16` pass) with no new signal, added 2 more genuinely
new template categories chosen to cover pattern families absent from all 15
existing ones (checked against `../_reference/known_miscompile_issues.md`'s
pattern column; the areas the task brief suggested revisiting - const-eval,
SIMD loops, niche-filling generics, trait-object devirtualization, closure
capture boundaries, drop order - were already each covered by an existing
category, so these two instead target surfaces none of the 15 touched):

- `cell_shared_ref_aliasing` (`templates/cell_aliasing.py`) - shared-reference
  interior-mutability aliasing via `Cell<T>`: two `Copy`s of the same
  `&Holder` alternate `.get()`/`.set()` through randomly-inlined helper
  functions, probing whether the compiler ever treats a `&T` containing an
  `UnsafeCell` as `noalias`/invariant across a call it isn't allowed to.
  Distinct from `unions.py`/`packed_structs.py` (which target a *write*
  through a union/packed sibling field being lost) and from `reborrow.py`
  (which targets `&mut` reborrow-chain collapsing) - this category never
  creates an `&mut` or an independent second allocation at all, only two
  names for the one already-shared reference.
- `bit_intrinsics_chain` (`templates/bit_intrinsics.py`) - `rotate_left`/
  `rotate_right`/`swap_bytes`/`reverse_bits`/`count_ones`/`leading_zeros`/
  `trailing_zeros` chained through randomly-inlined helper functions and
  interleaved with `wrapping_add`/`^`, including rotate amounts well past the
  type's own bit width (exercising the documented modulo-bit-width
  wraparound). Targets the `fshl`/`fshr`/`bswap`/`bitreverse`/`ctpop`/`ctlz`/
  `cttz` intrinsic lowering/combine surface, which no existing category
  exercises (`arithmetic.py` only chains plain `wrapping_*`/`checked_*`/
  `saturating_*`, no bit permutation).

Both pass the same UB-freedom bar as every other category (see their module
docstrings): `Cell::get`/`set` are always safe and total, the bit-intrinsic
methods are documented as panic-free for every input (including the
past-bit-width rotate amount and the all-zero `leading_zeros`/
`trailing_zeros` case), no `unsafe`, no raw pointers, no `&mut` ever created.

Ran the two new categories alone (20 programs, seed 1, first smoke test; 400
programs, seed 777, larger batch; both at the default O0-vs-O3 config), then
a 3-way `-C opt-level=1` vs `=2` vs `=3` config (200 programs, seed 555), then
a 60-program batch's self-check against `rustc +nightly -C opt-level=3`
followed by the same corpus against stable `rustc -C opt-level=3`, and
finally the full 17-category rotation together (340 programs, seed 42,
default config) to confirm the new imports didn't disturb the existing 15.
All passes were clean on the first attempt - no self-check mismatches, no
compile failures, no cross-config divergences - so no template fixes were
needed this round:

```
done. 400 programs, 0 failed to compile (same on all configs),
0 self-check mismatches (generator-oracle bugs, not findings),
0 real cross-config divergences.
```

```
done. 340 programs, 0 failed to compile (same on all configs),
0 self-check mismatches (generator-oracle bugs, not findings),
0 real cross-config divergences.
```

## Validation result (batching + noise injection)

Added `--cases-per-file`/`--noise-level` (`templates/batching.py`,
`templates/noise.py` - see "Batching for throughput"/"Noise injection"
above). Validation order, per the usual rule for a `diff_runner.py` change -
prove the detector itself first, then prove the new generator output is
clean:

1. `./check_canaries.sh` - still PASS (unchanged; `diff_runner.py`'s new
   stdout-diff section in `REPORT.md` is additive and didn't touch the
   divergence-detection logic itself).
2. Hand-verified the stdout-diff attribution end to end: fed `write_finding`
   a synthetic 3-sub-case batched `ProgramResult` where only `case_1`'s line
   differed between configs - the `REPORT.md` diff section correctly
   isolated `-case_1:2` / `+case_1:999` with `case_0`/`case_2` unchanged.
3. Random mode, `--noise-level 2`, batched: 3400 sub-cases (200/category x 17
   categories, seed `20261007`, `--cases-per-file 68`) - 50 files, all
   compiled, all ran, **0 self-check mismatches, 0 divergences**.
4. Static mode, `--noise-level 2`, batched: the full 62-program declared
   matrix at `--cases-per-file 12` - 6 files, all compiled, all ran, **0
   self-check mismatches, 0 divergences**.
5. Every intermediate smoke test along the way (single-case noise level 1/2,
   a 20-sub-case batch, a 10-category-mixed batch, the static corpus at
   `--cases-per-file 1`) was individually compiled and its stdout diffed
   against its `.expected` by hand before moving on to the larger runs above.

```
done. 50 programs, 0 failed to compile (same on all configs),
0 self-check mismatches (generator-oracle bugs, not findings),
0 real cross-config divergences.
```

Throughput measurement (5000 random-mode sub-cases, `--noise-level 0` so the
comparison isolates batching's own effect, `rustc 1.99.0`, 24 threads;
`generator.py`'s own run time is sub-second either way and excluded - the
numbers below are `diff_runner.py`'s compile+run wall-clock only):

| `--cases-per-file` | files (= rustc invocations per config) | wall-clock | sub-cases/sec |
|---|---|---|---|
| 1 | 5000 | 51.4s | ~97 |
| 50 | 100 | 4.5s | ~1103 |
| 100 | 50 | 4.0s | ~1247 |

~12.8x at `--cases-per-file 100` vs. today's one-case-per-file default, for
identical total coverage.

## Validation result (round 20 - identity-chain paradigm)

Added the identity-chain paradigm (`identity_chain_safe` +
`identity_chain_unsafe` - see "A second generation paradigm" above), 8
safe transforms + 2 unsafe-extension patterns, `--depth`/`--breadth`
(defaults 8/3, forwarded through `run_round.sh` too). Validation order
followed the same discipline as every prior paradigm-level change here
("prove the detector first, then prove the new generator output is clean"),
but with an extra step given how load-bearing the "trivial oracle" claim is
for this paradigm specifically - any self-check mismatch here means a
transform in the library isn't actually an identity, not a Python-mirror
bug, so it was worth independently confirming outside `diff_runner.py` too,
not just trusting the harness's own self-check pass:

1. `./check_canaries.sh` - still PASS (unchanged; this round never touched
   `diff_runner.py`).
2. `identity_chain_safe` alone, 20 programs (seed 1, module defaults
   depth=8/breadth=3): every program individually compiled and run by hand
   at both `-C opt-level=0` and `-O`, stdout diffed against `.expected`
   outside `diff_runner.py` - clean.
3. `identity_chain_safe` alone, 200 programs (seed 777, depth=10/breadth=4,
   deliberately higher than the defaults to stress the code-size safety
   valve) - same hand-compile-and-diff treatment, exercising all three
   composite kinds (tuple/array/struct) and every scalar type at least
   once - clean.
4. `identity_chain_safe` alone via `run_round.sh`, 400 sub-cases (20 files,
   seed `20261007`, `--cases-per-file 20 --noise-level 1`, O0 vs O3) - **0
   self-check mismatches, 0 divergences**.
5. `identity_chain_unsafe` alone via `run_round.sh`, 100 sub-cases (10
   files, seed 555, depth=9/breadth=3, `--cases-per-file 10 --noise-level
   2`) - exercises both the pointer-round-trip transform and the union
   byte-reinterpretation final step - **0 self-check mismatches, 0
   divergences**.
6. Full 18-category default rotation (`identity_chain_safe` + the 17
   pre-existing categories, `identity_chain_unsafe` correctly excluded),
   760 sub-cases (38 files, seed 909090) - confirms the new category didn't
   disturb the existing rotation and that opt-in exclusion actually works -
   **0 self-check mismatches, 0 divergences**.
7. `identity_chain_safe` + `identity_chain_unsafe` explicitly together,
   3000 sub-cases (50 files, seed 13579, depth=9/breadth=4,
   `--cases-per-file 60 --noise-level 2`) - 9.5s wall - **0 self-check
   mismatches, 0 divergences**.
8. Same two categories at a 3-way `-C opt-level=1` vs `=2` vs `=3` config
   (300 sub-cases, seed 2468) - **0 self-check mismatches, 0 divergences**.
9. Same two categories, single-config self-check against `rustc +nightly`
   (150 sub-cases, seed 86420) - confirms the trivial oracle holds on
   nightly too, not just stable.

```
done. 38 programs, 0 failed to compile (same on all configs),
0 self-check mismatches (generator-oracle bugs, not findings),
0 real cross-config divergences.
```

See `ROUNDS.log` for the full per-pass record (round 20 entries).

### Survey: icemaker (github.com/matthiaskrgr/icemaker)

Per the task brief's suggestion, briefly surveyed icemaker rather than
building against it. It's a crash/ICE-corpus fuzzer: it runs rustc (and
clippy/rustdoc/rustfmt/miri) across many flag combinations over a large
collection of EXISTING Rust source files - rustc's own test suite and
"glacier" (a corpus of already-reduced, already-filed ICE repros) - looking
for a crash, then attempts automatic reduction of whatever crashes. It does
not do differential/miscompile testing (no "does this program's observable
behavior agree across configs" comparison at all - a crash is its only
signal), so its own detection logic doesn't transfer here. Its input corpus
doesn't obviously transfer either: both rustc's test suite and glacier skew
heavily toward code that's deliberately unusual, invalid, or already known
to crash something, not the "compiles cleanly, runs to completion, prints a
deterministic value" shape every program in this tool's corpus needs - most
files in either corpus would need nontrivial filtering/adaptation (checking
they compile and run without panicking at all, picking out ones with
`println!`-style observable output, stripping deliberately-invalid ones)
before they could seed anything here. Given the shallow 5-10 minute budget
for this survey, nothing looked clearly and trivially reusable, so no
integration was built - noted here for a future session that wants to
revisit it with a larger time budget.

### Future: automatic minimization with treereduce-rust

Not implemented this round - there is nothing to reduce yet. Noting it here
for whichever future round first confirms a real new finding: today's
candidate-triage workflow (`../README.md`) shrinks a repro by hand.
[`treereduce-rust`](https://github.com/langston-barrett/treereduce) (a
C-Reduce-style, syntax-aware reducer for Rust, used for e.g.
rust-lang/rust#163457's reduction) could automate that step once a finding
is confirmed real - feeding it the failing `.rs` file plus a small
"does this still reproduce the divergence" check script (re-running the
same two compile configs and comparing stdout, the same comparison
`diff_runner.py` already does for one program) and letting it shrink the
file automatically instead of by hand. This is deliberately just a pointer:
don't install or wire it up without a confirmed finding to reduce first.

## Files

- `generator.py` - CLI entry point for generating the random corpus.
- `static_generator.py` - CLI entry point for generating the static (deterministic/combinatorial) corpus.
- `diff_runner.py` - CLI entry point for differential testing a corpus - shared by both modes (`--corpus`/`--findings` just point at a different pair of directories).
- `templates/common.py` - shared RNG/type helpers, the `Program` dataclass, used by both modes.
- `templates/batching.py` - packs multiple sub-cases' `Program`s into one batched `.rs` file (`--cases-per-file`), shared by both modes - see "Batching for throughput".
- `templates/noise.py` - provably-inert noise injection (`--noise-level`), shared by both modes - see "Noise injection".
- `templates/identity_chain.py` - the `identity_chain_safe` category and its identity-transform library/chain-building engine (`build_chain`, `ChainCtx`) - see "A second generation paradigm".
- `templates/identity_chain_unsafe.py` - the opt-in `identity_chain_unsafe` extension (pointer round-trip + union byte-reinterpretation), built on top of `identity_chain.py`.
- `templates/*.py` - one file per random-mode template category (see table above).
- `templates_static/common.py` - the `product_scenarios()` cartesian-product helper.
- `templates_static/*.py` - one file per static-mode template category, each declaring its own `SCENARIO_AXES`.
- `corpus/`, `findings/`, `corpus_static/`, `findings_static/` - gitignored scratch output (regenerate anytime).
