use comfy_table::Table;

fn main() {
    let _ = comfy_table::utils::arrangement::helper::count_border_columns(&Table::default(), usize::MAX);
}
