use indicatif::InMemoryTerm;

fn main() {
    let _ = <indicatif::InMemoryTerm as indicatif::TermLike>::flush(&InMemoryTerm::new(10000u16, 10000u16));
}
