use quick_xml::SeError;
use quick_xml::events::attributes::Attributes;
use quick_xml::name::NamespaceResolver;

fn main() {
    let se_error = SeError::Custom("\u{200b}".to_string());
    let string = se_error.to_string();
    let namespace_resolver = NamespaceResolver::default();
    let mut attributes = Attributes::html(&string, 1usize);
    let _ = attributes.has_nil(&namespace_resolver);
}
