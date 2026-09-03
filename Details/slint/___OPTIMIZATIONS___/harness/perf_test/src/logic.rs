use std::cell::Cell;
use std::rc::Rc;
use std::time::Instant;

use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};

use crate::generator;
use crate::{Logic, MainWindow, RecordItem};

/// Connects the UI callbacks to the content generators. Shared by the windowed app
/// and the headless benchmark so both measure exactly the same code.
pub fn wire_up(app: &MainWindow) -> Rc<VecModel<RecordItem>> {
    wire_up_with_line_length(app, generator::LINE_LEN)
}

/// `line_length` only affects the `text` mode's generated content.
pub fn wire_up_with_line_length(app: &MainWindow, line_length: usize) -> Rc<VecModel<RecordItem>> {
    let records = Rc::new(VecModel::<RecordItem>::default());
    app.set_records(ModelRc::from(records.clone()));

    // Only used to keep generated lines unique, so it may drift from the real
    // line count when the text is edited by hand - that is fine.
    let generated_lines = Rc::new(Cell::new(0usize));
    // Characters per generated line; see `push_line_of`.
    let line_len = Rc::new(Cell::new(line_length));

    let logic = app.global::<Logic>();

    logic.on_add_lines({
        let weak = app.as_weak();
        let generated_lines = generated_lines.clone();
        let line_len = line_len.clone();
        move |count| {
            let app = weak.unwrap();
            let count = count.max(0) as usize;

            let build_start = Instant::now();
            let mut text = String::from(app.get_text_content().as_str());
            text.reserve(count * (line_len.get() + 24));
            let first = generated_lines.get();
            for index in first..first + count {
                generator::push_line_of(&mut text, index, line_len.get());
            }
            generated_lines.set(first + count);
            let bytes = text.len();
            let build = build_start.elapsed();

            let apply_start = Instant::now();
            app.set_text_content(text.into());
            let apply = apply_start.elapsed();

            app.set_status(
                format!(
                    "+{count} lines: generated in {build:.2?}, property set in {apply:.2?} - buffer {} lines / {:.2} MB (layout and rendering happen on the next frame)",
                    generated_lines.get(),
                    bytes as f64 / 1_000_000.0
                )
                .into(),
            );
        }
    });

    logic.on_clear_text({
        let weak = app.as_weak();
        let generated_lines = generated_lines.clone();
        move || {
            let app = weak.unwrap();
            generated_lines.set(0);
            let start = Instant::now();
            app.set_text_content(SharedString::new());
            app.set_status(format!("text cleared in {:.2?}", start.elapsed()).into());
        }
    });

    logic.on_add_records({
        let weak = app.as_weak();
        let records = records.clone();
        move |count| {
            let count = count.max(0) as usize;
            let first = records.row_count();

            let start = Instant::now();
            records.extend((first..first + count).map(generator::make_record));
            let elapsed = start.elapsed();

            weak.unwrap().set_status(
                format!(
                    "+{count} records: model filled in {elapsed:.2?} - {} records total (layout and rendering happen on the next frame)",
                    records.row_count()
                )
                .into(),
            );
        }
    });

    logic.on_clear_records({
        let weak = app.as_weak();
        let records = records.clone();
        move || {
            let start = Instant::now();
            records.clear();
            weak.unwrap().set_status(format!("records cleared in {:.2?}", start.elapsed()).into());
        }
    });

    logic
        .on_selection_info(|text, cursor, anchor| generator::selection_info(&text, cursor, anchor));

    records
}
