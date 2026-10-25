use similar::DiffOp;
use similar::udiff::UnifiedHunkHeader;

fn main() {
    let _ = UnifiedHunkHeader::new(&[DiffOp::Insert { old_index: 1usize, new_index: usize::MAX, new_len: 1usize }]);
}
