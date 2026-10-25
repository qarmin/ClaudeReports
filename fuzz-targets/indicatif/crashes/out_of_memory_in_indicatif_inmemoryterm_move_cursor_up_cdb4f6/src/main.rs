use indicatif::InMemoryTerm;
use indicatif::MultiProgress;

fn main() {
    let _ = MultiProgress::new();
    let _ = <indicatif::InMemoryTerm as indicatif::TermLike>::move_cursor_up(&InMemoryTerm::new(10000u16, 10000u16), 0usize);
}
