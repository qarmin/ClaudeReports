use vello_common::tile::CulledWindings;

fn main() {
    let mut culled_windings = CulledWindings::default();
    culled_windings.mark_row_range_active(0usize, 1usize);
}
