use lopdf::Document;
use lopdf::Object;

fn main() {
    let mut document = Document::default();
    let _ = document.add_object::<lopdf::Object>(Object::from(vec![]));
    document.renumber_objects_with(u32::MAX);
}
