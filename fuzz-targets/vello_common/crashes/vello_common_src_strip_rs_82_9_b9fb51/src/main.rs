use vello_common::strip::StripFillSegment;

fn main() {
    let strip_fill_segment = StripFillSegment { tile_x0: 0u16, tile_x1: 0u16, tile_y: u16::MAX };
    let _ = strip_fill_segment.y();
}
