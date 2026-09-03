# 75 - Non-power-of-two `chunked_arena::CHUNK_SIZE` (200)

IDEAS.md's backlog (added while closing out candidate 074) pointed out that every value 074
swept (32/64/256/512/1024/4096) is a power of two, where `ChunkedArena::get_mut`'s
`index / CHUNK_SIZE` and `index % CHUNK_SIZE` compile to a shift and a mask, not a real division
instruction - so 074 could not actually confirm or refute candidate 073's own writeup, which
attributed part of its gap versus candidate 071's diagnostic figure to "division/modulo
indexing" cost. This candidate set out to test `CHUNK_SIZE = 200`, a non-power-of-two value,
on the assumption that it would force a real integer division/modulo at every arena access -
disassembly below shows that assumption was itself wrong (see "Why the result looks the way it
does"), which is as informative an answer to IDEAS.md's question as the timing result is.

`chunked_arena::ChunkedArena` does not exist on `master` - candidate 073's own patch was reverted
per this workspace's rule ("the main project's working tree is always left clean"), so
`patch.diff` here is 073's full patch, unmodified (`CHUNK_SIZE = 256`), applied first as the
prerequisite this candidate's one-line edit sits on top of; the edit itself is not shipped as a
separate `patch.diff` since it is a one-line change to a constant already inside 073's own
`patch.diff` (`internal/core/partial_renderer.rs`: `const CHUNK_SIZE: usize = 256;` -> `200`).

## What was tried

Two binaries built (`cargo build --bin perf_test --profile rdebug`,
`internal/core/partial_renderer.rs`'s `const CHUNK_SIZE: usize = <value>;` set to `256` and
`200` respectively), compared pairwise via `tools/paired.sh` (`RUNS=9 FRAMES=20`), same protocol
as candidates 073/074. Scene: `scroll --count 100000 --row-style layout --scroll-step 0` (the
scene both 073 and 074 used for their own primary comparisons).

Each row's pair is (`CHUNK_SIZE = 256` A-side of that `paired.sh` invocation) -> (`CHUNK_SIZE =
200` B-side of the same invocation); the 256 side is re-measured per pairing, not reused, per
this workspace's paired-comparison discipline.

| Variant | scenario | timing | paired verdict |
|---|---|---|---|
| `CHUNK_SIZE = 200` | `layout` static, single-core (`taskset -c 2`), run 1 | median 148.12 -> 149.27ms (+0.8%) | 5/9 B faster - NOT consistent |
| `CHUNK_SIZE = 200` | `layout` static, single-core (`taskset -c 2`), run 2 (repeat) | median 147.57 -> 147.40ms (-0.1%) | 4/9 B faster - NOT consistent |
| `CHUNK_SIZE = 200` | `layout` static, multi-core (unpinned), run 1 | median 145.90 -> 148.77ms (+2.0%) | 3/9 B faster - NOT consistent |
| `CHUNK_SIZE = 200` | `layout` static, multi-core (unpinned), run 2 (repeat) | median 146.72 -> 146.77ms (+0.0%) | 6/9 B faster - NOT consistent |

## Result

No consistent signal in either direction, on either core count, across two repeated runs each
(single-core: 5/9 then 4/9; multi-core: 3/9 then 6/9 - none reach the 8-or-9-of-9 threshold this
workspace's tooling requires to call a paired result real). `CHUNK_SIZE = 200` performs the same,
within noise, as `CHUNK_SIZE = 256` on this scene - but disassembly (see below) shows this is not
because a real division/modulo turned out to be cheap: no division instruction is emitted for
either value in the first place, so this result does not actually test what IDEAS.md's entry
asked for. The premise ("a non-power-of-two chunk size pays a real division/modulo") does not
hold for a compile-time-constant `CHUNK_SIZE` at all, at any value.

## Does it change behavior/output?

- **Bit-exact.** `tools/pixelcheck.sh` (`RUNS=6`), `256` vs. `200`: 248 px noise floor, 223 px
  raw difference, 0 px outside the noise floor on `scroll --row-style layout --scroll-step 0`.
- `cargo test -p i-slint-core --lib` at `CHUNK_SIZE = 200`: 307/307 passed, unchanged from
  candidates 073/074.
- `cargo test -p slint --test partial_renderer` at `CHUNK_SIZE = 200`: 24/24 passed, unchanged.
- **Is this a breaking change?** No - `CHUNK_SIZE` is a private `const` inside a private module
  (`chunked_arena`, itself private to `partial_renderer.rs`), not part of candidate 073's public
  surface (which is unchanged - still none, see 073's own `RESULT.md`).

## Why the result looks the way it does

Disassembly of the `CHUNK_SIZE = 200` binary (`objdump -d --demangle`, functions
`ChunkedArena::insert` and `ChunkedArena::free`, both of which compute `index / CHUNK_SIZE` and
`index % CHUNK_SIZE`) shows **no `div`/`idiv` instruction at either indexing site**. LLVM
compiles the division by the compile-time constant `200` into a magic-number multiply and shift
(`free`: `shr $0x3,%rax` then `movabs $0x28f5c28f5c28f5c3,%rcx; mul %rcx; shr $0x2,%rdx` - the
standard Granlund-Montgomery "division by invariant integers using multiplication" technique;
`imul $0xc8,%rdx,%rax` then a subtract recovers the remainder), exactly as it would for a
power-of-two divisor, just with a different (non-shift) multiplier and an extra `mul`/`shr` pair
instead of a single `shr`. This is a **general LLVM optimization for any divisor known at compile
time**, not specific to powers of two - a real `div`/`idiv` is only emitted when the divisor is a
*runtime* value the compiler cannot see at compile time.

This means the premise behind IDEAS.md's entry (and behind candidate 074's own "sidesteps the
cost by construction" framing) does not hold: a `const CHUNK_SIZE`, power-of-two or not, never
pays a real division/modulo instruction in this code, because it is always a compile-time
constant. Candidate 073's own writeup calling out "division/modulo indexing" as a suspected
cost was therefore imprecise from the start, independent of which value was chosen - the actual
per-access cost (whatever it is) is a `mov`+`shr` (power-of-two) or a handful of extra
instructions (`movabs`+`mul`+`shr`+`imul`+`sub`, non-power-of-two) either way, not a division.
The magic-multiply sequence is itself cheap (a handful of ALU instructions, no data-dependent
latency the way integer division has), which is consistent with - and now mechanistically
explains, rather than just statistically fails to contradict - this candidate's null timing
result. Testing what a *genuine* runtime-computed division/modulo would cost here would require
making `CHUNK_SIZE` a runtime value (e.g. a field set at construction instead of a `const`),
which is a materially different, larger change than swapping a constant, and was not attempted.
This leaves candidate 073's other named suspect - "less predictable inlining through the now-
generic `Option<PropertyTracker>` accesses" - as the only one of its two named causes not yet
ruled out or confirmed for the gap versus candidate 071's diagnostic figure.

## What's not done / open questions

- **The core question is now different from the one IDEAS.md's entry asked.** Since no `const`
  `CHUNK_SIZE` value (power-of-two or not) triggers a real division instruction, isolating
  "division/modulo cost" would require making the divisor a genuine runtime value, not sweeping
  further constants - not attempted here, and a materially larger change (turns a
  compile-time-constant array size into a runtime-sized allocation, changing `ChunkedArena`'s
  internal representation, not just one constant).
- `layout`/`flat` scrolling and `rects`/`list` scenes were not tested, same gap 074 left open.
- No `perf`/allocation-profiler attribution was attempted for overall per-access cost - same
  `perf_event_open`-blocked environment limitation as 073/074; the disassembly above answers the
  division/modulo-specific question directly (no div instruction exists to attribute cost to),
  but does not attribute whatever the remaining fixed per-access cost is to a specific cause.
- Only `insert`/`free` were disassembled; `get_mut`/`slot_mut` (the hot per-frame read path) are
  small enough to be inlined at call sites with no standalone symbol, so they were not located
  and disassembled directly - inferred to use the same magic-multiply pattern since they perform
  the identical `index / CHUNK_SIZE` / `index % CHUNK_SIZE` computation on the same constant, but
  this was not directly confirmed by disassembling the inlined call sites themselves.
