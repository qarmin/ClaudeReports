use gimli::constants::DW_AT_upc_threads_scaled;
use gimli::constants::DW_FORM_ref8;
use gimli::read::AttributeSpecification;

fn main() {
    let _ = AttributeSpecification::new(DW_AT_upc_threads_scaled, DW_FORM_ref8, Some(0i64));
}
