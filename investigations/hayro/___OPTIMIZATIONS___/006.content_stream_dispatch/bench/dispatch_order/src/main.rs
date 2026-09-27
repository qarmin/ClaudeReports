use std::hint::black_box;
use std::time::Instant;

const SEQUENCE_TXT: &str = include_str!("../op_sequence.txt");

// Exact copy of the arm list + order from hayro-syntax's
// `TypedInstruction::dispatch` (hayro-syntax/src/content/ops_generated.rs),
// with each arm's real work replaced by a distinct u8 tag so this isolates
// pure match-arm cost, not `Foo::from_stack()` parsing cost.
#[inline(never)]
fn dispatch_original_order(op_name: &[u8]) -> u8 {
    match op_name {
        b"BX" => 0,
        b"EX" => 1,
        b"q" => 2,
        b"Q" => 3,
        b"cm" => 4,
        b"w" => 5,
        b"J" => 6,
        b"j" => 7,
        b"M" => 8,
        b"d" => 9,
        b"ri" => 10,
        b"i" => 11,
        b"gs" => 12,
        b"m" => 13,
        b"l" => 14,
        b"c" => 15,
        b"v" => 16,
        b"y" => 17,
        b"h" => 18,
        b"re" => 19,
        b"S" => 20,
        b"s" => 21,
        b"f" => 22,
        b"F" => 23,
        b"f*" => 24,
        b"B" => 25,
        b"B*" => 26,
        b"b" => 27,
        b"b*" => 28,
        b"n" => 29,
        b"W" => 30,
        b"W*" => 31,
        b"CS" => 32,
        b"cs" => 33,
        b"SC" => 34,
        b"SCN" => 35,
        b"sc" => 36,
        b"scn" => 37,
        b"G" => 38,
        b"g" => 39,
        b"RG" => 40,
        b"rg" => 41,
        b"K" => 42,
        b"k" => 43,
        b"sh" => 44,
        b"Do" => 45,
        b"BI" => 46,
        b"Tc" => 47,
        b"Tw" => 48,
        b"Tz" => 49,
        b"TL" => 50,
        b"Tf" => 51,
        b"Tr" => 52,
        b"Ts" => 53,
        b"BT" => 54,
        b"ET" => 55,
        b"Td" => 56,
        b"TD" => 57,
        b"Tm" => 58,
        b"T*" => 59,
        b"Tj" => 60,
        b"'" => 61,
        b"\"" => 62,
        b"TJ" => 63,
        b"d0" => 64,
        b"d1" => 65,
        b"MP" => 66,
        b"DP" => 67,
        b"BMC" => 68,
        b"BDC" => 69,
        b"EMC" => 70,
        _ => 255,
    }
}

// Same 73 arms, reordered so the operators that are actually most frequent
// in `img.pdf` (q/Q/cm/l/Tf/Tr/BT/Tm/ET/rg/TJ/m/S/RG, ~85% of all operators
// combined) come first. Tag values kept identical to the original so the
// two functions are behaviorally identical, only order differs.
#[inline(never)]
fn dispatch_frequency_order(op_name: &[u8]) -> u8 {
    match op_name {
        b"q" => 2,
        b"Q" => 3,
        b"cm" => 4,
        b"l" => 14,
        b"Tf" => 51,
        b"Tr" => 52,
        b"BT" => 54,
        b"Tm" => 58,
        b"ET" => 55,
        b"rg" => 41,
        b"TJ" => 63,
        b"m" => 13,
        b"S" => 20,
        b"RG" => 40,
        b"c" => 15,
        b"M" => 8,
        b"w" => 5,
        b"Tj" => 60,
        b"g" => 39,
        b"h" => 18,
        b"Do" => 45,
        b"f" => 22,
        b"W" => 30,
        b"G" => 38,
        b"n" => 29,
        b"f*" => 24,
        // Remaining arms: unchanged relative order, never observed in img.pdf.
        b"BX" => 0,
        b"EX" => 1,
        b"J" => 6,
        b"j" => 7,
        b"d" => 9,
        b"ri" => 10,
        b"i" => 11,
        b"gs" => 12,
        b"v" => 16,
        b"y" => 17,
        b"re" => 19,
        b"s" => 21,
        b"F" => 23,
        b"B" => 25,
        b"B*" => 26,
        b"b" => 27,
        b"b*" => 28,
        b"W*" => 31,
        b"CS" => 32,
        b"cs" => 33,
        b"SC" => 34,
        b"SCN" => 35,
        b"sc" => 36,
        b"scn" => 37,
        b"K" => 42,
        b"k" => 43,
        b"sh" => 44,
        b"BI" => 46,
        b"Tc" => 47,
        b"Tw" => 48,
        b"Tz" => 49,
        b"TL" => 50,
        b"Ts" => 53,
        b"Td" => 56,
        b"TD" => 57,
        b"T*" => 59,
        b"'" => 61,
        b"\"" => 62,
        b"d0" => 64,
        b"d1" => 65,
        b"MP" => 66,
        b"DP" => 67,
        b"BMC" => 68,
        b"BDC" => 69,
        b"EMC" => 70,
        _ => 255,
    }
}

fn bench(name: &str, f: fn(&[u8]) -> u8, ops: &[&[u8]]) {
    let iters = 20_000;
    let mut best = u128::MAX;
    for _ in 0..7 {
        let start = Instant::now();
        let mut acc = 0u64;
        for _ in 0..iters {
            for op in ops {
                acc = acc.wrapping_add(black_box(f(black_box(op))) as u64);
            }
        }
        black_box(acc);
        let elapsed = start.elapsed().as_nanos();
        best = best.min(elapsed);
    }
    println!(
        "{name:<24} {:>10.3} ns/op  ({:.3} us/full-pass, {} ops/pass)",
        best as f64 / (iters as f64 * ops.len() as f64),
        best as f64 / iters as f64 / 1000.0,
        ops.len()
    );
}

fn main() {
    let owned: Vec<Vec<u8>> = SEQUENCE_TXT
        .split_whitespace()
        .map(|s| s.as_bytes().to_vec())
        .collect();
    let ops: Vec<&[u8]> = owned.iter().map(|v| v.as_slice()).collect();
    println!("real img.pdf operator sequence: {} operators\n", ops.len());

    // Correctness: both functions must agree on every input.
    for op in &ops {
        assert_eq!(
            dispatch_original_order(op),
            dispatch_frequency_order(op),
            "mismatch for {:?}",
            String::from_utf8_lossy(op)
        );
    }
    println!("correctness check passed: identical output for all {} real ops\n", ops.len());

    bench("original order", dispatch_original_order, &ops);
    bench("frequency order", dispatch_frequency_order, &ops);
}
