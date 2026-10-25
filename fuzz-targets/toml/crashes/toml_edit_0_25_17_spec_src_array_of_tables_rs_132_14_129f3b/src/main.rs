use toml_edit::ArrayOfTables;

fn main() {
    let mut array_of_tables = ArrayOfTables::new();
    let _ = array_of_tables.remove(0usize);
}
