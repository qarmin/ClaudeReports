# VELLO_HYBRID_WEBGL_TEX_SUB_IMAGE_UNWRAP

`sparse_strips/vello_hybrid/src/render/webgl/mod.rs:3685-3726` (`upload_rgba32ui_rows`), called
from `upload_filter_data_texture`, `upload_alpha_texture` and `upload_encoded_paints_texture`,
ends with `gl.tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_array_buffer_view(...)
.unwrap()`. The candidate report's premise: this `web_sys` binding returns
`Result<(), JsValue>`, all 3 call sites' `used_size`/`width`/`rows` arithmetic is internally
consistent (so a `debug_assert_eq!` a few lines above never fires from ordinary scene input), but
a genuine *external* WebGL-level failure - a lost GPU context, a driver/extension quirk - could
still make the underlying JS call throw, turning that `Err` into a hard panic via `.unwrap()` on
an otherwise ordinary rendering path (the same "Result-to-.unwrap()" shape as the already-confirmed
candidate 026, but on the WebGL backend and via context loss rather than atlas exhaustion).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `sparse_strips/vello_hybrid/src/render/webgl/mod.rs`'s 3 call sites of `upload_rgba32ui_rows` plus the `web-sys` binding it calls (`~/.cargo/registry/.../web-sys-0.3.91/src/features/gen_WebGl2RenderingContext.rs`) | Static code reading, ~15 min | Confirmed `used_size`/`width`/`rows` are computed consistently at all 3 call sites (matches the candidate report). Confirmed the binding is generated with `#[wasm_bindgen(catch)]`, i.e. it returns `Err(JsValue)` **only** when the underlying JS `texSubImage2D()` call actually throws a JS exception - it does not surface ordinary WebGL errors recorded via `gl.getError()` as `Err` at all. This reframed the question: does a real browser's `texSubImage2D` ever *throw* for a genuine WebGL-level failure, or does it only ever set a GL error flag? |
| Real headless-Chromium probe (Node.js 22 + puppeteer, Google Chrome for Testing 151.0.7922.34 `chrome-headless-shell`, software WebGL2 via ANGLE/SwiftShader) exercising the actual browser API `upload_rgba32ui_rows` depends on: create a real `WebGL2RenderingContext`, perform the identical `texSubImage2D` call with matching enums/params, then genuinely force context loss via the real `WEBGL_lose_context` extension's `loseContext()` (confirmed lost via `gl.isContextLost()`) and repeat the same call; separately, try 4 more genuine WebGL-level failure conditions (undersized source buffer, negative width, out-of-bounds sub-rectangle, a deleted-but-bound texture) | Real browser, 3 repeat runs of the context-loss probe + 2 repeat runs of the 4-condition probe, see `output.txt` | **No JS exception was thrown in any case.** After real context loss, `texSubImage2D` returns normally (`lostContextCallThrew: false`, all 3/3 runs) and `gl.getError()` reports `37442` = `CONTEXT_LOST_WEBGL` - a GL error code, never surfaced through the `web-sys` `Result`. The 4 other conditions (undersized buffer, negative width, out-of-bounds region, deleted texture) also never threw (2/2 runs each) - each set `gl.getError()` to `INVALID_OPERATION`/`INVALID_VALUE` instead. Fully deterministic: all repeat runs produced byte-for-byte identical JSON output. |
| Spec corroboration: fetched the WebGL 1.0 spec text (registry.khronos.org) and grepped its error-handling phrasing | Static, ~5 min | The spec text consistently uses "generates an INVALID_ENUM/INVALID_OPERATION/INVALID_VALUE error" language for validation failures (never "throws"), consistent with what was observed empirically. (Exact "no effect while lost" wording wasn't located in the fetched WebGL 1.0 text within the time budget, but the empirical browser behavior above is authoritative regardless.) |

## Status

**Not reproducible.** The suspected trigger (a genuine browser-level WebGL failure - lost context,
or several other realistic GL-error conditions - causing the underlying JS call to throw, which
`.unwrap()` would then turn into a panic) does not occur in a real browser: `texSubImage2D` in a
real Chrome/ANGLE/SwiftShader WebGL2 implementation reports every failure condition tested (context
loss, undersized buffer, invalid width, out-of-bounds region, deleted texture) exclusively via
`gl.getError()`, never by throwing a JS exception - and the `web-sys` binding this code calls only
ever produces `Err` from an actual thrown exception, never from a recorded GL error. Since nothing
tested makes the call return `Err`, `.unwrap()` never observes a panic-triggering value for any of
these conditions. All probes were deterministic and reproduced identically across repeat runs (3/3
for context loss, 2/2 for the other 4 conditions) - see `output.txt`.

This doesn't prove `.unwrap()` here can *never* panic under any conceivable browser/driver bug
(browser implementations occasionally have their own bugs, and the "driver/extension issue" half
of the candidate's premise is inherently unfalsifiable without a specific known-buggy
browser/driver combination to test against) - but the specific, concrete trigger named in the
candidate report (context loss) was directly tested against a real, current, standards-compliant
WebGL2 implementation and does not reproduce, and neither do 4 other realistic GL-error conditions
in the same class.

**Fully reproducible**: N/A (Not reproducible - no bug to reproduce).

## Class

N/A

## Root cause

Not applicable as a bug - the code's actual behavior differs from the candidate report's premise:
`gl.tex_sub_image_2d_with_..(...)`'s `Result` return, per its `#[wasm_bindgen(catch)]` binding,
can only become `Err` from a **thrown JS exception**, and real-browser testing shows WebGL2's
`texSubImage2D` does not throw for context loss or for the other tested GL-error conditions - it
only ever records a GL error via `gl.getError()`, which this code path never even inspects. So the
`.unwrap()` here, while stylistically identical to candidate 026's confirmed
`image_cache.allocate(...).unwrap()` panic-on-`Result` pattern, does not have a demonstrated
reachable `Err` case the way that one did (atlas exhaustion is a real `AtlasError` `Result` vello's
own code returns; a WebGL browser API throwing for ordinary resource/state failures is not
something this session could get a real browser to do).

## Introduced by

N/A - not a confirmed bug.

## The fix

No fix attempted; no reproducible bug found to fix. (For general robustness, converting these
`.unwrap()`s to propagate a `RenderError` the way candidate 026's fix did for the wgpu backend's
`upload_image` would still be a reasonable defensive improvement precedent - i.e. "don't rely on
this JS call never throwing" is a defensible design choice even without a demonstrated trigger -
but that is a style/defensiveness judgment call for the maintainers, not a fix for a demonstrated
bug, so it wasn't made here per this skill's "don't force a patch for an input that happened to be
found" guidance, which doesn't apply since no triggering input was found at all.)

- Project test suite: not run (no code change was made).

## What's not done / open questions

- Only Chrome/ANGLE/SwiftShader was tested. Firefox and Safari's WebGL2 implementations were not
  directly probed this session (no browser binaries for either were available in this sandbox);
  the WebGL specification mandates the GL-error-not-exception behavior observed here across
  conformant implementations, so this is not expected to differ, but it wasn't independently
  verified against those engines.
- This session did not build/run the actual compiled `vello_hybrid` wasm code in a browser (e.g.
  via the project's existing `wasm-bindgen-test` harness in
  `sparse_strips/vello_hybrid/examples/native_webgl/tests/webgl.rs`) - that harness needs a
  WebDriver-compatible browser driver (`chromedriver`/`geckodriver`), which wasn't set up this
  session (no network-reachable download host for those tools worked in this sandbox; only
  puppeteer/playwright's own Chromium download paths were reachable). Instead, the browser-level
  API behavior `upload_rgba32ui_rows` depends on was probed directly and independently via
  puppeteer, which is sufficient to evaluate this specific candidate's premise (whether the
  browser call can throw) without needing to compile and load the wasm module itself.
- The identical `.unwrap()` pattern also exists on the WebGL backend's `upload_gradient_texture`
  (`sparse_strips/vello_hybrid/src/render/webgl/mod.rs`, ~line 1963,
  `gl.tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(...).unwrap()`) and on
  roughly 50 other `gl.*(...).unwrap()` call sites throughout the same file - all following the
  same `#[wasm_bindgen(catch)]`-Result-to-`.unwrap()` shape. Given this session's finding that the
  underlying JS calls don't throw for the realistic failure conditions tested, these are not
  flagged as separate candidates; if a future session finds a real browser/driver combination that
  *does* throw from an ordinary WebGL call, all of these would be worth revisiting as a batch
  rather than one at a time.
