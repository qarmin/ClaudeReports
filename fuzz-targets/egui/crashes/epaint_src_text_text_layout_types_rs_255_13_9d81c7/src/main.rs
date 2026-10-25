use emath::Align;
use epaint::text::ByteIndex;
use epaint::text::LayoutJob;
use epaint::text::LayoutSection;
use epaint::text::TextFormat;
use epaint::text::TextWrapping;

fn main() {
    let layout_job = LayoutJob { text: "".to_string(), sections: vec![LayoutSection { leading_space: 0.0f32, byte_range: ByteIndex::from(1usize)..ByteIndex::ZERO, format: TextFormat::default() }], wrap: TextWrapping::truncate_at_width(0.0f32), first_row_min_height: 0.0f32, break_on_newline: false, halign: Align::Center, justify: false, round_output_to_gui: false, keep_trailing_whitespace: false };
    layout_job.debug_sanity_check();
}
