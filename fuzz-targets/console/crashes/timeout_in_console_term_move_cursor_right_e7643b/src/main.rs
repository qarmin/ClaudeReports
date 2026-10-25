use console::Term;

fn main() {
    let term = Term::stderr();
    let _ = term.move_cursor_right(1usize);
    let _ = term.clear_last_lines(4294967295usize);
}
