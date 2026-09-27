# 114 - Qt native-widget icon paint-cycle end-to-end verification (follow-up to candidate 113)

Candidate 113 fixed `NativeButton`/`NativeTab` (`internal/backends/qt/qt_widgets/`) re-rasterizing
an SVG-sourced `icon` on every paint (GitHub #7511) with a per-widget `RefCell` cache keyed by
`Image` equality, but only isolated the exact function the fix gates (`image_to_pixmap`), calling
it directly from a probe - never a real, live `QApplication` paint cycle. This candidate applies
the *same, unmodified* fix and drives it through a real offscreen-QPA Qt window instead: a
compiled `Button { icon: ... }` (`slint_build::CompilerConfiguration::with_style("qt")`), shown
under the real Qt backend (`i-slint-backend-qt`, not just the "qt" *style*), with repeated
hover/press/keyboard-focus changes dispatched via the public `Window::dispatch_event` API to force
real repaints - closing the "does a real repaint actually re-invoke `actual_icon()`, and how
often" open question left in 113's own `RESULT.md` and in `IDEAS.md`.

## What was tried

New standalone harness (`bench/`, crate `qt-icon-e2e-probe`) - no existing project bench reaches
an icon-bearing *native* Qt widget through a real `QApplication` event loop (`perf_test --qt` only
toggles style, has no icon scene and no Qt-backend windowing at all; see README.md's "Instrumentation
added" section for what this candidate adds as the first reusable harness of this kind).

- **Scene** (`ui/app.slint`, compiled with `with_style("qt".into())`): a `Window` with two
  `Button`s - `icon-button` (`icon: @image-url(shapes.svg)`, `icon-size: 128px`, the same 400-path
  asset candidate 041/113 used) and `other-button` (plain text, no icon set at all).
- **Driver** (`main.rs`): a repeating `slint::Timer` (4ms) that on each tick, for `N` ticks: toggles
  pointer hover in/out of `icon-button` every tick, does a full press+release click every 3rd tick,
  and sends a `Tab` key press/release (moving keyboard focus between the two buttons) every 5th
  tick - all via the public `Window::dispatch_event(WindowEvent::...)` API, run under
  `QT_QPA_PLATFORM=offscreen`, `i-slint-backend-qt`'s real `run_event_loop()` (`qApp->exec()`).
  Nothing about `icon-button`'s icon or `icon-size` ever changes across the whole run - exactly the
  "something else about the window changed" scenario GitHub #7511 describes.
- **Instrumentation** (`bench_probe.patch`, temporary, `bench-probe`-gated, on top of 113's
  `patch.diff`): two independent (calls, misses) counter pairs, one bucket for widgets with a real
  icon set and one for widgets with no icon (`ImageInner::None`) - `bench_probe_record_call` at the
  top of `actual_icon()`'s icon branch, `bench_probe_record_miss` only when the cache actually
  misses and `image_to_pixmap` is about to run. A separate `bench_probe_baseline.patch` provides
  the same counters with no cache check (every call is also a miss), for measuring the pristine
  (pre-113) tree with the identical harness.
- **`ensure_initialized`/`ensure_initialized(true)`** are already called internally by
  `i-slint-backend-qt`'s own `Backend::new()`/`run_event_loop()` - no extra setup needed beyond
  selecting the `backend-qt` feature and `QT_QPA_PLATFORM=offscreen`.

Qt 6.5.3 (`gcc_64`, qtbase only) installed via `aqtinstall` into this project's own gitignored
`tmp/Qt` (same recipe as candidate 113's own environment note). `uptime` load average 2.2-3.3 on
24 cores throughout. The harness is single-threaded end to end (one `QApplication` event loop, no
worker threads), so single-core (`taskset -c 3`) and multi-core (unpinned) coincide, as expected
and as 113 also noted for its own probe.

### Calls vs. misses (the real repaint pipeline's actual behavior)

| ticks | tree | icon-button calls | icon-button misses | other-button calls | other-button misses |
|---|---|---|---|---|---|
| 40 | baseline (pre-113) | 21 | 21 | 21 | 21 |
| 40 | patched (113's fix) | 21 | **1** | 21 | 21 |
| 200 | baseline | 101 | 101 | 101 | 101 |
| 200 | patched | 101 | **1** | 101 | 101 |
| 1000 | baseline | 501 | 501 | 501 | 501 |
| 1000 | patched | 501 | **1** | 501 | 501 |

(`calls` is roughly `ticks/2 + 1` because Qt coalesces the hover-toggle-driven repaints - only
about half of the alternating hover/press/focus ticks actually produce a new paint pass before the
next one arrives; this is Qt's own dirty-region/repaint scheduling, not an artifact of the probe.)

### Wall-clock (2000 ticks, `icon-size: 128px`, single-core and multi-core)

| tree | single-core run 1 | single-core run 2 | multi-core (unpinned) |
|---|---|---|---|
| baseline | 8160.1 ms | 8180.3 ms | 8136.8 ms |
| patched | 8173.6 ms | 8271.5 ms | 8154.2 ms |

No measurable wall-clock difference despite misses dropping from 1001 to 1 - see "Why the result
looks the way it does".

## Result

**Confirms the open question from candidate 113/`IDEAS.md` directly and unambiguously**: yes, in a
real, live `QApplication` paint cycle, ordinary hover/press/keyboard-focus changes elsewhere in the
window really do re-invoke `NativeButton::actual_icon()` for an unrelated icon-bearing button on
every repaint (calls scale linearly with tick count - 501 calls at 1000 ticks). On the pristine
tree, every one of those calls re-rasterizes (501 misses). With candidate 113's unmodified fix
applied, the identical repaint sequence produces the identical call count (501) but only **1**
miss, regardless of how many ticks the run lasts (1, 40, 200, or 1000 ticks all show exactly 1
miss) - the cache-hit path (`Image::eq` + size compare) is what actually runs for every repaint
after the first, end to end, through the real dirty-region/repaint decision logic, not just in
113's isolated function-level probe.

The wall-clock measurement, however, does **not** show a corresponding difference (~8.15-8.27s for
both trees at 2000 ticks) - see below for why this specific harness can't observe it, and why that
does not contradict the calls/misses result above.

## Does it change behavior/output?

- **Bit-exact?** Yes - this candidate applies the exact same `patch.diff` as candidate 113,
  unmodified; nothing here changes real source beyond what 113 already measured as bit-exact
  (identical output for a cache hit vs. a fresh `image_to_pixmap` call). This candidate only adds
  new *measurement* scaffolding (`bench_probe.patch`, `bench_probe_baseline.patch`), never applied
  together with the real fix in a way that changes runtime behavior for anything but the two new
  counters.
- **Is this a breaking change?** No - same conclusion as 113 (internal-only `Item` types).
- Project test suite (`cargo test -p i-slint-backend-qt --features rtti`, `QT_QPA_PLATFORM=offscreen`,
  real Qt 6.5.3 linked): **0 unit tests, 1/1 doctest passed**, identical before (pristine
  `cdb6c43f7`) and after (113's `patch.diff` applied) - re-confirms 113's own test-suite result,
  unaffected by anything this candidate adds.

## Why the result looks the way it does

The calls/misses result is exactly what 113 predicted from its own function-level probe, now
confirmed through the real pipeline: `Image`'s `PartialEq` (`vtable::VRc::ptr_eq` for the `Svg`
variant) holds across repaints because the compiled `icon: @image-url(...)` binding is a constant,
set once via `Property::set()` at construction (confirmed in the generated code, `app.rs`) - not a
recomputed binding - so `self.icon()` returns a clone of the exact same underlying `VRc` on every
call, and the cache's identity check reliably hits.

The wall-clock result looks flat between trees because **this harness's own fixed 4ms timer
cadence dominates total wall time**, not the per-repaint rasterization cost: 2000 ticks x 4ms =
8000ms is already ~98% of the measured ~8.15-8.27s regardless of tree, so the ~1.0ms-per-miss cost
113's own probe measured at 128px (times up to 1000 eliminated misses, i.e. up to ~1s of
theoretically eliminated work) gets absorbed into scheduling noise between fixed-cadence timer
fires rather than shortening the run. This is a property of *this specific* pacing choice, not
evidence that the fix has no wall-clock effect - candidate 113's own direct, unpaced probe
already showed the per-call cost precisely (0.33-1.0ms depending on icon size, "immeasurably fast"
on a cache hit). A harness with an unpaced/back-to-back repaint loop (no fixed-interval `Timer`)
would be needed to make the wall-clock savings visible end-to-end here; not attempted in this
session (see "What's not done" below).

## What's not done / open questions

- The wall-clock harness paces itself with a fixed-interval `Timer`, which caps the achievable
  repaint rate below where the per-call rasterization savings would show up in total wall time.
  An unpaced variant (fire the next tick as soon as the previous repaint completes, e.g. via
  `invoke_from_event_loop`/a zero-delay re-arm) would be needed to see the wall-clock delta
  end-to-end; not built this session.
- `NativeTab`/`NativeTabWidget` (113's other fixed call site) was not re-verified through this
  same live-window approach - only `Button`/`NativeButton`, matching the scene the original
  `IDEAS.md` entry proposed. The mechanism is structurally identical (same cache shape, same
  `Image::eq` check), so no reason to expect a different result, but it wasn't independently run.
- Found, and reported separately (not a performance result): `ImageInner`'s `PartialEq` has no
  arm for `(None, None)`, so `Image::default() == Image::default()` is `false` - this is why
  `other-button` (no icon set) shows `calls == misses` on *both* trees above (its cache, once
  113's fix is applied, can never hit). The rasterization cost for an empty image is trivial (see
  `ImageInner::render_to_buffer`'s own `None` handling), so this doesn't cost real time here, but
  it is a general, independent correctness bug - see `___OTHER___/004.IMAGE_NONE_NOT_EQUAL_TO_ITSELF/`.
