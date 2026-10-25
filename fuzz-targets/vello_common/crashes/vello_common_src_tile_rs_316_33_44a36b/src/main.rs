use vello_common::tile::Tile;

fn main() {
    let tile = Tile::SENTINEL;
    let _ = tile.prev_loc(&Tile { packed_winding_line_idx: 0u32, x: 0u16, y: u16::MAX });
}
