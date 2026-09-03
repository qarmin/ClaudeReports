# tr!() gettext lookup re-executed per binding re-evaluation

`internal/core/translations.rs:187-273`'s `translate()` calls the gettext-rs (`dgettext`/`dngettext`)
path on every call, even when only an unrelated interpolation argument changed and the locale
generation counter (`global_translation_property()`) is unchanged from the last call at that same
call site. This was flagged during a 2026-08-23 i18n/font discovery sweep, backlogged in
`IDEAS.md`, and picked up here.

## What was tried

A thread-local cache (`cached_translate_gettext`, keyed by the *content* of `(original, ctx,
domain)` plus the locale generation) that skips `translate_gettext()`'s FFI call and its
`mangle_context`/`demangle_context` allocations on a hit, reusing the cached pattern string and
only paying a `to_string()` copy before the existing argument-formatting `write!()` runs. Applied
only to the plural-less path (`plural.is_empty()`) - a plural form's `msgstr` depends on gettext's
per-language plural rule for `n`, so caching it under a generation-only key without `n` would be
a real correctness bug (see "Why the result looks the way it does").

A pointer-identity cache key (`original.as_ptr()`, assuming the compiler embeds `&'static str`
literals at each call site) was designed first and abandoned once codegen was actually checked:
`private_unstable_api::translate()` takes `original`/`contextid`/`domain` as owned `SharedString`,
and the generated Rust code for `Expression::StringLiteral` is `sp::SharedString::from(#s)` - a
**fresh heap allocation from the literal text on every call**, not a stable `&'static str`. A
pointer key would silently never hit for the exact code path this candidate targets. Content-based
keying (hash the three strings, store owned copies once per unique call site) was used instead, as
the backlog entry actually specified.

