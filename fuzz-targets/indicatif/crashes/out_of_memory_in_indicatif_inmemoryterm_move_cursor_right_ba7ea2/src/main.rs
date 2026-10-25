use indicatif::InMemoryTerm;

fn main() {
    let _ = <indicatif::InMemoryTerm as indicatif::TermLike>::move_cursor_right(&InMemoryTerm::new(10000u16, 10000u16), 0usize);
}
