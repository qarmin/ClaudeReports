// Shared RSS-measurement harness for ___OPTIMIZATIONS___ candidates in the calamine project.
// Not part of the calamine crate itself - a standalone binary that depends on it via a path dep.
//
// Usage: rss_probe <mode> <file path> [sheet name]
//   modes:
//     range    - open_workbook + worksheet_range() for every sheet, then drop everything
//     formula  - open_workbook + worksheet_formula() for every sheet (candidate 013's own target)
//     cells    - open_workbook + worksheet_cells_reader() streaming for every sheet (xlsx/xlsb only)
//     open     - open_workbook only, no sheet reads at all (baseline: workbook metadata / zip index)
//
// Prints one line of machine-readable output to stdout:
//   mode=<mode> file=<path> elapsed_ms=<u128> peak_rss_kb=<u64> end_rss_kb=<u64> cells=<u64> sig=<u64>
//
// peak_rss_kb comes from /proc/self/status VmHWM (Linux high-water mark of resident set size),
// read once at the very end of the run - this is what SKILL.md's MEASUREMENT.md calls out as the
// simplest reliable peak-RSS source when `/usr/bin/time -v` isn't installed (it isn't on this box).
// `sig` is a cheap correctness signature (rolling XOR/wrapping-add over cell positions/values) so
// two runs (baseline vs patched) can be compared for bit-exact output without hashing the whole
// dataset.

use calamine::{open_workbook, open_workbook_auto, Data, Reader, Xlsb, Xlsx};
use std::env;
use std::fs;
use std::time::Instant;

fn read_vmhwm_kb() -> u64 {
    let status = fs::read_to_string("/proc/self/status").expect("read /proc/self/status");
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("VmHWM:") {
            return rest.trim().trim_end_matches(" kB").trim().parse().unwrap_or(0);
        }
    }
    0
}

fn read_vmrss_kb() -> u64 {
    let status = fs::read_to_string("/proc/self/status").expect("read /proc/self/status");
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("VmRSS:") {
            return rest.trim().trim_end_matches(" kB").trim().parse().unwrap_or(0);
        }
    }
    0
}

fn sig_for_data(sig: &mut u64, row: usize, col: usize, cell: &Data) {
    *sig ^= (row as u64).wrapping_mul(0x9E3779B97F4A7C15).wrapping_add(col as u64);
    match cell {
        Data::Empty => {}
        Data::String(s) => {
            *sig = sig.wrapping_add(s.len() as u64);
        }
        Data::Float(f) => {
            *sig = sig.wrapping_add(f.to_bits());
        }
        Data::Int(i) => {
            *sig = sig.wrapping_add(*i as u64);
        }
        Data::Bool(b) => {
            *sig = sig.wrapping_add(*b as u64);
        }
        Data::DateTime(d) => {
            *sig = sig.wrapping_add(d.as_f64().to_bits());
        }
        Data::DateTimeIso(s) | Data::DurationIso(s) => {
            *sig = sig.wrapping_add(s.len() as u64);
        }
        Data::Error(_) => {
            *sig = sig.wrapping_add(1);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: rss_probe <range|cells|open> <file> [sheet]");
        std::process::exit(1);
    }
    let mode = args[1].as_str();
    let path = &args[2];

    let start = Instant::now();
    let mut cells: u64 = 0;
    let mut sig: u64 = 0;

    match mode {
        "open" => {
            let workbook = open_workbook_auto(path).expect("cannot open workbook");
            sig = workbook.sheet_names().len() as u64;
        }
        "range" => {
            let mut workbook = open_workbook_auto(path).expect("cannot open workbook");
            let names = workbook.sheet_names();
            for name in names {
                if let Ok(range) = workbook.worksheet_range(&name) {
                    for (r, row) in range.rows().enumerate() {
                        for (c, cell) in row.iter().enumerate() {
                            cells += 1;
                            sig_for_data(&mut sig, r, c, cell);
                        }
                    }
                }
            }
        }
        "formula" => {
            let mut workbook = open_workbook_auto(path).expect("cannot open workbook");
            let names = workbook.sheet_names();
            for name in names {
                if let Ok(range) = workbook.worksheet_formula(&name) {
                    for (r, row) in range.rows().enumerate() {
                        for (c, cell) in row.iter().enumerate() {
                            cells += 1;
                            sig ^= (r as u64).wrapping_mul(0x9E3779B97F4A7C15).wrapping_add(c as u64);
                            sig = sig.wrapping_add(cell.len() as u64);
                        }
                    }
                }
            }
        }
        // worksheet_cells_reader() streams cell-by-cell instead of materializing the dense
        // Range<Data> grid - only implemented directly on Xlsx/Xlsb, not through the generic
        // Reader trait / Sheets(auto) wrapper, so format is picked from the file extension here.
        "cells" => {
            let ext = std::path::Path::new(path)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            match ext {
                "xlsx" | "xlsm" | "xlam" => {
                    let mut workbook: Xlsx<_> = open_workbook(path).expect("cannot open workbook");
                    let names = workbook.sheet_names();
                    for name in names {
                        if let Ok(mut reader) = workbook.worksheet_cells_reader(&name) {
                            while let Ok(Some(cell)) = reader.next_cell() {
                                cells += 1;
                                sig = sig.wrapping_add(cell.get_position().0 as u64);
                            }
                        }
                    }
                }
                "xlsb" => {
                    let mut workbook: Xlsb<_> = open_workbook(path).expect("cannot open workbook");
                    let names = workbook.sheet_names();
                    for name in names {
                        if let Ok(mut reader) = workbook.worksheet_cells_reader(&name) {
                            while let Ok(Some(cell)) = reader.next_cell() {
                                cells += 1;
                                sig = sig.wrapping_add(cell.get_position().0 as u64);
                            }
                        }
                    }
                }
                other => {
                    eprintln!("cells mode not supported for extension: {other}");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("unknown mode: {mode}");
            std::process::exit(1);
        }
    }

    let elapsed_ms = start.elapsed().as_millis();
    let end_rss_kb = read_vmrss_kb();
    let peak_rss_kb = read_vmhwm_kb();

    println!(
        "mode={mode} file={path} elapsed_ms={elapsed_ms} peak_rss_kb={peak_rss_kb} end_rss_kb={end_rss_kb} cells={cells} sig={sig}"
    );
}
