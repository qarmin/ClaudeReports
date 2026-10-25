//! Per-frame cost (UI build + tessellate) of several UI scenarios, for A/B comparisons.
use criterion::{Criterion, criterion_group, criterion_main};
use egui::{Context, Event, Key, Modifiers, Pos2, RawInput, pos2};
use std::hint::black_box;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn frame(ctx: &Context, input: RawInput, mut ui_fn: impl FnMut(&mut egui::Ui)) {
    let mut out = ctx.run_ui(input, |ui| ui_fn(ui));
    black_box(ctx.tessellate(core::mem::take(&mut out.shapes), out.pixels_per_point));
    out.textures_delta.clear();
}

fn scene(c: &mut Criterion, name: &str, mut ui_fn: impl FnMut(&mut egui::Ui)) {
    let ctx = Context::default();
    for _ in 0..3 {
        frame(&ctx, RawInput::default(), &mut ui_fn);
    }
    c.bench_function(name, |b| b.iter(|| frame(&ctx, RawInput::default(), &mut ui_fn)));
}

fn benches(c: &mut Criterion) {
    scene(c, "scene_buttons_300", |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for i in 0..300 {
                if ui.button(format!("Button number {i}")).clicked() {
                    black_box(i);
                }
            }
        });
    });

    scene(c, "scene_widget_mix_100", |ui| {
        let mut v = 0.5_f32;
        let mut b = true;
        let mut s = String::from("text");
        let mut n = 3_i32;
        egui::ScrollArea::vertical().show(ui, |ui| {
            for i in 0..100 {
                ui.horizontal(|ui| {
                    ui.label(format!("Row {i}"));
                    ui.add(egui::Slider::new(&mut v, 0.0..=1.0));
                    ui.checkbox(&mut b, "check");
                    ui.text_edit_singleline(&mut s);
                    ui.add(egui::DragValue::new(&mut n));
                    ui.radio_value(&mut n, i, "radio");
                });
            }
        });
    });

    scene(c, "scene_grid_labels_1000", |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            egui::Grid::new("g").striped(true).show(ui, |ui| {
                for r in 0..200 {
                    for col in 0..5 {
                        ui.label(format!("cell {r},{col}"));
                    }
                    ui.end_row();
                }
            });
        });
    });

    scene(c, "scene_table_1000_rows", |ui| {
        use egui_extras::{Column, TableBuilder};
        TableBuilder::new(ui)
            .striped(true)
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::remainder())
            .header(20.0, |mut h| {
                h.col(|ui| {
                    ui.strong("A");
                });
                h.col(|ui| {
                    ui.strong("B");
                });
                h.col(|ui| {
                    ui.strong("C");
                });
            })
            .body(|body| {
                body.rows(18.0, 1000, |mut row| {
                    let i = row.index();
                    row.col(|ui| {
                        ui.label(i.to_string());
                    });
                    row.col(|ui| {
                        ui.label(format!("name {i}"));
                    });
                    row.col(|ui| {
                        ui.label("some longer description text");
                    });
                });
            });
    });

    {
        let mut text = egui_demo_lib::LOREM_IPSUM_LONG.repeat(20);
        scene(c, "scene_text_edit_multiline_18k", |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(egui::TextEdit::multiline(&mut text).desired_width(600.0).desired_rows(20));
            });
        });
    }

    scene(c, "scene_painter_shapes_2000", |ui| {
        let rect = ui.max_rect();
        let p = ui.painter();
        let green = egui::Color32::GREEN;
        for i in 0..2000 {
            let x = (i % 80) as f32 * 12.0 + rect.left();
            let y = (i / 80) as f32 * 12.0 + rect.top();
            match i % 4 {
                0 => {
                    p.rect(
                        egui::Rect::from_min_size(pos2(x, y), egui::vec2(10.0, 10.0)),
                        2.0,
                        green,
                        (1.0, egui::Color32::WHITE),
                        egui::StrokeKind::Inside,
                    );
                }
                1 => {
                    p.circle(pos2(x + 5.0, y + 5.0), 4.0, green, (1.0, egui::Color32::RED));
                }
                2 => {
                    p.line_segment([pos2(x, y), pos2(x + 10.0, y + 10.0)], (1.5, egui::Color32::YELLOW));
                }
                _ => {
                    p.add(egui::Shape::line(
                        vec![pos2(x, y), pos2(x + 5.0, y + 10.0), pos2(x + 10.0, y)],
                        (1.0, egui::Color32::LIGHT_BLUE),
                    ));
                }
            }
        }
    });

    scene(c, "scene_polylines_2x5000_points", |ui| {
        let r = ui.max_rect();
        let a: Vec<Pos2> = (0..5000).map(|i| pos2(r.left() + i as f32 * 0.15, r.top() + 100.0 + 50.0 * (i as f32 * 0.01).sin())).collect();
        let b: Vec<Pos2> = (0..5000).map(|i| pos2(r.left() + i as f32 * 0.15, r.top() + 200.0 + 50.0 * (i as f32 * 0.013).cos())).collect();
        ui.painter().add(egui::Shape::line(a, (1.5, egui::Color32::RED)));
        ui.painter().add(egui::Shape::line(b, (1.5, egui::Color32::BLUE)));
    });

    {
        let ctx = Context::default();
        let tex = ctx.load_texture("t", egui::ColorImage::filled([64, 64], egui::Color32::RED), Default::default());
        let sized = egui::load::SizedTexture::from_handle(&tex);
        let mut f = |ui: &mut egui::Ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for _ in 0..200 {
                    ui.add(egui::Image::new(sized).fit_to_exact_size(egui::vec2(48.0, 48.0)).corner_radius(4.0));
                }
            });
        };
        for _ in 0..3 {
            frame(&ctx, RawInput::default(), &mut f);
        }
        c.bench_function("scene_images_200", |b| b.iter(|| frame(&ctx, RawInput::default(), &mut f)));
    }

    scene(c, "scene_show_rows_10000", |ui| {
        let row_height = ui.text_style_height(&egui::TextStyle::Body);
        egui::ScrollArea::vertical().max_height(400.0).show_rows(ui, row_height, 10_000, |ui, range| {
            for i in range {
                ui.label(format!("Row {i}"));
            }
        });
    });

    {
        let ctx = Context::default();
        let mut text = egui_demo_lib::LOREM_IPSUM_LONG.repeat(20);
        let n = std::cell::Cell::new(0_usize);
        let mut f = |ui: &mut egui::Ui| {
            egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                let out = egui::TextEdit::multiline(&mut text).desired_width(600.0).id(egui::Id::new("te")).show(ui);
                if n.get() < 3 {
                    out.response.request_focus();
                }
            });
        };
        for _ in 0..4 {
            n.set(n.get() + 1);
            frame(&ctx, RawInput::default(), &mut f);
        }
        c.bench_function("scene_text_edit_typing_18k", |b| {
            b.iter(|| {
                let mut input = RawInput::default();
                input.events.push(Event::Text("x".into()));
                frame(&ctx, input, &mut f)
            })
        });
    }

    scene(c, "scene_meshes_200_quads", |ui| {
        let r = ui.max_rect();
        for i in 0..200 {
            let mut mesh = egui::Mesh::default();
            mesh.add_colored_rect(
                egui::Rect::from_min_size(pos2(r.left() + (i % 20) as f32 * 30.0, r.top() + (i / 20) as f32 * 30.0), egui::vec2(25.0, 25.0)),
                egui::Color32::from_rgb(i as u8, 100, 200),
            );
            ui.painter().add(egui::Shape::mesh(mesh));
        }
    });

    {
        let ctx = Context::default();
        let mut sel = true;
        let mut f = |ui: &mut egui::Ui| {
            ui.style_mut().interaction.selectable_labels = true;
            for i in 0..60 {
                ui.label(format!("Selectable label number {i} with some more text in it"));
            }
            ui.checkbox(&mut sel, "x");
        };
        for _ in 0..3 {
            frame(&ctx, RawInput::default(), &mut f);
        }
        c.bench_function("scene_selectable_labels_60", |b| b.iter(|| frame(&ctx, RawInput::default(), &mut f)));
    }

    for n in [100_usize, 1000, 3000] {
        scene(c, &format!("scale_buttons_{n}"), move |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for i in 0..n {
                    ui.button(format!("Button {i}"));
                }
            });
        });
    }
    {
        let ctx = Context::default();
        let mut f = |ui: &mut egui::Ui| {
            let c = ui.ctx().clone();
            for i in 0..120_usize {
                egui::Window::new(format!("Window {i}"))
                    .id(egui::Id::new(("w", i)))
                    .default_pos(pos2(10.0 + i as f32 * 5.0, 10.0 + i as f32 * 4.0))
                    .show(&c, |ui| {
                        ui.label("hello");
                    });
            }
        };
        for _ in 0..3 {
            frame(&ctx, RawInput::default(), &mut f);
        }
        c.bench_function("scale_windows_120", |b| b.iter(|| frame(&ctx, RawInput::default(), &mut f)));
    }

    {
        use egui::epaint::{CubicBezierShape, EllipseShape, QuadraticBezierShape, RectShape};
        use egui::{Color32, Rect, Shape, Stroke, StrokeKind, vec2};
        let kinds: Vec<(&str, Box<dyn Fn(f32, f32) -> Shape>)> = vec![
            ("rect_fill", Box::new(|x, y| Shape::rect_filled(Rect::from_min_size(pos2(x, y), vec2(20.0, 12.0)), 0.0, Color32::GREEN))),
            ("rect_fill_round4", Box::new(|x, y| Shape::rect_filled(Rect::from_min_size(pos2(x, y), vec2(20.0, 12.0)), 4.0, Color32::GREEN))),
            ("rect_stroke_inside", Box::new(|x, y| Shape::rect_stroke(Rect::from_min_size(pos2(x, y), vec2(20.0, 12.0)), 4.0, Stroke::new(1.0, Color32::WHITE), StrokeKind::Inside))),
            ("rect_fill_stroke_round", Box::new(|x, y| Shape::Rect(RectShape::new(Rect::from_min_size(pos2(x, y), vec2(40.0, 20.0)), 6.0, Color32::GREEN, Stroke::new(1.5, Color32::WHITE), StrokeKind::Middle)))),
            ("rect_round_big", Box::new(|x, y| Shape::rect_filled(Rect::from_min_size(pos2(x, y), vec2(60.0, 40.0)), 20.0, Color32::GREEN))),
            ("rect_blur", Box::new(|x, y| Shape::Rect(RectShape::filled(Rect::from_min_size(pos2(x, y), vec2(60.0, 40.0)), 6.0, Color32::BLACK).with_blur_width(8.0)))),
            ("circle_fill_r6", Box::new(|x, y| Shape::circle_filled(pos2(x, y), 6.0, Color32::GREEN))),
            ("circle_stroke_r12", Box::new(|x, y| Shape::circle_stroke(pos2(x, y), 12.0, Stroke::new(2.0, Color32::WHITE)))),
            ("ellipse_fill", Box::new(|x, y| Shape::Ellipse(EllipseShape::filled(pos2(x, y), vec2(14.0, 8.0), Color32::GREEN)))),
            ("line_segment", Box::new(|x, y| Shape::line_segment([pos2(x, y), pos2(x + 20.0, y + 9.0)], Stroke::new(1.5, Color32::WHITE)))),
            ("hline_pixel", Box::new(|x, y| Shape::hline(x..=x + 20.0, y.round(), Stroke::new(1.0, Color32::WHITE)))),
            ("polygon_convex_hex", Box::new(|x, y| Shape::convex_polygon((0..6).map(|i| { let a = i as f32 * 1.0472; pos2(x + 8.0 * a.cos(), y + 8.0 * a.sin()) }).collect(), Color32::GREEN, Stroke::new(1.0, Color32::WHITE)))),
            ("quad_bezier", Box::new(|x, y| Shape::QuadraticBezier(QuadraticBezierShape::from_points_stroke([pos2(x, y), pos2(x + 10.0, y + 20.0), pos2(x + 20.0, y)], false, Color32::TRANSPARENT, Stroke::new(1.5, Color32::WHITE))))),
            ("cubic_bezier", Box::new(|x, y| Shape::CubicBezier(CubicBezierShape::from_points_stroke([pos2(x, y), pos2(x + 5.0, y + 20.0), pos2(x + 15.0, y - 20.0), pos2(x + 20.0, y)], false, Color32::TRANSPARENT, Stroke::new(1.5, Color32::WHITE))))),
            ("dashed_line", Box::new(|x, y| Shape::Vec(Shape::dashed_line(&[pos2(x, y), pos2(x + 40.0, y + 6.0)], Stroke::new(1.0, Color32::WHITE), 4.0, 2.0)))),
        ];
        for (name, make) in kinds {
            scene(c, &format!("shape_{name}_1000"), move |ui| {
                let r = ui.max_rect();
                for i in 0..1000 {
                    let x = r.left() + (i % 25) as f32 * 45.0;
                    let y = r.top() + (i / 25) as f32 * 30.0;
                    ui.painter().add(make(x, y));
                }
            });
        }
    }

    scene(c, "scene_collapsing_nested_50", |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for i in 0..50 {
                egui::CollapsingHeader::new(format!("Header {i}")).default_open(true).show(ui, |ui| {
                    ui.label("inner label");
                    ui.horizontal(|ui| {
                        ui.label("a");
                        ui.label("b");
                    });
                });
            }
        });
    });

    {
        let ctx = Context::default();
        let mut open = vec![true; 30];
        let mut f = |ui: &mut egui::Ui| {
            let c = ui.ctx().clone();
            for (i, o) in open.iter_mut().enumerate() {
                egui::Window::new(format!("Window {i}"))
                    .id(egui::Id::new(("w", i)))
                    .open(o)
                    .default_pos(pos2(10.0 + i as f32 * 20.0, 10.0 + i as f32 * 15.0))
                    .show(&c, |ui| {
                        ui.label("hello");
                        ui.button("ok");
                    });
            }
        };
        for _ in 0..3 {
            frame(&ctx, RawInput::default(), &mut f);
        }
        c.bench_function("scene_windows_30", |b| b.iter(|| frame(&ctx, RawInput::default(), &mut f)));
    }

    {
        let ctx = Context::default();
        let mut n = 0_usize;
        let mut f = |ui: &mut egui::Ui| {
            for i in 0..50 {
                ui.button(format!("b{i}"));
            }
        };
        for _ in 0..3 {
            frame(&ctx, RawInput::default(), &mut f);
        }
        c.bench_function("scene_input_events_50_buttons", |b| {
            b.iter(|| {
                n += 1;
                let mut input = RawInput::default();
                input.events.push(Event::PointerMoved(Pos2::new((n % 400) as f32, (n % 300) as f32)));
                input.events.push(Event::Key {
                    key: Key::A,
                    physical_key: None,
                    pressed: n % 2 == 0,
                    repeat: false,
                    modifiers: Modifiers::NONE,
                });
                input.events.push(Event::Text("a".into()));
                frame(&ctx, input, &mut f)
            })
        });
    }

    {
        let ctx = Context::default();
        let mut counter = 0_u32;
        let mut f = |ui: &mut egui::Ui| {
            counter += 1;
            for i in 0..40 {
                ui.label(format!("changing text {i} frame {counter}"));
            }
        };
        for _ in 0..3 {
            frame(&ctx, RawInput::default(), &mut f);
        }
        c.bench_function("scene_changing_labels_40", |b| b.iter(|| frame(&ctx, RawInput::default(), &mut f)));
    }
}

criterion_group!(g, benches);
criterion_main!(g);
