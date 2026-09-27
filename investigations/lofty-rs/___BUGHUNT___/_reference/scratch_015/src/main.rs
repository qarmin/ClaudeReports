use lofty::config::{ParseOptions, WriteOptions};
use lofty::file::AudioFile;
use lofty::iff::wav::WavFile;
use lofty::id3::v2::Id3v2Tag;
use lofty::tag::Accessor;

use std::io::{Cursor, Seek, SeekFrom};

fn main() {
    let base = std::fs::read(
        "/home/rafal/Projekty/Obce/lofty-rs/lofty/tests/taglib/data/empty.wav",
    )
    .unwrap();
    println!("base wav len: {}", base.len());

    let mut cursor = Cursor::new(base);

    // Step 1: write a small ID3v2 tag as the last chunk.
    {
        let mut f = WavFile::read_from(&mut cursor, ParseOptions::new()).unwrap();
        cursor.seek(SeekFrom::Start(0)).unwrap();

        let mut id3v2 = Id3v2Tag::default();
        id3v2.set_title(String::from("T"));
        f.set_id3v2(id3v2);
        f.save_to(&mut cursor, WriteOptions::default()).unwrap();
    }

    let after_step1_len = cursor.get_ref().len();
    println!("len after small tag write: {}", after_step1_len);

    // Step 2: write a MUCH larger ID3v2 tag, overwriting the existing small
    // one that sits as the very last chunk in the file.
    cursor.seek(SeekFrom::Start(0)).unwrap();
    {
        let mut f = WavFile::read_from(&mut cursor, ParseOptions::new()).unwrap();
        cursor.seek(SeekFrom::Start(0)).unwrap();

        let mut id3v2 = Id3v2Tag::default();
        id3v2.set_comment("X".repeat(200_000));
        f.set_id3v2(id3v2);

        println!("about to write large tag...");
        f.save_to(&mut cursor, WriteOptions::default()).unwrap();
    }

    let after_step2_len = cursor.get_ref().len();
    println!("len after large tag write: {}", after_step2_len);

    let declared = u32::from_le_bytes(cursor.get_ref()[4..8].try_into().unwrap());
    println!(
        "RIFF header declares size = {} (=> total {} bytes); actual physical file = {} bytes",
        declared,
        declared as u64 + 8,
        after_step2_len
    );
}
