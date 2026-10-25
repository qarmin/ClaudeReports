use lopdf::Bookmark;
use lopdf::Document;

fn main() {
    let mut document = Document::new_from_prev(&Document::default());
    let _ = document.add_bookmark(Bookmark { children: vec![0u32], title: "".to_string(), format: 0u32, color: [0.0f32, 0.0f32, 0.0f32], page: (0u32, 1u16), id: 0u32 }, None);
    let _ = document.build_outline();
}
