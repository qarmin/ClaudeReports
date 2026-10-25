use rav1e::prelude::BlockSize;

fn main() {
    let _ = rav1e::bench::rdo::clip_visible_bsize(0usize, 0usize, BlockSize::BLOCK_16X16, 0usize, usize::MAX);
}
