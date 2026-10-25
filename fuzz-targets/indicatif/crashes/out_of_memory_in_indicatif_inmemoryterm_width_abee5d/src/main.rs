use indicatif::InMemoryTerm;

fn main() {
    let _ = <indicatif::InMemoryTerm as indicatif::TermLike>::width(&InMemoryTerm::new(10000u16, 10000u16));
}
