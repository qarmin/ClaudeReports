use calamine::{open_workbook_auto, Reader};
use std::env;

fn main() {
    let path = env::args().nth(1).expect("path arg");
    let mut wb = open_workbook_auto(&path).expect("open");
    let names = wb.sheet_names();
    for name in names {
        match wb.worksheet_formula(&name) {
            Ok(range) => {
                println!("sheet={name} start={:?} end={:?}", range.start(), range.end());
                for (r, row) in range.rows().enumerate() {
                    for (c, v) in row.iter().enumerate() {
                        if !v.is_empty() {
                            println!("  [{r},{c}] = {v:?}");
                        }
                    }
                }
            }
            Err(e) => println!("sheet={name} ERROR {e:?}"),
        }
    }
}
