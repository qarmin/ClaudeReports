use console::Term;

fn main() {
    let term = Term::stdout();
    let _ = term.move_cursor_to(usize::MAX, 0usize);
}
