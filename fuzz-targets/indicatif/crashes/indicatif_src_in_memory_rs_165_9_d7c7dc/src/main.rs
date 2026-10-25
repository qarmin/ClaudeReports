use indicatif::InMemoryTerm;

fn main() {
    let _ = <indicatif::InMemoryTerm as indicatif::TermLike>::write_line(&InMemoryTerm::new(1u16, 1u16), "\n\t");
}
