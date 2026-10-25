use indicatif::InMemoryTerm;

fn main() {
    let _ = <indicatif::InMemoryTerm as indicatif::TermLike>::write_str(&InMemoryTerm::new(1u16, 2u16), "👍🏽");
}
