use dicom_pixeldata::{WindowLevel, WindowLevelTransform};

// Real-world tag values quoted verbatim from a reported file (Enet4/dicom-rs#826):
// (0028,1050) WindowCenter = 0.4565323565
// (0028,1051) WindowWidth  = 0.9130647131
// A RescaleSlope this small (~2.4e-4, also from the same report) is why the
// windowing values land in a sub-1.0 range in the first place: they're applied
// to already-rescaled pixel values, not raw stored integers.
const WINDOW_CENTER: f64 = 0.4565323565;
const WINDOW_WIDTH: f64 = 0.9130647131;
const Y_MAX: f64 = 255.0;

fn main() {
    let transform = WindowLevelTransform::linear(WindowLevel {
        width: WINDOW_WIDTH,
        center: WINDOW_CENTER,
    });

    // Sample rescaled pixel values spread across the declared window.
    let samples = [-0.05, 0.0, 0.1, 0.25, 0.4565323565, 0.6, 0.75, 0.9, 0.95, 1.0];

    println!("value -> output (y_max = {Y_MAX})");
    let mut outputs = Vec::new();
    for v in samples {
        let y = transform.apply(v, Y_MAX);
        println!("{v:>14} -> {y}");
        outputs.push(y);
    }

    // The window is supposed to spread values smoothly across [0, y_max].
    // If every strictly-inside-the-window sample collapses to just the two
    // extremes (0 or y_max), the transform degenerated into a hard
    // black/white threshold instead of a gradient - this is the bug from
    // Enet4/dicom-rs#826: `WindowLevelTransform::new`/`linear` clamps any
    // WindowWidth < 1 up to exactly 1, and the LINEAR function's own
    // definition (denominator `ww - 1`) becomes degenerate at ww == 1
    // (min == max), which always maps to `0` or `y_max` with nothing in
    // between.
    let interior: Vec<f64> = outputs[2..8].to_vec(); // values for 0.1..0.9, all inside [min,max) of the real window
    let all_extreme = interior.iter().all(|&y| y == 0.0 || y == Y_MAX);
    let distinct_interior_values: std::collections::BTreeSet<u64> =
        interior.iter().map(|y| y.to_bits()).collect();

    println!();
    println!("interior samples (should form a gradient): {interior:?}");
    println!("distinct interior output values: {}", distinct_interior_values.len());

    if all_extreme && distinct_interior_values.len() <= 2 {
        println!("BUG REPRODUCED: sub-unit WindowWidth collapsed into a hard 0/{Y_MAX} threshold, no gradient.");
        std::process::exit(1);
    } else {
        println!("OK: window produces a smooth gradient, not a hard threshold.");
    }
}
