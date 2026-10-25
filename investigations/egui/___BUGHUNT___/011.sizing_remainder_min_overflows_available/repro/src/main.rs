use egui::{Context, RawInput, Rect, pos2, vec2};
use egui_extras::{Size, StripBuilder};

/// Lays out a horizontal strip of `sizes` in exactly `available` points (no spacing) and returns the cell widths.
fn cell_widths(sizes: &[Size], available: f32) -> Vec<f32> {
    let ctx = Context::default();
    let mut widths = vec![];
    let mut out = ctx.run_ui(
        RawInput { screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(500.0, 200.0))), ..Default::default() },
        |ui| {
            ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            ui.allocate_ui(vec2(available, 50.0), |ui| {
                let mut builder = StripBuilder::new(ui);
                for &size in sizes {
                    builder = builder.size(size);
                }
                builder.horizontal(|mut strip| {
                    for _ in sizes {
                        strip.cell(|ui| widths.push(ui.max_rect().width()));
                    }
                });
            });
        },
    );
    out.textures_delta.clear();
    widths
}

fn report(name: &str, sizes: &[Size], available: f32) {
    let widths = cell_widths(sizes, available);
    let sum: f32 = widths.iter().sum();
    println!("{name}: available {available}, cell widths {widths:?}, sum {sum}{}", if sum > available + 1e-3 { "  <-- overflows" } else { "" });
}

/// Random remainder-only strips: how often do the cells together exceed both the available length and the sum of their minimums?
fn random_check() {
    let mut state = 42_u64;
    let mut next = |n: u64| {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state % n
    };
    let (mut overflows, mut worst) = (0, 0.0_f32);
    let runs = 3000;
    for _ in 0..runs {
        let count = 2 + next(5) as usize;
        let mins: Vec<f32> = (0..count).map(|_| if next(3) == 0 { 0.0 } else { next(40) as f32 }).collect();
        let sizes: Vec<Size> = mins.iter().map(|&m| Size::remainder().at_least(m)).collect();
        let available = 20.0 + next(150) as f32;
        let sum: f32 = cell_widths(&sizes, available).iter().sum();
        let limit = available.max(mins.iter().sum());
        if sum > limit + 1e-3 {
            overflows += 1;
            worst = worst.max(sum - limit);
        }
    }
    println!("random remainder-only strips: {overflows}/{runs} exceed max(available, sum of minimums), worst excess {worst}");
}

fn main() {
    random_check();
    // control: the first remainder's minimum alone is enough to be handled
    report("remainders [min 11, none, none]", &[Size::remainder().at_least(11.0), Size::remainder(), Size::remainder()], 30.0);
    // two minimums, the second only just below the average that was computed before the first one was taken out
    report("remainders [min 11, min 10, none]", &[Size::remainder().at_least(11.0), Size::remainder().at_least(10.0), Size::remainder()], 30.0);
    report("remainders [min 31, min 30, none]", &[Size::remainder().at_least(31.0), Size::remainder().at_least(30.0), Size::remainder()], 90.0);
}
