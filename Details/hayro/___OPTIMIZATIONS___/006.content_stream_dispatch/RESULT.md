# Content-stream operator dispatch (`hayro-syntax`'s `TypedInstruction::dispatch`)

**No patch attempted against real source.** The reordering below was tested against a
standalone copy of the match in `bench/dispatch_order/`, never applied to
`hayro-syntax/src/content/ops_generated.rs` itself, since a diff of the actual source file was
never produced or needed to get the answer (see "Result").

Closes out a `// TODO: Explore whether dispatching can be made more efficient.` comment present
in the source (`hayro-syntax/src/content/mod.rs:334`, right above the call site of
`TypedInstruction::dispatch` in `hayro-syntax/src/content/ops_generated.rs:451`).

Part of "content-stream dispatch overhead" from `../README.md` (24-29% of `render()`'s own time,
of which tokenizing/parsing the content stream into typed instructions was estimated at ~8.8%).
`dispatch()` is called once per operator from `TypedIter::next()`
(`hayro-syntax/src/content/mod.rs:332-337`) and matches the raw operator byte string (e.g.
`b"Tf"`, `b"cm"`) against 73 possible operators to build the corresponding `TypedInstruction`
variant.

## What was tried

### Step 1: what does `img.pdf` actually call, and how often?

`bench/op_freq/` (standalone project, path dep on `hayro-syntax`, embeds `img.pdf` via
`include_bytes!`) loads the real page and counts every operator:

```
cd bench/op_freq
cargo run --release
```

```
total operators: 4781
op        count      pct
Q           468    9.79%
q           468    9.79%
cm          465    9.73%
l           418    8.74%
Tf          326    6.82%
Tr          326    6.82%
BT          326    6.82%
Tm          326    6.82%
ET          326    6.82%
rg          296    6.19%
TJ          259    5.42%
m           144    3.01%
S           124    2.59%
RG          121    2.53%
c            88    1.84%
... (12 more operators, each <1.6%)
```

The top 14 operators cover ~85% of all calls. Notably, `Tf`/`Tr`/`BT`/`Tm`/`ET` alone are 34% of
all operators combined (this is a text-heavy page - 326 glyph-drawing text objects), and in the
current generated match (`ops_generated.rs:451-524`) they sit near arm #50-58 out of 73, while
`q`/`Q`/`cm` (also very frequent) happen to already be arms #3-5. This looked like a plausible
reordering opportunity - worth actually measuring before assuming it helps.

### Step 2: does match-arm order actually matter?

`bench/dispatch_order/` (standalone project, no dependencies) copies the exact 73-arm match from
`ops_generated.rs` verbatim (each arm's real `Foo::from_stack(...)?.into()` work replaced by a
distinct `u8` tag, to isolate pure match-arm cost from operand-parsing cost), in two versions:
- `dispatch_original_order`: arms in the exact order they appear in the real source.
- `dispatch_frequency_order`: same 73 arms, reordered so the operators measured above as most
  frequent in `img.pdf` come first.

Fed the real, order-preserving 4781-operator sequence from `img.pdf` (captured to
`op_sequence.txt` by `bench/op_freq`), `taskset -c 2`, best of 7 runs of 20,000 full passes each:

```
cd bench/dispatch_order
taskset -c 2 cargo run --release
```

| Variant | ns/op | Δ% vs original |
|---|---:|---:|
| `dispatch_original_order` (current code, unchanged) | 2.30-2.42 | - |
| `dispatch_frequency_order` (hot operators moved to front) | 2.45-2.53 | +4% to +10% slower |

## Result

Repeated 3x, consistent direction each time: reordering never helped, and was consistently
slightly slower. This means the match is not compiled as a sequential comparison chain where arm
position matters - rustc/LLVM already lowers a `match` on `&[u8]` byte-string patterns of this
shape into something close to optimal (almost certainly: switch on length, then an integer
compare using a small number of loaded bytes per length group), making frequency-based manual
reordering pure noise-or-worse.

Even in the best case, the entire 4781-operator dispatch match for the whole page took **11.0-11.7
microseconds total** (2.3-2.5ns x 4781 ops). Compare to the ~175 microseconds/page that
`bench/op_freq` measured for the *full* `typed_operations()` pass (tokenizing + reading each
operand off the `Reader` + building every `TypedInstruction`, not just the final dispatch match):
the dispatch match itself is only **~6.5% of that**. The other ~93% is spent reading operands
(numbers/names/arrays via `Reader`) and constructing each `TypedInstruction`'s fields
(`Foo::from_stack`), not in choosing which operator it is.

Relative to a whole-page render (~3.1-10ms depending on the PDF, per `../README.md`), the
dispatch match alone is on the order of **0.1-0.4%** of total time.

## Does it change behavior/output?

Not applicable - no code was changed (the reordering was tested against a standalone copy of the
match, never applied to real source).

## Why the result looks the way it does

The compiler already optimizes byte-string match dispatch well enough that manual operator
reordering by real-world frequency doesn't help - the theoretical asymmetry between "hot" and
"cold" arms doesn't translate into a measurable win at this scale (11 microseconds/page total),
and the dispatch match was never a meaningful fraction of total render time to begin with.

## What's not done / open questions

Did not drill into the ~93% (operand reading / `TypedInstruction` construction) beyond what
`../README.md` already covers - would need per-line instrumentation inside `hayro-syntax`'s
`Reader`, a much larger investigation, and even the outer ~8.8% "tokenizing" bucket this sits
inside is itself a small fraction of total render time.
