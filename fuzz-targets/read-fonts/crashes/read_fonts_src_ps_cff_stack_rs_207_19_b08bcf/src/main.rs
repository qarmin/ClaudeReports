use read_fonts::ps::cff::stack::Stack;

fn main() {
    let stack = Stack::default();
    let _ = stack.fixed_array::<4>(usize::MAX);
}
