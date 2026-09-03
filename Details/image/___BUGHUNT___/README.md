# What is under test, and how findings are confirmed

Read this first if picking the work up cold - it explains the tooling, `SUMMARY.md` lists the
candidates investigated with it.

## Toolchain availability in this project

| tool | works here? | notes |
|---|---|---|
| `cargo test --lib` (default features) | Yes | 331 passing as of `44ce9226` (2026-08-27). Fast, always run before/after any fix. |
| `avif-native` feature (dav1d-backed AVIF decode) | Yes, with a workaround | `dav1d-sys` needs `pkg-config` to find `dav1d`; only the runtime `libdav1d7` package is installed here (no `-dev`/`.pc` file), and there's no sudo/apt access to install one, nor `meson`/`ninja` to build dav1d from source (the fallback CI uses, see `.github/workflows/rust.yml`'s `test_avif_decoding` job). Worked around with a local, non-system pkg-config shim - see "Instrumentation/tooling set up" below. |
| `miri` | Not tried yet this project | No candidate this far has needed it (no `unsafe` at the center of a confirmed finding yet, aside from a couple of well-guarded FFI pointer reads in the AVIF decoder). |
| ASAN/UBSAN | Not tried yet this project | Same - no candidate needed it yet. |
| `cargo clippy` | Yes | Works out of the box. |

## Existing project harnesses worth reusing

- `tests/limits.rs` - a real test-per-format harness asserting `Limits{max_image_width,
  max_image_height, max_alloc}` is actually enforced, both through `ImageReader` and through each
  format's raw `ImageDecoder::set_limits`. **Important**: its `avif()` test currently only runs
  when built with `avif-native`, and CI's own `test_avif_decoding` job never runs `cargo test` with
  that feature (only `cargo build`) - so a regression here can pass CI silently. Worth spot-checking
  any AVIF-adjacent change against this test manually (with the pkg-config shim below) rather than
  trusting CI alone.
- `tests/truncate_images.rs`, `tests/avif_termination.rs` - crash/hang-resistance smoke tests per
  format, cheap to run alongside any decoder-adjacent fix.

## Instrumentation/tooling set up for this workspace

Building this project with the `avif-native` feature (the real dav1d-backed `AvifDecoder`, as
opposed to the `avif` feature which is ravif/encode-only) normally fails here:
`dav1d-sys`'s build script uses the `system-deps` crate, which looks for a `dav1d.pc` pkg-config
file (not present - only the runtime shared library `libdav1d.so.7` from the `libdav1d7` apt
package is installed) and, failing that, tries to build dav1d from source via `meson`+`ninja`
(neither installed, no sudo to add them).

Workaround (used in candidate 011, reusable for any future AVIF-native candidate): `dav1d-sys`
ships **pre-generated FFI bindings** (`src/lib.rs`, no `bindgen`/headers involved at build time) -
it only needs to *link* against `libdav1d`, not compile against its headers. So a local, link-only
pkg-config shim is enough:

```bash
mkdir -p dav1d_shim/lib dav1d_shim/pkgconfig
ln -sf /usr/lib/x86_64-linux-gnu/libdav1d.so.7 dav1d_shim/lib/libdav1d.so
cat > dav1d_shim/pkgconfig/dav1d.pc << 'EOF'
prefix=<absolute path to>/dav1d_shim
libdir=${prefix}/lib
Name: dav1d
Description: local link-only shim
Version: 1.5.0
Libs: -L${libdir} -ldav1d
Cflags:
EOF
PKG_CONFIG_PATH=<absolute path to>/dav1d_shim/pkgconfig LD_LIBRARY_PATH=<absolute path to>/dav1d_shim/lib cargo build --no-default-features --features avif-native
```

Nothing is installed system-wide; the shim lives entirely inside a scratch/candidate directory.
Candidate 011's `justfile` has a `shim` recipe that builds this automatically from whatever
`libdav1d.so.*` `ldconfig` finds, so any future candidate's `justfile` can copy that recipe
verbatim instead of re-deriving it.

Caveat: this only works because `dav1d-sys` 0.8.3 ships pre-generated bindings. If a future
`dav1d`/`dav1d-sys` version switches to `bindgen`-at-build-time, this shim would also need real
headers and this trick would stop working.

## Ground rules used in this workspace

1. No claim without a command that reproduces it - every `Confirmed` candidate has a real,
   standalone `repro/` and the exact command that runs it.
2. Every candidate with a fix gets a real `patch.diff`, applied and built for real, plus the
   project's own tests where they cover the code.
3. `Not reproducible`/`Inconclusive` results are written down with the same care as confirmed
   bugs - a result with no file is indistinguishable from "never checked".
4. If a later candidate changes an earlier one's confirmation status, don't silently rewrite that
   candidate's `RESULT.md` - add a `CORRECTION.md` in its folder and update its `SUMMARY.md` row
   to point at it.
