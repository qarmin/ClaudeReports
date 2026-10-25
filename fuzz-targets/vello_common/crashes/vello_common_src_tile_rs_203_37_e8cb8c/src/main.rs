use vello_common::tile::CulledWindings;

fn main() {
    let culled_windings = CulledWindings::new(1u16);
    culled_windings.for_active_rows_in_range::<fn(usize)>(0usize, 1000000000usize, |_| ());
}
