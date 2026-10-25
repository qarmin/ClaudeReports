use av_data::pixel::Chromaton;

fn main() {
    let chromaton = Chromaton { h_ss: 0u8, v_ss: 0u8, packed: false, depth: 0u8, shift: 0u8, comp_offs: 0u8, next_elem: 0u8 };
    let _ = chromaton.get_linesize(0usize, 0usize);
}
