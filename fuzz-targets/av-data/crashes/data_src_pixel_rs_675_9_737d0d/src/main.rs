use av_data::pixel::Chromaton;

fn main() {
    let chromaton = Chromaton { h_ss: 0u8, v_ss: 1u8, packed: false, depth: 1u8, shift: 0u8, comp_offs: 1u8, next_elem: 1u8 };
    let _ = chromaton.get_data_size(100usize, 1000000000000000000usize, 1000000000usize);
}
