# WAV_LIST_INFO_METADATA_DISCARDED

`symphonia-format-riff/src/wave/mod.rs`'s `WavReader::try_new` parses a WAVE file's own
`LIST`/`INFO` chunk into a local `metadata: MetadataLog` variable (`metadata.push(read_info_chunk(&mut
mss, list.len)?)`, around line 136), but that variable is never actually used - `WavReader`'s
own `metadata` field is instead initialized straight from `opts.external_data.metadata.unwrap_or_default()`
(around line 173), discarding everything just parsed. This looked suspicious because
`symphonia-format-riff`'s own `AiffReader::try_new` handles the analogous case correctly (seeds its
local `metadata` from `opts.external_data.metadata.unwrap_or_default()` *first*, then pushes parsed
tags onto it), and because of GitHub issue https://github.com/pdeljanov/Symphonia/issues/552 (open,
no PR at the time of writing), which reports exactly this behavior.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct code reading | Compared `WavReader::try_new` (symphonia-format-riff/src/wave/mod.rs) against `AiffReader::try_new` (symphonia-format-riff/src/aiff/mod.rs) in the same crate | Confirmed the discrepancy: AIFF correctly seeds its local `metadata` from `opts.external_data.metadata.unwrap_or_default()` and pushes onto it; WAV builds a separate local `metadata` from `Default::default()`, parses INFO tags into it, then throws it away and uses `opts.external_data.metadata.unwrap_or_default()` for the struct field instead |
| Manual boundary-value repro (public API, single-process, single-participant - see below) | Release build, a minimal hand-built but ordinary WAV file (RIFF/fmt /LIST-INFO/data) fed to the crate's own public `WavReader::try_new` + `FormatReader::metadata()` | Reproduces on first try, 4/4 runs (deterministic release-build logic bug, no sanitizer/miri needed - this is plain wrong-but-not-crashing output, not memory-safety UB) |

No escalation beyond the first tool was needed - this is a deterministic logic bug in a
non-`unsafe`, non-adversarial-parsing code path (a plain field-initialization mistake, not
something a fuzzer or sanitizer would surface any better than direct reading + a hand-built valid
input already did).

## Status

**Confirmed**, reproduces 4/4 runs, release build (`cargo run --release`), no fixed seed needed -
this is a deterministic logic bug, not a memory-layout-dependent one. Also re-verified with a
second input in the same class (an `IART`/Artist tag instead of `INAM`/Title) - see `output.txt`.

