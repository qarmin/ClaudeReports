use gimli::write::Expression;

fn main() {
    let mut expression = Expression::raw(vec![]);
    expression.set_target(1usize, 0usize);
}
