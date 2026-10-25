use console::Term;

fn main() {
    let term = Term::buffered_stdout();
    let _ = term.clear_last_lines(1000000000usize);
}
