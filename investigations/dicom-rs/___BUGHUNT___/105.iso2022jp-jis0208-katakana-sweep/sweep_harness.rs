// Not applied to the tree - this candidate found no bug, so there is no patch.diff.
// This is the exact temporary #[cfg(test)] harness that was added to
// encoding/src/text.rs (inside `mod tests`, next to `iso_ir_87_baseline`), run once via
// `cargo test -p dicom-encoding --lib iso_ir_87_jis0208_and_katakana_sweep -- --nocapture`,
// and then reverted with `git checkout -- encoding/src/text.rs`. Kept here for reference only.

#[test]
fn iso_ir_87_jis0208_and_katakana_sweep() {
    // Candidate 105 (___BUGHUNT___): sibling sweep to candidate 104's JIS X 0212
    // (`ESC $ ( D`) brute force, this time over the `ESC $ B` (JIS X 0208) and
    // `ESC ( I` (half-width Katakana) escape paths, whose encoder does have a
    // matching code path (unlike JIS X 0212) - checking whether the underlying
    // `index_japanese::jis0208::forward`/`backward` table pair is nonetheless
    // asymmetric anywhere within the byte ranges both decoder and encoder accept.
    let codec = SpecificCharacterSet(CharsetImpl::IsoIr87);

    let mut jis0208_asymmetric = Vec::new();
    for lead in 0x21u8..=0x7e {
        for trail in 0x21u8..=0x7e {
            let bytes = vec![0x1b, b'$', b'B', lead, trail];
            let decoded = match codec.decode(&bytes) {
                Ok(s) => s,
                Err(_) => continue,
            };
            if codec.encode(&decoded).is_err() {
                jis0208_asymmetric.push((lead, trail, decoded));
            }
        }
    }

    let mut katakana_asymmetric = Vec::new();
    for b in 0x21u8..=0x5f {
        let bytes = vec![0x1b, b'(', b'I', b];
        let decoded = match codec.decode(&bytes) {
            Ok(s) => s,
            Err(_) => continue,
        };
        if codec.encode(&decoded).is_err() {
            katakana_asymmetric.push((b, decoded));
        }
    }

    println!(
        "JIS X 0208 asymmetric pairs: {} / 8836",
        jis0208_asymmetric.len()
    );
    println!(
        "Katakana asymmetric bytes: {} / 63",
        katakana_asymmetric.len()
    );
    if let Some((lead, trail, ch)) = jis0208_asymmetric.first() {
        println!("first JIS X 0208 asymmetric pair: {lead:#x} {trail:#x} -> {ch:?}");
    }
    if let Some((b, ch)) = katakana_asymmetric.first() {
        println!("first Katakana asymmetric byte: {b:#x} -> {ch:?}");
    }

    assert!(
        jis0208_asymmetric.is_empty(),
        "found {} asymmetric JIS X 0208 pairs",
        jis0208_asymmetric.len()
    );
    assert!(
        katakana_asymmetric.is_empty(),
        "found {} asymmetric Katakana bytes",
        katakana_asymmetric.len()
    );
}