**Fully reproducible**: **Yes**. The repro calls the crate's own public `WavReader::try_new` /
`FormatReader::metadata()` API on a plain, ordinary (if minimal) WAV file - RIFF header, `fmt `
chunk, a `LIST`/`INFO` chunk (the real-world layout: placed before `data`, exactly where every real
WAV encoder that writes INFO tags puts it), and a `data` chunk. This is a single-participant
scenario (a file handed to a parser's real public entry point) - per `CONFIRMATION.md` section 11,
there is no second participant to fake here, so no client/server or two-process setup is needed;
the WAV bytes themselves *are* the genuine external input a real caller would supply.

## Class

logic

## Root cause

In `WavReader::try_new`:

```rust
let mut metadata: MetadataLog = Default::default();   // (line 97, before fix)
...
RiffWaveChunks::List(lst) => {
    let list = lst.parse(&mut mss)?;
    match &list.form {
        b"INFO" => metadata.push(read_info_chunk(&mut mss, list.len)?),  // populates the LOCAL variable
        _ => list.skip(&mut mss)?,
    }
}
...
return Ok(WavReader {
    ...
    metadata: opts.external_data.metadata.unwrap_or_default(),  // (line 173, before fix) uses a DIFFERENT MetadataLog, ignores the local one entirely
    ...
});
```

The local `metadata` variable that accumulates `read_info_chunk` results (the file's own
`LIST`/`INFO` tags - title, artist, comment, etc.) is populated but never read again; the struct
field of the same name is independently initialized from the caller-supplied
`opts.external_data.metadata`, which is `None`/empty for any ordinary caller that just opens a
file (nobody pre-supplies "external" metadata for a plain WAV open - that mechanism exists for
things like ID3 tags found by a container prober *before* handing off to a specific format's
reader, not for tags the format's own container natively carries). The result: any tag embedded in
a WAV file's own `LIST`/`INFO` chunk is silently parsed, then thrown away, and
`FormatReader::metadata()` always returns empty/`None` for it regardless of what the file actually
contains.

The GitHub issue this candidate is based on (#552) also notes that the chunk-parsing `loop`
returns as soon as it reaches the `Data` chunk (`RiffWaveChunks::Data(dat) => { ... return Ok(WavReader {...}) }`),
so even a fix that used the local variable correctly would still miss a `LIST`/`INFO` chunk placed
*after* the `data` chunk in the file - the loop never continues past `Data` to look for one. This
second issue was not separately fixed this session (see "What's not done" below); the confirmed
finding and the applied fix are specifically about the discarded-local-variable bug, which is
unconditional (it discards INFO metadata seen *before* `data` too, which is the common real-world
placement).

## Introduced by

9a5c526 "core (probe): Remove probed metadata." (2024-03-19). Before this commit, the WAV reader's
struct-literal simply used the local, already-populated `metadata` variable directly
(`metadata,` - shorthand field init). This commit changed that single line to
`metadata: options.metadata.unwrap_or_default(),` as part of a broader refactor (moving from
"probed metadata handled separately" to "an initial metadata set optionally supplied via
`FormatOptions`, with the reader's own log seeded from it") - but for `WavReader` specifically, the
change silently stopped seeding from the *local* variable at all, discarding it, while the
sibling `FormatReader`s that this session checked (`AiffReader`, `FlacReader`, `OggReader`,
`MkvReader`, `IsoMp4Reader`) all seed their local `metadata` from
`opts.external_data.metadata.unwrap_or_default()` *and then* push their own parsed tags onto it -
the pattern this commit apparently intended but didn't apply to the WAV reader's own already-named
`metadata` local (a naming collision between the pre-existing local variable and the newly
introduced field-init expression, both called "metadata", is a plausible mechanism for how this
was missed in review). Confirmed via `git log -S "options.metadata.unwrap_or_default"` /
`git show 9a5c526 -- symphonia-format-riff/src/wave/mod.rs`, which shows the single-line diff from
`metadata,` to `metadata: options.metadata.unwrap_or_default(),` with no other change to the
surrounding function.

## The fix

Changed two lines in `WavReader::try_new` (`symphonia-format-riff/src/wave/mod.rs`) to match the
pattern already used elsewhere in the same crate (`AiffReader::try_new`) and in every other
`FormatReader` implementation in this workspace that combines caller-supplied and natively-parsed
metadata:

1. Seed the local `metadata` variable from `opts.external_data.metadata.unwrap_or_default()`
   instead of `Default::default()`, so caller-supplied metadata is preserved as the starting point.
2. Use that same, now-populated local `metadata` variable directly in the returned `WavReader`
   struct literal, instead of re-reading (and re-discarding) `opts.external_data.metadata` a second
   time.

```diff
-        let mut metadata: MetadataLog = Default::default();
+        let mut metadata: MetadataLog = opts.external_data.metadata.unwrap_or_default();
...
-                        metadata: opts.external_data.metadata.unwrap_or_default(),
+                        metadata,
```

This is sufficient per `CONFIRMATION.md` section 7: checked against two different tags in the same
class (`INAM`/Title and `IART`/Artist), not just the one that first triggered the finding - both
now surface correctly through `FormatReader::metadata()` after the fix, and both still failed to
before it (see `output.txt`).

- Project test suite: `cargo test -p symphonia-format-riff`: 2 passed, 0 failed before -> 2 passed,
  0 failed after (identical). `cargo test --workspace` (every crate summed): 106 passed, 0 failed
  before -> 106 passed, 0 failed after (identical) - no regressions introduced.

## What's not done / open questions

- The second issue noted in the linked GitHub issue (#552) - a `LIST`/`INFO` chunk placed *after*
  the `data` chunk is never reached at all, because the parsing loop returns as soon as it hits the
  `Data` chunk arm - was **not** fixed this session. Fixing it would require restructuring the loop
  to keep scanning trailing chunks after `data` (bounded by the outer RIFF chunk length), which is
  a larger, more architecture-affecting change than this session's bounded fix; a maintainer should
  decide how much post-`data` scanning is safe/desirable (e.g. whether to require a seekable source,
  how to bound it for streamed/unbounded RIFF lengths per the `riff_len == u32::MAX` case already
  handled in this same function).
- Only `symphonia-format-riff`'s own `WavReader`/`AiffReader` pair was compared this session.
  `AiffReader` was confirmed to already do this correctly (see "Root cause" above) - this bug class
  does not currently appear duplicated elsewhere in this crate. Other crates in the workspace
  (`symphonia-bundle-flac`, `symphonia-format-ogg`, `symphonia-format-mkv`,
  `symphonia-format-isomp4`) were spot-checked via `grep -rn "external_data.metadata"` and all
  follow the correct seed-then-push pattern. `symphonia-format-caf` was also spot-checked: it
  never parses a native metadata chunk into a local variable at all (`opts.external_data.metadata.unwrap_or_default()`
  is the sole source of its `metadata` field), so this specific discard-a-local-variable bug class
  doesn't apply there.
