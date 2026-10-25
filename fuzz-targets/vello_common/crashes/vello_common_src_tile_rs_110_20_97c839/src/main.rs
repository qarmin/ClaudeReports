use vello_common::tile::CulledWindings;

fn main() {
    let mut culled_windings = CulledWindings::default();
    culled_windings.mark_row_active(0usize);
}
