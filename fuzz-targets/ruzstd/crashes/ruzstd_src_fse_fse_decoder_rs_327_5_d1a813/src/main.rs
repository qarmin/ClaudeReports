use ruzstd::fse::FSETable;

fn main() {
    let mut f_s_e_table = FSETable::new(1u8);
    let _ = f_s_e_table.build_from_probabilities(1u8, &[1000i32]);
}
