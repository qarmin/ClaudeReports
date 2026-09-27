# segments()/BezPath's Shape methods panic when the first PathEl is ClosePath

`Segments::next()` (`kurbo/src/bezpath.rs`) establishes the current subpath's start/last point
from whichever `PathEl` the wrapped iterator yields first, but its match arm for `PathEl::ClosePath`
is `panic!("Can't start a segment on a ClosePath")` instead of a value - hit whenever the very
first element the iterator produces is `ClosePath`. `BezPath` imposes no real invariant that the
first element be a `MoveTo`: the check in `BezPath::from_vec`/`push` is only a `debug_assert!`
(compiled out in release builds), and the free function `kurbo::segments()` (used directly, or via
`BezPath::segments()`, or via `&[PathEl]`'s own `Shape` impl) performs no check at all in any
build. This was flagged as a candidate to check (candidate #004 of this session's batch).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + direct execution | `kurbo::segments(vec![PathEl::ClosePath].into_iter()).collect::<Vec<_>>()`, debug build | Panicked at `kurbo/src/bezpath.rs:902:42` with the exact message from the candidate report. |
| Same, release build (`cargo run --release`) | Same input | Panicked identically - the free `segments()` function has no debug-only guard at all. |
| `BezPath::from_vec(vec![PathEl::ClosePath])` then `.area()` (a `Shape` trait method, a realistic caller-facing entry point) | debug build | Panics earlier, inside `from_vec` itself (`debug_assert!` at bezpath.rs:208, message "BezPath must begin with MoveTo") - a different, closely related guard that only exists in debug builds. |
| Same, release build | Same input | `from_vec`'s `debug_assert!` compiles out; `.area()` reaches `segments()` and panics with the exact same "Can't start a segment on a ClosePath" message/location as the free-function case. |

One approach (manual boundary-value construction) was sufficient to get a fully deterministic,
100%-repeatable reproduction on the first try - no escalation to miri/ASAN/fuzzing was needed
(this is a pure logic/control-flow panic, not memory-unsafety or input-space-dependent behavior;
see `HEURISTICS.md`'s tool-selection guidance for boundary-value construction on this bug class).

## Status

**Confirmed**. Reproduces 4/4 runs (1 initial + 3 repeat runs, all `cargo run --release`, no fixed
seed needed - the panic is fully deterministic on this exact input, not layout/seed-dependent).

**Fully reproducible**: **Yes**. This is a single-participant finding - a `BezPath`/`PathEl`
sequence handed to `kurbo`'s own public `segments()`/`Shape` API *is* the genuine external input
(the same kind of value any real caller building or deserializing a path could construct and pass
in), with no second participant whose reaction is being faked - see `CONFIRMATION.md` section 11's
"single-process repro remains completely fine" case. `repro/` calls the real public entry points
(`kurbo::segments()` and `BezPath::area()` via the `Shape` trait) directly, no internal-only
scaffolding.

## Class

crash

## Root cause

`Segments::next()` (`kurbo/src/bezpath.rs:888-932`) lazily initializes its `start_last: Option<(Point,
Point)>` state on the first element it sees, via `self.start_last.get_or_insert_with(|| { match el
{ ... } })`. The match has a value for `MoveTo`/`LineTo`/`QuadTo`/`CurveTo` (using their own
endpoint as a synthetic starting point, which is already the established "permissive" behavior for
a path that doesn't start with `MoveTo`, added in commit e0597a3 - see "Introduced by"), but for
`ClosePath` there's no sensible endpoint to synthesize, so the original author chose to panic there
instead. Nothing upstream of this iterator prevents a `ClosePath` from being the first element
handed to it:

- `BezPath::from_vec`/`push` only `debug_assert!` the first element is `MoveTo` - a no-op in
  release builds, and even in debug builds this only guards `BezPath`'s own constructors, not the
  free `segments()` function or `&[PathEl]`'s own `Shape` impl, which any caller can invoke
  directly on an arbitrary slice/iterator of `PathEl` with no debug or release guard at all.
- `BezPath::elements_mut()` also lets a caller mutate an already-valid path's first element to
  `ClosePath` after construction, bypassing `from_vec`/`push`'s check entirely (not separately
  reproduced this session, but the same code path applies).

Any `Shape` method built on `.segments()` - `area()`, `perimeter()`, `winding()`, `bounding_box()`,
`path_elements()`-based rendering, etc. - inherits this panic whenever it's ever called on a path
(or path-like `&[PathEl]`) whose first element is `ClosePath`.

## Introduced by

e0597a3f "Be more permissive in segments_of_slice." (2020-04-06, not on `main`'s current default
branch history directly but reachable via `git log --follow`, kurbo's pre-workspace-restructure
history). Before this commit, *any* non-`MoveTo` first element (including `ClosePath`) panicked
with `"First element has to be a PathEl::Moveto!"` - this commit made `LineTo`/`QuadTo`/`CurveTo`
permissive (treating their own endpoint as a synthetic start) but deliberately kept `ClosePath` as
a hard panic with today's exact message, since there was no endpoint value to synthesize for it at
the time. The code was later moved (unchanged in this respect) by d67b9dd "Make `kurbo` a crate
within a workspace (#434)" to its current location at `kurbo/src/bezpath.rs:902`.

## The fix

Give the `ClosePath` arm a real value instead of panicking: use `Point::ZERO` as the synthetic
start/last point, matching the pattern already used for `MoveTo`/`LineTo`/`QuadTo`/`CurveTo` (each
already just picks *some* point to seed `start`/`last` with in the "permissive" case). Since a
leading `ClosePath` inherently has no prior point to close a subpath back to, this makes `start ==
last` for that first iteration, so the existing `PathEl::ClosePath => if *last != *start { ... }
else { continue }` branch immediately falls into `continue` - i.e. the leading `ClosePath` produces
no segment and is silently skipped, exactly as an empty/no-op subpath should.

```diff
                     PathEl::QuadTo(_, p2) => p2,
                     PathEl::CurveTo(_, _, p3) => p3,
-                    PathEl::ClosePath => panic!("Can't start a segment on a ClosePath"),
+                    // No current point to close to; treat as a no-op below (start == last).
+                    PathEl::ClosePath => Point::ZERO,
                 };
```

Verified against more than just the exact triggering input (`CONFIRMATION.md` section 7): also
checked `[ClosePath, ClosePath, MoveTo((0,0)), LineTo((1,0)), LineTo((1,1)), ClosePath]` (multiple
leading `ClosePath`s followed by a real subpath) - produces the correct 3 segments (two lines plus
the closing line back to the `MoveTo` point), confirming the fix doesn't just suppress the panic
but preserves correct behavior once real path data follows.

Before/after (full run captured in `output.txt`):
- Before: `segments(): REPRODUCED - panicked: "Can't start a segment on a ClosePath"` /
  `BezPath::area(): REPRODUCED - panicked: "..."` (exit code 1).
- After: `segments(): did not reproduce - got []` / `BezPath::area(): did not reproduce - got -0`
  (exit code 0).

- Project test suite (`cargo test --workspace`): 187 + 10 + 55 passed, 0 failed, 1 ignored
  doctest (unchanged) - **before** and **after** the patch. No new failures introduced.

## What's not done / open questions

- `BezPath::elements_mut()` provides another way to construct a `BezPath` whose first element is
  `ClosePath` (mutate an existing valid path's first element in place, bypassing `from_vec`/
  `push`'s `debug_assert!` entirely) - not separately reproduced this session, but goes through
  the exact same `Segments::next()` code path and is fixed by the same patch.
- `PathEl::ClosePath` appearing first was the only case checked; a `BezPath` that is *empty*
  (`vec![]`) was not separately exercised here but is already handled correctly (existing code:
  `self.elements.next()` returns `None` immediately, the `for` loop body never runs, `next()`
  returns `None` - no `start_last` initialization is attempted).
- Did not check whether `SvgParser`/other kurbo entry points that build a `BezPath` internally
  could ever themselves produce a `ClosePath`-first path and pass it into a `Shape` method -
  this candidate targets the `segments()`/`Shape` iterator layer itself, which is the shared
  root cause regardless of which caller produces the malformed input.
