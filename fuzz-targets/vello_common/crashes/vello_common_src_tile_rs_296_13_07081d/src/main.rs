use vello_common::tile::Tile;

fn main() {
    let _ = Tile::new_clamped(0u16, 0u16, 1000000000u32, 0u32);
}
