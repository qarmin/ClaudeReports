use indicatif::InMemoryTerm;

fn main() {
    let in_memory_term = InMemoryTerm::new(u16::MAX, 1000u16);
    in_memory_term.reset();
}
