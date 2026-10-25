use valuable::NamedField;
use valuable::NamedValues;

fn main() {
    let string = "".to_string();
    let named_field = NamedField::new(&string);
    let vec = vec![];
    let vec2 = vec![];
    let named_values = NamedValues::new(&vec, &vec2);
    let _ = named_values.get(&named_field);
}
