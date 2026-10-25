use ruzstd::fse::FSETable;

fn main() {
    let mut f_s_e_table = FSETable::new(0u8);
    let _ = f_s_e_table.build_from_probabilities(21u8, &[1i32]);
}
