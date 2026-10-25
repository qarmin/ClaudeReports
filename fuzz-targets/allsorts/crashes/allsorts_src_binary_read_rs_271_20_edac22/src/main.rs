use allsorts::binary::read::ReadScope;
use allsorts::binary::read::ReadScopeOwned;

fn main() {
    let vec = vec![];
    let read_scope = ReadScope::new(&vec);
    let read_scope_owned = ReadScopeOwned::new(read_scope.offset(1usize));
    let read_scope2 = read_scope_owned.scope();
    let _ = read_scope2.offset(usize::MAX);
}
