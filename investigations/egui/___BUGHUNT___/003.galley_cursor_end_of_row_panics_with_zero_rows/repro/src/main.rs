use egui::{
    Color32, Context, Event, FontId, Key, Modifiers, RawInput, TextEdit,
    text::{CCursor, CCursorRange, LayoutJob},
};

fn frame(ctx: &Context, text: &mut String, max_rows: usize, events: Vec<Event>) -> egui::Id {
    let mut id = None;
    let mut out = ctx.run_ui(
        RawInput {
            events,
            focused: true,
            ..Default::default()
        },
        |ui| {
            let mut layouter = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
                let mut job = LayoutJob::simple(
                    buf.as_str().to_owned(),
                    FontId::default(),
                    Color32::WHITE,
                    wrap_width,
                );
                job.wrap.max_rows = max_rows;
                ui.fonts_mut(|f| f.layout_job(job))
            };
            let out = TextEdit::multiline(text).layouter(&mut layouter).id_salt("te").show(ui);
            out.response.request_focus();
            id = Some(out.response.id);
        },
    );
    out.textures_delta.clear();
    id.unwrap()
}

fn main() {
    let ctx = Context::default();
    let mut text = String::from("hello world");

    // max_rows = 1 is fine
    let id = frame(&ctx, &mut text, 1, vec![]);
    let mut st = TextEdit::load_state(&ctx, id).unwrap_or_default();
    st.cursor.set_char_range(Some(CCursorRange::one(CCursor::new(0))));
    TextEdit::store_state(&ctx, id, st);
    frame(&ctx, &mut text, 1, vec![]);
    let end = Event::Key {
        key: Key::End,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    };
    frame(&ctx, &mut text, 1, vec![end.clone()]);
    println!("max_rows=1: End key handled without panic");

    // max_rows = 0 leaves the galley with zero rows
    println!("max_rows=0: pressing End ...");
    frame(&ctx, &mut text, 0, vec![end]);
    println!("max_rows=0: End key handled without panic");
}
