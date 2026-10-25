use gimli::write::Expression;

fn main() {
    let mut expression = Expression::raw(vec![]);
    expression.set_target(0usize, 0usize);
}
