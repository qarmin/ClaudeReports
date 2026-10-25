use egui::{Context, Event, Key, Modifiers, RawInput, TextEdit, text::CCursor, text::CCursorRange};

fn frame(ctx: &Context, text: &mut String, events: Vec<Event>) -> egui::Id {
    let mut id = None;
    let mut out = ctx.run_ui(
        RawInput {
            events,
            focused: true,
            ..Default::default()
        },
        |ui| {
            let out = TextEdit::multiline(text).lock_focus(true).id_salt("te").show(ui);
            out.response.request_focus();
            id = Some(out.response.id);
        },
    );
    out.textures_delta.clear();
    id.unwrap()
}

fn cursor(ctx: &Context, id: egui::Id) -> Option<usize> {
    TextEdit::load_state(ctx, id)?.cursor.char_range().map(|r| r.primary.index.0)
}

fn main() {
    // Direct trait API
    {
        use egui::TextBuffer as _;
        let mut s = String::from("x\n  ");
        let mut c = CCursor::new(4);
        s.decrease_indentation(&mut c);
        println!("trait: text={s:?} cursor={} (expected 2)", c.index);
    }

    // End to end through TextEdit: Shift+Tab with the caret at the end of "x\n  "
    let ctx = Context::default();
    let mut text = String::from("x\n  ");
    let id = frame(&ctx, &mut text, vec![]);
    let mut st = TextEdit::load_state(&ctx, id).unwrap_or_default();
    st.cursor.set_char_range(Some(CCursorRange::one(CCursor::new(4))));
    TextEdit::store_state(&ctx, id, st);
    frame(&ctx, &mut text, vec![]);
    println!("before: text={text:?} cursor={:?}", cursor(&ctx, id));
    let ev = Event::Key {
        key: Key::Tab,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::SHIFT,
    };
    frame(&ctx, &mut text, vec![ev]);
    println!("after Shift+Tab: text={text:?} cursor={:?} (expected Some(2))", cursor(&ctx, id));
}
