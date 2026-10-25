use krilla::document::Document;
use krilla::form::FieldGroup;
use krilla::form::FieldTree;
use krilla::form::Node;

fn main() {
    let mut document = Document::default();
    document.set_field_tree(FieldTree { fields: vec![Node::from(FieldGroup { name: ".".to_string(), fields: vec![] })] });
    let _ = document.finish();
}