**Environment limitation discovered while measuring (see `IDEAS.md`'s two new follow-up entries):**
this sandbox has no `locales` package and no root/apt access (`localedef` fails - no charmaps
installed), and reading glibc's `intl/dcigettext.c` confirms `LANGUAGE`/catalog search is
hardcoded to be skipped entirely whenever the active locale category is `C`, `POSIX`, or any
`C.<encoding>` (the only non-`POSIX` locale installed here is `C.utf8`) - verified both by reading
the source and by a standalone C/`ctypes` reproduction outside this candidate's own patch. So the
scenario the candidate is actually about - a real catalog already loaded, matched via an
mmap+hash-table lookup - could not be exercised end-to-end here. Two modes were measured instead:
the literal "no locale installed" case (`dgettext`'s own near-zero-cost early return, which this
patch's cache overhead cannot beat) and a `with-context` proxy that forces the *other* per-call
cost `translate_gettext` always pays regardless of whether a catalog matches (`mangle_context`'s
`format!` allocation before the call, `demangle_context`'s `split`/`into()` after) - the cache
removes both categories of that other cost, giving a real, if partial, view of the win in the
untestable real-catalog case.

Bench: standalone crate (`bench/`, no existing `perf_test` scene touches i18n at all), calling
`i_slint_core::translations::translate()` directly (no Slint window/component needed - the
gettext-rs branch doesn't read `GLOBAL_CONTEXT`), 300 000 iterations, 10% warmup discarded, a
hand-built `fr/LC_MESSAGES/bench.mo` catalog (unreachable per the above, kept for anyone re-running
this on a machine with real locale data), FNV-style running hash of every output for correctness.

| Variant / mode | single-core p50 (baseline &rarr; patched) | multi-core p50 (baseline &rarr; patched) | notes |
|---|---|---|---|
| `changing-args` (no context, arg changes every call) | 210ns &rarr; 240ns (+14%) | 210ns &rarr; 245ns (+17%) | regression: baseline's `dgettext` early-return (`C` locale) is already near-free; the cache's own hash+lookup+`to_string()` costs more |
| `same-args` (no context, arg constant) | 161-210ns &rarr; 190ns (no clear direction, baseline noisy) | 180-201ns &rarr; 181ns (small win, ~5-10%) | inside/near the noise floor either way |
| `with-context` (non-empty context, arg changes every call) | 250-251ns &rarr; 141ns (-44%) | 241-250ns &rarr; 140-141ns (-42%) | real win: cache skips `mangle_context`/`demangle_context`'s allocations and the FFI round trip entirely on a hit |

Each cell is 3 repeated runs per variant/core-count/build (`taskset -c 3` for single-core,
unpinned for multi-core); the ranges shown are the spread across those 3 runs, not a single
sample. Hashes were bit-identical between baseline and patched builds in every mode (see
"Correctness" below).

## Result

Mixed and mode-dependent, and - per the environment-limitation note above - measured on a proxy
for the target scenario, not the scenario itself:

- Where `translate_gettext`'s own per-call cost is already near-zero (no context, locale-not-found
  fast path), the cache adds measurable overhead: **+14-19%** consistently across single- and
  multi-core, repeated runs.
- Where `translate_gettext` does real allocating work every call regardless of catalog outcome (a
  non-empty translation context - the *default* for any Slint app that hasn't disabled
  `DefaultTranslationContext`, see `resolving.rs`), the cache is a clear, repeatable win:
  **-42% to -44%**, single- and multi-core, 3/3 runs each direction.
- The plain "no context, changing args" case (the exact wording of the original candidate
  description) lands closest to a wash on `same-args` and a small loss on `changing-args` in this
  environment, because this sandbox cannot reach the actually-expensive branch (a real catalog hash
  lookup) the candidate's own cost estimate ("3-6 heap allocations plus a C FFI/glibc catalog
  lookup") was describing.

## Does it change behavior/output?

- **Bit-exact?** Yes. The bench's running hash of every produced string was identical between the
  baseline and patched binaries in all three modes (`4b9842e851404070` for `changing-args` and
  `with-context`, `3ac1de079cc81340` for `same-args`).
- **Is this a breaking change?** No public API changes; `cached_translate_gettext` is a private
  function.
- One correctness-relevant design point, not a behavior change but worth flagging for a reviewer:
  a stale cache entry is only replaced by content match + generation mismatch, so a live app that
  keeps a `tr!()` call site around for the lifetime of the process retains exactly one entry per
  unique `(msgid, context, domain)` it has ever evaluated (refreshed in place on every real locale
  switch) - not unbounded growth per locale switch, but also never evicted, matching the existing
  `TextLayoutCache`-style precedent already flagged as "never evicts" elsewhere in this workspace
  (candidate 026) rather than introducing a new unbounded-growth class.
- Plural forms (`ntranslate`/`dngettext`) are untouched - always take the uncached path, exactly as
  before this patch.
- Project test suite (`cargo test -p i-slint-core --lib --features gettext-rs`): **307 passed, 0
  failed** before the patch; **307 passed, 0 failed** after. No new failures.

## Why the result looks the way it does

The candidate's own cost estimate ("3-6 heap allocations plus a C FFI/glibc catalog lookup") is
accurate for an app with a real, loaded translation catalog - but reproducing that specific
condition requires an installed non-`C`/non-`POSIX` locale, which this sandboxed dev environment
does not have and cannot install without root (confirmed by reading glibc's own
`guess_category_value`/`__dcigettext` source: any `C.<encoding>` locale, the only kind installable
here without root, is explicitly excluded from ever loading a catalog, by design, regardless of
`LANGUAGE`/`bindtextdomain`). What *was* measurable - the `mangle_context`/`demangle_context`
allocations `translate_gettext` always pays before/after the FFI call whenever a translation
context is set - already shows the shape of win the full fix should produce once a catalog lookup
is added on top: those two allocations plus the FFI call are exactly what a cache hit skips.
Without a context and without a catalog to search, `dgettext`'s own fast path is cheap enough that
the cache's bookkeeping (hash three strings, look up a bucket, scan it, clone a string out) costs
more than what it replaces - this is the one scenario in the candidate's own description this
environment could actually validate as a **regression**, not a win.

## What's not done / open questions

- Re-run this candidate's `bench/` on a machine with a real, installed non-`C` locale and a real
  `gettext`-loaded catalog for the target msgid - the scenario the candidate is actually about.
  Both follow-up leads (this one, and the `SharedString`-per-call-site allocation one) are recorded
  in `IDEAS.md`.
- Not measured: the `feature = "tr"` (external `Translator`) path - this cache only wraps the
  `gettext-rs` branch; a caller using the `tr` crate's own translator gets no benefit or cost
  change from this patch either way.
- Allocation counts were not independently instrumented for this candidate (no existing
  `perf_test`-style allocation counter reaches this code path outside a Slint component); the
  hash-based correctness check and the timing numbers above are the only evidence collected.
