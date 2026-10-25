use allsorts::binary::read::ReadScope;

fn main() {
    let vec = vec![];
    let read_scope = ReadScope::new(&vec);
    let mut read_ctxt = read_scope.ctxt();
    let _ = read_ctxt.read_array_stride::<allsorts::binary::U8>(9usize, 10000000000000000000usize);
}
