use vello_common::strip::StripFillSegment;

fn main() {
    let strip_fill_segment = StripFillSegment { tile_x0: 0u16, tile_x1: u16::MAX, tile_y: 0u16 };
    let _ = strip_fill_segment.x1();
}
