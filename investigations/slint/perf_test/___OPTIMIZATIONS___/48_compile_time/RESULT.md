# 48 - what a `.slint` file costs to build

Everything else in this workspace measures what an application costs to *run*. This measures what it
costs a developer to change it, which is a different budget and had never been looked at.

Generated `.slint` files of increasing size into `target/slintgen` (N sibling `Rectangle`s, each with
a `Text`), and timed `tools/compiler`.

## The compiler itself is ~0.2-0.3 ms per element

Medians of 3, `--release` build of `slint-compiler`:

| elements | compile | ms per element |
| --- | --- | --- |
| 10 | 11 ms | 1.100 |
| 50 | 17 ms | 0.340 |
| 200 | 41 ms | 0.205 |
| 400 | 81 ms | 0.203 |
| 800 | 180 ms | 0.225 |
| 1 600 | 458 ms | 0.286 |
| 3 200 | 927 ms | 0.290 |
| 6 400 | 1 906 ms | 0.298 |

About **9 ms of fixed cost**, then roughly linear with a mild upward drift - the per-element figure
rises from 0.20 ms at 400 elements to 0.30 ms at 6 400, so there is a small superlinear term but
nothing alarming.

**C++ output is consistently ~34% faster to generate than Rust** (0.64-0.68 of the Rust time at
400/1 600/6 400 elements).

## The number that matters is the expansion ratio

| source | `.slint` | generated Rust | ratio |
| --- | --- | --- | --- |
| `flat_400` | 51.7 KB | 2.86 MB | **55x** |
| `flat_6400` | 848 KB | 45.9 MB | **54x** |
| **`perf_test/ui/*.slint`** (a real UI) | **50.6 KB** | **3.42 MB** | **68x** |

By lines, `perf_test` is 1 319 lines of `.slint` becoming **32 220 lines of Rust** - 24x.

## And that is where the developer's time goes

| step, `perf_test` | time |
| --- | --- |
| `slint-compiler` on `ui/main.slint` | **158 ms** |
| full rebuild after touching one `.slint` file | **24.8 s** |

The Slint compiler is **0.6%** of it. The other 24.6 seconds are `rustc` compiling the 3.4 MB of Rust
it produced.

So "Slint is slow to compile" is, for a project of this size, almost entirely a statement about how
much Rust the code generator emits, not about how fast the generator runs. Optimising the compiler
would be optimising 0.6% of the wait; **the lever is the expansion ratio.**

## What this does not establish

- Whether 68x is avoidable. A generated item tree has to name every item and every property, so some
  expansion is inherent; whether it is 10x or 68x inherent is a compiler-design question this does
  not answer.
- Whether the emitted Rust is *shaped* badly for `rustc` (one huge function, deep generics, many
  monomorphisations) as opposed to merely large. That distinction decides whether the fix is "emit
  less" or "emit the same amount differently", and it needs `rustc -Ztime-passes` on the generated
  file - which needs a nightly toolchain this environment does not have.
- The C++ side: the generator is 34% faster, but what `clang` then does with the output is not
  measured, and C++ compile times are not obviously better.

## Reproduce

```sh
cargo build --release -p slint-compiler
# generate the fixtures (see the script in this candidate's history) then:
for n in 400 1600 6400; do
  time ./target/release/slint-compiler -f rust target/slintgen/flat_$n.slint -o /dev/null
done

# the real-world ratio
./target/release/slint-compiler -f rust --style fluent -I perf_test/ui perf_test/ui/main.slint -o /tmp/gen.rs
ls -l /tmp/gen.rs
```
