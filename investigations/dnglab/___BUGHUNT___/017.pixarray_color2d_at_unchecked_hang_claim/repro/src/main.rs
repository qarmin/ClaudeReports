// Reproduction for candidate 017: does an out-of-bounds `Color2D::at`/`at_mut` call, via
// rawler's own real public API (`rawler::pixarray::Color2D`), actually "hang indefinitely" in a
// release build, as claimed by GitHub issue #488 (https://github.com/dnglab/dnglab/issues/488)?
//
// This runs the reporter's own PoC verbatim (width=3, height=3, row=height, col=width - a
// modest 1-row-worth-past-the-end out-of-bounds access), plus two escalating out-of-bounds
// magnitudes, through the real, unmodified, public `Color2D::new`/`at`/`at_mut` API - the exact
// code path https://github.com/dnglab/dnglab/issues/488 points at (rawler/src/pixarray.rs).
//
// `std::hint::black_box` on every input prevents the compiler from const-folding the
// out-of-bounds-ness away (which, left alone, LLVM proves statically and turns into an
// unconditional trap - see scenario "literal" below, which is exactly the reporter's PoC as
// written). Timing is printed around each access so a hang (this process would need to be
// killed externally, e.g. by `timeout` in the justfile) is visibly distinguished from a fast
// crash or a fast, silent garbage read.
use rawler::pixarray::Color2D;
use std::hint::black_box;
use std::time::Instant;

fn run_scenario(name: &str, width: usize, height: usize, row: usize, col: usize) {
  let width = black_box(width);
  let height = black_box(height);
  let row = black_box(row);
  let col = black_box(col);

  println!("--- scenario '{name}': width={width} height={height} row={row} col={col} ---");
  let mut color2d = Color2D::<i32, 1>::new(width, height);

  let t0 = Instant::now();
  let value = *color2d.at(row, col);
  println!("[{name}] at() returned {value:?} in {:?} (no hang)", t0.elapsed());

  let t1 = Instant::now();
  let m = color2d.at_mut(row, col);
  *m = [42];
  println!("[{name}] at_mut() wrote in {:?} (no hang)", t1.elapsed());
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let scenario = args.get(1).map(String::as_str).unwrap_or("all");

  // Scenario "literal": the reporter's exact PoC values, compile-time-constant (not
  // black_box'd) - included separately below since black_box changes its outcome (see
  // output.txt: this one traps via an LLVM-inserted illegal instruction, not a hang either).
  if scenario == "all" || scenario == "literal" {
    println!("--- scenario 'literal' (reporter's exact PoC, compile-time-constant indices) ---");
    let mut color2d = Color2D::<i32, 1>::new(3, 3);
    let t0 = Instant::now();
    let _value = color2d.at(3, 3); // row = height, col = width, straight from the GitHub issue
    println!("[literal] at() returned in {:?} (no hang)", t0.elapsed());
  }

  // Scenario "modest": one row's worth past the end - the same logical out-of-bounds shape as
  // the reporter's PoC, but with runtime-only values so LLVM cannot prove it's UB at compile time.
  if scenario == "all" || scenario == "modest" {
    run_scenario("modest", 3, 3, 3, 3);
  }

  // Scenario "far": far enough past the allocation to almost certainly leave the mapped page.
  if scenario == "all" || scenario == "far" {
    run_scenario("far", 3, 3, 1_000_000, 0);
  }

  println!("=== all scenarios completed without hanging ===");
}
