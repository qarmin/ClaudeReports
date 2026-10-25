use der_parser::ber::BerObject;

fn main() {
    let ber_object = BerObject::from_seq(vec![]);
    let pretty_ber = ber_object.as_pretty(usize::MAX, 1usize);
    let vec = vec![];
    let ber_object2 = BerObject::from_int_slice(&vec);
    let _ = pretty_ber.next_indent(&ber_object2);
}
