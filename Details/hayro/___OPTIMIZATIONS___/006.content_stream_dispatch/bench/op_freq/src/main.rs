use hayro_syntax::Pdf;
use rustc_hash::FxHashMap;
use std::time::Instant;

const IMG_PDF: &[u8] = include_bytes!("../img.pdf");

fn main() {
    let pdf = Pdf::new(IMG_PDF.to_vec()).expect("failed to load img.pdf");
    let page = &pdf.pages().get(0).expect("no page 0");

    let mut freq: FxHashMap<Vec<u8>, u64> = FxHashMap::default();
    let mut total = 0u64;
    let mut sequence: Vec<String> = Vec::new();
    let mut ops = page.operations();
    while let Some(instr) = ops.next() {
        *freq.entry(instr.operator.to_vec()).or_insert(0) += 1;
        sequence.push(String::from_utf8_lossy(&instr.operator).into_owned());
        total += 1;
    }
    std::fs::write("op_sequence.txt", sequence.join(" ")).unwrap();

    let mut sorted: Vec<_> = freq.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    println!("total operators: {total}");
    println!("{:<6} {:>8} {:>8}", "op", "count", "pct");
    for (op, count) in &sorted {
        println!(
            "{:<6} {:>8} {:>7.2}%",
            String::from_utf8_lossy(op),
            count,
            100.0 * *count as f64 / total as f64
        );
    }

    // Re-time raw tokenizing+dispatch (TypedIter::next()) in isolation, several runs.
    let iters = 2000;
    let mut best = u128::MAX;
    for _ in 0..5 {
        let start = Instant::now();
        for _ in 0..iters {
            let mut typed = page.typed_operations();
            while typed.next().is_some() {}
        }
        let elapsed = start.elapsed().as_nanos();
        best = best.min(elapsed);
    }
    println!(
        "\ntyped_operations() full pass: {:.3} us/iter (best of 5, {} iters)",
        best as f64 / iters as f64 / 1000.0,
        iters
    );
}
