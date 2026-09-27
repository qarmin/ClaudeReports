# Each applied fix, measured on its own

Three of this session's fixes live in the same directory (`sharedparley/`), so reverting a *file*
measures a bundle, not a fix. `tools/isolate.py` turns exactly one fix off - restoring the loop or
the call site it replaced - and leaves the others applied, so every row below is one change.

```
tools/isolate.py list          # what can be toggled
tools/isolate.py off <fix>     # write the disabled variant
tools/isolate.py restore       # put the tree back (verified byte-identical by md5)
```

Medians of 7 runs, both binaries built in the same session. Each `OFF` binary was checked to be
genuinely different before its row was trusted (the allocation count for 14, the panic for 23).

| fix | scene it acts on | OFF | ON | |
| --- | --- | --- | --- | --- |
| **14** dependency-node pooling | `scroll` 100 000 `bare` | 25.91 ms | 19.48 ms | **-25%** |
| **14** | `rects` 3 000 | 2.67 ms | 2.48 ms | -7% |
| **19** selection band search | `text` 400 000 `--select-all` | 5.20 ms | 4.40 ms | **-15%** |
| **23** horizontal glyph cull | `text` 200 x 2 000 chars | 18.05 ms | 11.85 ms | **-34%** |
| **23** | 8 000-char line | **panic** | ok | crash fixed |
| **26** text-cache sweep | `scroll` 60 000, memory | 917.4 MB | **304.5 MB** | **-67%** |
| **26** | `scroll` 20 000, memory | 316.9 MB | **115.9 MB** | **-63%** |
| **26** | `scroll` 20 000, frame | 23.33 ms | 25.46 ms | +9% (see `26_.../FIX.md`) |
| **26** | `scroll` 10 000, frame | 8.96-10.60 ms | 8.63-9.61 ms | no measurable change |
| **26** | `list` 60 000 (ListView) | 2.39 ms | 2.24 ms | unaffected |
| **35** cull geometry read | `scroll` 100 000 `bare` | 19.63 ms | **17.24 ms** | **-12%** (14/14 pairs) |
| **35** | `scroll` 20 000 layout rows | 24.59 ms | **21.54 ms** | **-12.4%** (9/9) |
| **35** | `nest` depth 8 | 13.30 ms | **11.96 ms** | **-10.1%** (9/9) |
| **35** | `list` 20 000 / `rects` 3 000 / `text` 50 000 | - | - | unaffected |
| **36** conditional state stack | `scroll` 100 000 `bare` | 17.93 ms | **16.46 ms** | **-8.2%** (11/12) |
| **36** | `nest` depth 8 | 12.40 ms | **11.94 ms** | -3.7% (12/12) |
| **36** | `scroll` 20 000 layout rows | 21.06 ms | **20.50 ms** | -2.7% (8/12) |
| **36** | `list` / `rects` / `text` / `clipped` / `opacity` | - | - | flat |
| **41** SVG per-frame cache | 12 SVGs sharing one source | 17.29 ms | **2.61 ms** | **-84.9%** (7/7) |
| **41** | 1 SVG repainted each frame | 3.38 ms | 3.36 ms | nothing to share within a frame |
| **41** | 30 PNGs (control) | 1.22 ms | 1.21 ms | unaffected |
| **29** sorted append | `list` 200 000 sorted `push` 200/frame | 8.93 ms | 3.99 ms | **-55%** |

## 03 has no toggle, so it was measured differently

Candidate 03 is the split of `TextParagraph` into metrics and glyphs; 19 and 23 are built on top of
it and cannot exist without it. Measuring it alone therefore meant **disabling 19 and 23 on both
sides** and comparing "`sharedparley` with only 03" against "`sharedparley` reverted":

| scene | 03 OFF | 03 ON | RSS OFF | RSS ON |
| --- | --- | --- | --- | --- |
| `text` 100 000 | 3.67 ms | 3.97 ms | 542.5 MB | **39.2 MB** |
| `text` 10 000 | 3.69 ms | 3.98 ms | 68.8 MB | **18.2 MB** |
| `text` 100 000 `--word-wrap` | 3.69 ms | 3.73 ms | 543.1 MB | 546.0 MB |
| `text` 100 000 `--resize-step 40` | **72.66 ms** | **4.83 ms** | | |

So 03 on its own: **-93% resident**, **-93% per resize frame**, **+0.30 ms per scrolling frame**,
and wrapped text untouched. Those are the numbers `03_text_document_shaping/RESULT.md` reports, now
confirmed without 19 and 23 mixed in.

## Why this is worth the extra builds

The combined table (`TODAY_END_TO_END.md`) says the session moved `scroll` -28% and `text` RSS
-93%, which is true but attributes nothing. It also produced a `list` "+21% regression" that turned
out to be noise and cost a bisect to disprove. Per-fix rows would have shown immediately that no
single change touched `list` at all.

**Rule for anything applied from here on: it gets an off-switch in `tools/isolate.py` at the time
it is written**, not reconstructed afterwards. Reconstructing the off-switch for 19 and 23 after
the fact meant re-deriving what the original loop looked like from the diff, which is exactly the
kind of thing that quietly measures the wrong pair of binaries.
