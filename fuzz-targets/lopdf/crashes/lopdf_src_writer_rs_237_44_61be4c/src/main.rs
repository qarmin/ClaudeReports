use lopdf::Document;
use lopdf::IncrementalDocument;

fn main() {
    let mut document = Document::new();
    document.renumber_objects_with(u32::MAX);
    let mut incremental_document = IncrementalDocument::create_from(vec![], document.clone());
    let _ = incremental_document.save_to::<Vec<u8>>(&mut vec![]);
}
