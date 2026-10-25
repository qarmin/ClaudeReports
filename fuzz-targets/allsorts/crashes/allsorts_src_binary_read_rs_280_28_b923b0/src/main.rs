use allsorts::binary::read::ReadScope;

fn main() {
    let vec = vec![];
    let read_scope = ReadScope::new(&vec);
    let read_scope2 = read_scope.offset(1usize);
    let _ = read_scope2.offset_length(usize::MAX, 0usize);
}
