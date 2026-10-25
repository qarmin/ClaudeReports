use allsorts::binary::read::ReadScope;
use allsorts::binary::read::ReadScopeOwned;

fn main() {
    let vec = vec![];
    let read_scope_owned = ReadScopeOwned::new(ReadScope::new(&vec));
    let read_scope = read_scope_owned.scope();
    let mut read_ctxt = read_scope.ctxt();
    let _ = read_ctxt.read_array::<allsorts::binary::I16Be>(10000000000000000000usize);
}
