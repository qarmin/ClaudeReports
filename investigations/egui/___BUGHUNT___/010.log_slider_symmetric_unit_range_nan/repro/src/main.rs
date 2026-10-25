use std::ops::RangeInclusive;

use egui::{Context, Event, Modifiers, PointerButton, RawInput, Rect, Slider, Ui, pos2, vec2};

/// Draws a logarithmic slider and drags its handle across the rail with a real pointer.
/// Returns the final value and the value after every frame.
fn drag_slider(range: RangeInclusive<f64>) -> Vec<f64> {
    let ctx = Context::default();
    let mut value = 0.5_f64;
    let mut values = vec![];
    let mut center = pos2(100.0, 20.0);
    let mut rail = Rect::NOTHING;
    for frame in 0..5 {
        let events = match frame {
            1 => vec![Event::PointerMoved(center)],
            2 => vec![Event::PointerButton { pos: center, button: PointerButton::Primary, pressed: true, modifiers: Modifiers::NONE }],
            3 => vec![Event::PointerMoved(pos2(rail.min.x + rail.width() * 0.25, center.y))],
            4 => vec![Event::PointerMoved(pos2(rail.min.x + rail.width() * 0.9, center.y))],
            _ => vec![],
        };
        let mut out = ctx.run_ui(
            RawInput { events, focused: true, screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(400.0, 100.0))), ..Default::default() },
            |ui: &mut Ui| {
                let resp = ui.add(Slider::new(&mut value, range.clone()).logarithmic(true).show_value(false));
                center = resp.rect.center();
                rail = resp.rect;
            },
        );
        out.textures_delta.clear();
        values.push(value);
    }
    values
}

fn main() {
    for range in [-10.0..=10.0, -1.0..=1.0] {
        let name = format!("{range:?}");
        match std::panic::catch_unwind(|| drag_slider(range)) {
            Ok(values) => println!("logarithmic slider {name}: values per frame {values:?}"),
            Err(_) => println!("logarithmic slider {name}: PANIC"),
        }
    }
}
