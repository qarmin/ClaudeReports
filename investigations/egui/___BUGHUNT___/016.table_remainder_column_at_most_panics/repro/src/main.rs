use egui::{Context, Pos2, RawInput, Rect, vec2};
use egui_extras::{Column, TableBuilder};

/// A one column table whose cells (about 40 points wide) are wider than the column's `at_most`.
fn run(column: Column) {
    let ctx = Context::default();
    for _ in 0..3 {
        let mut out = ctx.run_ui(
            RawInput { screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(400.0, 200.0))), ..Default::default() },
            |ui| {
                TableBuilder::new(ui)
                    .column(column.clone())
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.label("header text");
                        });
                    })
                    .body(|mut body| {
                        body.row(18.0, |mut row| {
                            row.col(|ui| {
                                ui.label("cell text");
                            });
                        });
                    });
            },
        );
        out.textures_delta.clear();
    }
}

fn main() {
    for (name, column) in [
        ("Column::auto().at_most(5.0)", Column::auto().at_most(5.0)),
        ("Column::initial(50.0).at_most(5.0)", Column::initial(50.0).at_most(5.0)),
        ("Column::remainder().at_most(5.0)", Column::remainder().at_most(5.0)),
        ("Column::remainder().at_most(100.0)", Column::remainder().at_most(100.0)),
    ] {
        let result = std::panic::catch_unwind(|| run(column));
        println!("{name}: {}", if result.is_ok() { "ok" } else { "PANIC" });
    }
}
