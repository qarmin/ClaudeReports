use lopdf::Document;

fn main() {
    let mut document = Document::default();
    document.renumber_objects_with(0u32);
}
