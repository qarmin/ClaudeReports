use object::macho::N_ALT_ENTRY;

fn main() {
    let _ = N_ALT_ENTRY.with_common_alignment(68u8);
}
