use console::Term;

fn main() {
    let term = Term::buffered_stderr();
    let _ = term.move_cursor_to(0usize, usize::MAX);
}
