# `Context::create_widget`: one lock acquisition instead of five to seven

A counting probe (temporary instrumentation of `Context::read/write`, not kept) showed 21 `Context` lock acquisitions per `ui.button(...)` (`parking_lot::RwLock`; ~8-10 ns each uncontended, and `atomic.rs` lines were the top lines of `create_widget`/`get_response` in `perf annotate`). `create_widget` took the lock for `memory.allows_interaction`, the widget insert, `ignore_focus`, the id-clash `used_ids` insert, the `warn_on_id_clash` option, `prev_pass_state` (highlight), the big `get_response` update and the accesskit-scroll request consumption. All of that now happens in one `write`: `get_response` was split into `new_response` (no lock) and `update_response(&mut ContextImpl, ...)`, the scroll-request consumption into `consume_scroll_requests`, and `check_for_id_clash` into the locked part plus `warn_about_id_clash` (the part that paints errors, still outside the lock). Candidate 005 had merged only two of these acquisitions with no visible effect; this removes six.

rustc 1.95.0 (59807616e 2026-04-14), base commit 86099aca1. Metrics: lock acquisitions per button (probe) and time.

## What was tried

Lock acquisitions per `ui.button` in the `alloc_count` harness (temporary probe): 21.0 -> 15.0 (empty frame 54 -> 48).

Time, interleaved A/B patched/base: scenes harness (3 rounds) scene_buttons_300 0.9843, 0.9647, 0.9980; scene_widget_mix_100 0.9806, 0.9793, 0.9764; scene_table_1000_rows 0.9609, 0.9250, 0.9675; scene_collapsing_nested_50 0.9638, 0.9608, 0.9610; scene_windows_30 0.9648, 0.9661, 0.9563. Demo (5 rounds): demo_no_tessellate 0.9851, 0.9868, 0.9915, 0.9998, 0.9710; demo_with_tessellate__realistic 0.9933, 0.9896, 0.9980, 0.9985, 0.9822.

## Result

Time: -2% to -4% (up to -7% in one table round) on widget-heavy scenes, about -1.2% on the demo UI build. Lock acquisitions: -6 per button.

## Does it change behavior/output?

- **Same behavior?** Yes in all tested paths: the order of the operations on `ContextImpl` is the same as before (memory focus bookkeeping -> widget insert -> id-clash insert -> response/interaction -> scroll requests). One reordering: the id-clash *error painting* (debug UI, only when a clash is detected) now happens after the response is computed instead of before; it does not read anything the response depends on. The accesskit scroll-request consumption now runs before the (debug-only) `on_widget_under_pointer` plugin hook and the accesskit node fill, which do not touch input action requests.
- Breaking change: no (private functions only).
- Tests: `cargo test -p egui` 86 + 3 + 188 passed, same as unpatched.
- Real change to the tested project's source: `patch.diff` (+200/-165, mostly re-indentation of the moved scroll-request block).

## Why the result looks the way it does

The context lock is taken ~20 times per widget; each CAS pair is ~10 ns, so 6 fewer acquisitions save ~60 ns of ~1.3 us per widget (~4-5%), matching the observed 2-4%.

## What's not done / open questions

- 15 acquisitions per button remain: `Painter::add` (`graphics_mut`) and `will_discard` per painted shape, `accesskit_node_builder` x2, `register_accesskit_parent`, `read_response`/theme lookup in `widget_style`, `viewport_id`, `fonts_mut`. Each is a candidate for merging with a neighbor.
