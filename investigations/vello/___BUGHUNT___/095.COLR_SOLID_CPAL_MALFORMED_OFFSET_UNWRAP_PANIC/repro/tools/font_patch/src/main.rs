//! Patches the CPAL table's `colorRecordsArrayOffset` field to an out-of-bounds value in a copy
//! of the project's own COLRv1 test font, while leaving every `PaintSolid.paletteIndex` field
//! (still pointing at a normally-valid, in-range CPAL index) untouched.

use read_fonts::{FontRef, TableProvider};
use read_fonts::types::Tag;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = &args[1];
    let output_path = &args[2];

    let data = std::fs::read(input_path).unwrap();
    let font = FontRef::new(&data).unwrap();

    let base_ptr = data.as_ptr() as usize;

    let cpal_data = font.table_data(Tag::new(b"CPAL")).unwrap();
    let cpal_bytes = cpal_data.as_bytes();
    let cpal_offset_in_file = cpal_bytes.as_ptr() as usize - base_ptr;

    println!("CPAL table found at file offset {cpal_offset_in_file}, length {}", cpal_bytes.len());

    let cpal = font.cpal().unwrap();
    println!(
        "num_palette_entries={}, num_palettes={}, num_color_records={}, color_records_array_offset={:?}",
        cpal.num_palette_entries(),
        cpal.num_palettes(),
        cpal.num_color_records(),
        cpal.color_records_array_offset()
    );
    match cpal.color_records_array() {
        Some(Ok(records)) => println!("color_records_array resolves OK: {} records", records.len()),
        Some(Err(e)) => println!("color_records_array resolves to Err: {e:?}"),
        None => println!("color_records_array resolves to None (null offset)"),
    }

    // colorRecordsArrayOffset field is a big-endian u32 at byte offset 8 within the CPAL table
    // (after version:u16, numPaletteEntries:u16, numPalettes:u16, numColorRecords:u16).
    let field_offset_in_file = cpal_offset_in_file + 8;
    let original = u32::from_be_bytes(data[field_offset_in_file..field_offset_in_file + 4].try_into().unwrap());
    println!("original colorRecordsArrayOffset field bytes = {original}");

    // Patch it to a huge, clearly out-of-bounds offset (larger than the entire file), so
    // `color_records_array()` resolves to `Some(Err(ReadError::OutOfBounds))` instead of the
    // original valid slice.
    let mut out = data.clone();
    let bogus: u32 = 0x0FFF_FFFF;
    out[field_offset_in_file..field_offset_in_file + 4].copy_from_slice(&bogus.to_be_bytes());

    std::fs::write(output_path, &out).unwrap();
    println!("wrote patched font to {output_path} (colorRecordsArrayOffset patched to {bogus})");

    // Verify by re-parsing the patched bytes.
    let font2 = FontRef::new(&out).unwrap();
    let cpal2 = font2.cpal().unwrap();
    match cpal2.color_records_array() {
        Some(Ok(records)) => println!("VERIFY: patched color_records_array resolves OK ({} records) -- NOT what we wanted", records.len()),
        Some(Err(e)) => println!("VERIFY: patched color_records_array resolves to Err: {e:?} (as intended)"),
        None => println!("VERIFY: patched color_records_array resolves to None (as intended)"),
    }
}
