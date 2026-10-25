use vello_common::tile::CulledWindings;

fn main() {
    let mut culled_windings = CulledWindings::new(1u16);
    culled_windings.mark_row_range_active(0usize, 100000000000000000usize);
}
