use brotli::SliceOffset;

fn main() {
    let slice_offset = SliceOffset(1usize, 0u32);
    let vec = vec![];
    let _ = <brotli::SliceOffset as brotli::interface::Unfreezable>::thaw(&slice_offset, &vec);
}
