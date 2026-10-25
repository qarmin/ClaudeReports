use gimli::write::Expression;

fn main() {
    let mut expression = Expression::new();
    expression.set_target(0usize, 1usize);
}
