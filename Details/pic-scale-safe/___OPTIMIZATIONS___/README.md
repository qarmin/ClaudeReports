# What is under test, and how it is measured

Read this first if picking the work up cold - it explains the harness, `SUMMARY.md` lists the
candidates measured with it.

**Candidates 001-012 were not measured using the harness described below - they were ported
wholesale from `image` repo's own `___OPTIMIZATIONS___/` workspace**, where they were found and
measured while `image` was vendoring/patching this crate to fix its own resize delegation
performance (a different harness, different scenarios, different host). Every such candidate's
own `RESULT.md` says so explicitly at the top. What this repo's own harness (below) is for:
sanity-checking a ported candidate's patch actually builds and runs reasonably against this
crate's own bench suite, and as the basis for genuinely-local candidates. **Candidate 013 is the
first candidate measured directly against this repo, using a new bench added for it (see
"Instrumentation added for this workspace" below) - its numbers are real, live measurements on
this machine, not ported.**

## Workload

| bench | what it stresses |
|---|---|
| `app/benches/resize_rgb` | `resize_rgb8` (u8, 3 channels), Lanczos3, one large real photo (`assets/nasa-4928x3279.png`, 4928x3279) resized to 1/4 size, compared against `image`'s own resizer and `fast_image_resize` |
| `app/benches/resize_rgba`, `resize_la`, `resize_plane` | same shape, different pixel layouts |
| `app/benches/resize_rgb_u16`, `resize_rgba_u16` | 16-bit variants |
| `app/benches/resize_rgb_f32`, `resize_rgba_f32` | f32 (floating-point path) variants |
| `app/benches/premultiply_alpha` (candidate 013 only, see below) | `premultiply_rgba8`/`unpremultiply_rgba8`/`premultiply_la8`/`premultiply_rgba16`, 1920x1080 synthetic buffers |

## Instrumentation added for this workspace

**`app/benches/premultiply_alpha`** (candidate 013, 2026-08-29): this repo had no bench reaching
`src/alpha.rs` before this candidate - every existing bench above exercises the resize path only.
Added a criterion bench covering `premultiply_rgba8`, `unpremultiply_rgba8`, `premultiply_la8`,
and `premultiply_rgba16` (bit-depth 10) at 1920x1080, following the existing benches' structure
(one `main.rs` per `[[bench]]` entry in `app/Cargo.toml`). **This bench is NOT currently present
in the working tree** - per this skill's "the main project's working tree is always left clean"
rule, it lives only inside candidate 013's own `patch.diff` (bundled together with that
candidate's actual source fix, the same way `image`'s own ported candidates bundle a
`benches/imageops.rs` change with their fix). A future candidate touching `alpha.rs` again should
extract this bench from `013.ALPHA8_SKIP_REDUNDANT_ALPHA_WRITE/patch.diff` and extend it, rather
than writing a second, separate `alpha.rs` bench from scratch (see SKILL.md's "Bench harnesses:
grow a few, don't multiply").

**A genuinely important, non-obvious fact about *this repo's own* build config was discovered
while building this bench, worth knowing before trusting any absolute number measured on this
machine going forward:** `.cargo/config.toml` at this repo's root sets
`rustflags = ["-Ctarget-cpu=native"]` for the whole workspace (both the crate and `app/`). This
means every `cargo build`/`cargo bench` invoked from within this repo already compiles with the
measuring machine's full native instruction set (AVX-512, on the machine used this session) -
**not** the generic x86-64 baseline candidates 005/006 measured (via a vendored copy inside a
*different* project, `image`, which has no such config). This setting does not propagate to
downstream consumers of the published crate (Cargo does not read a dependency's own
`.cargo/config.toml`), so it only affects this repo's own local dev/bench workflow - but it means
any future candidate's numbers measured here reflect an unusually well-vectorized starting point
compared to what a real downstream consumer's default build gets. Override with `RUSTFLAGS=""` to
measure the generic-baseline case a downstream consumer is more likely to hit (see candidate 013's
own `RESULT.md` for a case where this distinction completely flips a result's direction).

Every ported candidate's own original measurement used a *different* corpus entirely (small,
procedurally-generated `image`-crate test images, typically `400x300->32x24` or similar) - see
each `RESULT.md`. This repo's own bench corpus is one large real-world photo at one resize ratio;
it does not exercise the same small-output-size regime most of candidates 001-004 are about. A
future session wanting to actually reproduce those candidates' own numbers on this repo directly
would need to add a small-output-size bench case here first (see `IDEAS.md`).

## Measurement harness

`cargo bench --bench <name>` inside `app/` (criterion, standard `cargo bench` HTML+text report).
Pass `--features pic-scale-safe/rayon` to enable this crate's optional `rayon` feature for any
candidate that touches rayon-gated dispatch (001-004).

## Environment

Not independently characterized in this session (no `taskset`/`perf`/`uptime` baseline was
established, since no new measurement was taken here - see the porting note above). A future
session doing real measurement against this repo should establish this section properly per
`MEASUREMENT.md`.

## Ground rules used in this workspace

1. No claim without a command that reproduces it - every candidate folder has a `justfile`; for
   the ten ported candidates, that command reproduces the *patch applying and building*, not the
   original historical numbers (which came from a different project's harness entirely - see each
   `RESULT.md`).
2. Every candidate that touches real source gets a real `patch.diff`, verified (`git apply
   --check`, `cargo check`, `cargo test`) against this repo's current `HEAD` in this session, even
   though the *timing* numbers were not re-measured here.
3. Null results are written down with the same care as wins - candidates 005/006 (measurement-only,
   no patch) are recorded exactly like every other candidate.
4. If a later candidate changes the baseline enough that an earlier candidate's numbers no longer
   hold, don't silently rewrite that candidate's `RESULT.md` - add a `CORRECTION.md` and update its
   `SUMMARY.md` row to point at it.

## Tooling adaptation status (as of 2026-08-29)

`tools/paired.sh`'s `run()` function has been adapted (not yet validated against a real bench
run) for criterion's `time: [lo mid hi]` output shape, since no live measurement was taken while
porting candidates 001-010 into this workspace. `tools/matrix.sh`, `tools/pixelcheck.sh`, and
`tools/isolate.py` are still unmodified copies of the skill's own templates - their `ADAPT:`
sections need real adaptation (and testing against an actual bench run) before a future session
relies on them for genuinely new measurement work on this repo.
