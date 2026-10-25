use egui::{Context, RawInput, Rect, pos2, vec2};

/// `ui.columns(n, ..)` in a region `available` points wide (default item spacing 8): the max_rect of every column.
fn column_rects(n: usize, available: f32) -> Vec<Rect> {
    let ctx = Context::default();
    let mut rects = vec![];
    let mut out = ctx.run_ui(
        RawInput { screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(500.0, 200.0))), ..Default::default() },
        |ui| {
            ui.allocate_ui(vec2(available, 50.0), |ui| {
                ui.columns(n, |columns| {
                    for column in columns.iter_mut() {
                        column.label("x");
                        rects.push(column.max_rect());
                    }
                });
            });
        },
    );
    out.textures_delta.clear();
    rects
}

fn main() {
    for (n, available) in [(3, 100.0), (20, 100.0)] {
        let result = std::panic::catch_unwind(|| column_rects(n, available));
        match result {
            Ok(rects) => {
                let inverted = rects.iter().filter(|r| r.is_negative()).count();
                println!(
                    "columns({n}) in {available} points: first rects {:?}, {inverted}/{n} columns have a negative width",
                    &rects[..2]
                );
            }
            Err(_) => println!("columns({n}) in {available} points: PANIC"),
        }
    }
}
