use object::write::StringTable;

fn main() {
    let mut string_table = StringTable::new_in_order(0u32);
    let _ = string_table.write::<Vec<u8>>(&mut vec![], 1u32);
}
