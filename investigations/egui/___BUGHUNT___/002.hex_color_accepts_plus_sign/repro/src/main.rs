use ecolor::Color32;

fn main() {
    let mut bad = 0;
    for s in ["#+f0", "#+fff", "#+12345", "#+0000000", "#+f0f", "#-f0", "# f0"] {
        let r = Color32::from_hex(s);
        println!("{s:?} -> {r:?}");
        if r.is_ok() {
            bad += 1;
        }
    }
    println!("{bad} strings containing a non-hex '+' were accepted (expected 0)");
}
