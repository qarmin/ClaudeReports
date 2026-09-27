// Standalone bench for ___OPTIMIZATIONS___ candidate 016 (PR #661: worksheet_range_region).
// Only usable against a tree that has PR #661's patch applied (worksheet_range_region_ref/
// worksheet_range_region don't exist on unpatched calamine) - not part of the shared rss_probe
// harness for that reason, see this candidate's RESULT.md.
//
// Usage: region_probe <full|col1|row1k|tiny> <xlsx file> <sheet name>
//
// Prints: mode=<m> file=<f> elapsed_ms=<n> peak_rss_kb=<n> end_rss_kb=<n> cells=<n> sig=<u64>

use calamine::{open_workbook, Data, ReaderRef, Xlsx};
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
        Data::String(s) => *sig = sig.wrapping_add(s.len() as u64),
        Data::Float(f) => *sig = sig.wrapping_add(f.to_bits()),
        Data::Int(i) => *sig = sig.wrapping_add(*i as u64),
        Data::Bool(b) => *sig = sig.wrapping_add(*b as u64),
        Data::DateTime(d) => *sig = sig.wrapping_add(d.as_f64().to_bits()),
        Data::DateTimeIso(s) | Data::DurationIso(s) => *sig = sig.wrapping_add(s.len() as u64),
        Data::Error(_) => *sig = sig.wrapping_add(1),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("usage: region_probe <full|col1|row1k|tiny> <xlsx file> <sheet name>");
        std::process::exit(1);
    }
    let mode = args[1].as_str();
    let path = &args[2];
    let sheet = &args[3];

    let start = Instant::now();
    let mut cells: u64 = 0;
    let mut sig: u64 = 0;

    let mut workbook: Xlsx<_> = open_workbook(path).expect("cannot open workbook");

    let range = match mode {
        "full" => workbook.worksheet_range_region(sheet, .., ..),
        // Column projection only: 1 of 8 columns, every row (no row-bound early break).
        "col1" => workbook.worksheet_range_region(sheet, [0u32], ..),
        // Row projection only: every column, first 1000 of 300000 rows (exercises early break).
        "row1k" => workbook.worksheet_range_region(sheet, .., 0u32..1000),
        // Both axes: the shape closest to the linked polars issue (a small slice of a huge sheet).
        "tiny" => workbook.worksheet_range_region(sheet, [0u32], 0u32..1000),
        _ => {
            eprintln!("unknown mode: {mode}");
            std::process::exit(1);
        }
    }
    .expect("worksheet_range_region failed");

    for (r, row) in range.rows().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            cells += 1;
            sig_for_data(&mut sig, r, c, cell);
        }
    }

    let elapsed_ms = start.elapsed().as_millis();
    let end_rss_kb = read_vmrss_kb();
    let peak_rss_kb = read_vmhwm_kb();

    println!(
        "mode={mode} file={path} elapsed_ms={elapsed_ms} peak_rss_kb={peak_rss_kb} end_rss_kb={end_rss_kb} cells={cells} sig={sig}"
    );
}
