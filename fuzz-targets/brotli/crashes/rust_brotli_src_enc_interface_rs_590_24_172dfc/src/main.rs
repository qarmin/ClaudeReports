use brotli::SliceOffset;

fn main() {
    let input_reference_mut = <brotli::SliceOffset as brotli::interface::Unfreezable>::thaw_mut(&SliceOffset(1usize, 0u32), &mut vec![]);
}
