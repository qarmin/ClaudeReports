use ron::de::Position;
use ron::de::Span;

fn main() {
    let span = Span { start: Position { line: 1usize, col: 10usize }, end: Position { line: 1usize, col: 1usize } };
    let _ = span.substring_exclusive("aaaaaaaaa");
}
