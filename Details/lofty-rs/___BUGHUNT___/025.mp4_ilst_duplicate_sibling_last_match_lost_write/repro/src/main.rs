// Standalone repro for candidate 025: `atom_tree` (lofty/src/mp4/read/mod.rs), the helper
// `mp4/ilst/write.rs::save_to_existing` uses to locate an existing `ilst` atom among `meta`'s
// children, resolves to the LAST child matching fourcc `"ilst"` (`found_idx = i` is overwritten
// on every match in the loop, never gated on "not already found").
//
// The read path (`lofty/src/mp4/moov.rs`, the loop right before `parse_ilst`) instead breaks on
// the FIRST child matching `"ilst"`.
//
// When a real (malformed but well-formed-atom-wise) `meta` atom has more than one real `ilst`
// child as siblings, this mismatch means `save_to_existing` overwrites the LAST one, while every
// future read of the file (including the very next `Mp4File::read_from` after the write that
// "changed" the tag) still returns the FIRST one, unchanged. The user's tag edit is silently
// discarded: `save_to` reports success, but the change never becomes visible again.
//
// This drives the real, public `Mp4File::read_from` / `set_ilst` / `save_to` entry points on a
// real file byte buffer built by hand (a crafted MP4/M4A file on disk is a genuine external
// artifact - see CONFIRMATION.md section 11 - there is no second participant to fake here). The
// two `ilst` atoms spliced into the crafted file are themselves real bytes produced by the
// crate's own writer (`real_ilst_atom_bytes`), not hand-encoded iTunes atom-data byte layouts.
//
// Exit code convention: 0 = did not reproduce (reread title is "NEW"), non-zero = reproduced
// (reread title is still the stale "FIRST").

use lofty::config::{ParseOptions, WriteOptions};
use lofty::file::AudioFile;
use lofty::mp4::{Atom, AtomData, AtomIdent, Ilst, Mp4File};
use lofty::tag::Accessor;

use std::io::Cursor;

fn atom(fourcc: &[u8; 4], content: &[u8]) -> Vec<u8> {
    let len = (content.len() + 8) as u32;
    let mut out = Vec::with_capacity(content.len() + 8);
    out.extend_from_slice(&len.to_be_bytes());
    out.extend_from_slice(fourcc);
    out.extend_from_slice(content);
    out
}

fn build_hdlr() -> Vec<u8> {
    let mut content = Vec::new();
    content.extend_from_slice(&[0u8; 4]);
    content.extend_from_slice(&[0u8; 4]);
    content.extend_from_slice(b"mdir");
    content.extend_from_slice(b"appl");
    content.extend_from_slice(&[0u8; 12]);
    content.push(0);
    atom(b"hdlr", &content)
}

fn build_mp4_with_meta(meta_content: &[u8]) -> Vec<u8> {
    let mut ftyp_content = Vec::new();
    ftyp_content.extend_from_slice(b"M4A ");
    ftyp_content.extend_from_slice(&0u32.to_be_bytes());
    let ftyp = atom(b"ftyp", &ftyp_content);

    let mut mdhd_content = Vec::new();
    mdhd_content.extend_from_slice(&0u32.to_be_bytes());
    mdhd_content.extend_from_slice(&0u32.to_be_bytes());
    mdhd_content.extend_from_slice(&0u32.to_be_bytes());
    mdhd_content.extend_from_slice(&1000u32.to_be_bytes());
    mdhd_content.extend_from_slice(&0u32.to_be_bytes());
    let mdhd = atom(b"mdhd", &mdhd_content);

    let mut trak_hdlr_content = Vec::new();
    trak_hdlr_content.extend_from_slice(&[0u8; 8]);
    trak_hdlr_content.extend_from_slice(b"soun");
    let trak_hdlr = atom(b"hdlr", &trak_hdlr_content);

    let stsd = atom(b"stsd", &[0, 0, 0, 0, 0, 0, 0, 0]);
    let stbl = atom(b"stbl", &stsd);
    let minf = atom(b"minf", &stbl);

    let mut mdia_content = Vec::new();
    mdia_content.extend_from_slice(&mdhd);
    mdia_content.extend_from_slice(&trak_hdlr);
    mdia_content.extend_from_slice(&minf);
    let mdia = atom(b"mdia", &mdia_content);

    let trak = atom(b"trak", &mdia);

    let mut meta_full = Vec::new();
    meta_full.extend_from_slice(&0u32.to_be_bytes()); // version+flags = 0 (full meta)
    meta_full.extend_from_slice(meta_content);
    let meta = atom(b"meta", &meta_full);

    let udta = atom(b"udta", &meta);

    let mut moov_content = Vec::new();
    moov_content.extend_from_slice(&trak);
    moov_content.extend_from_slice(&udta);
    let moov = atom(b"moov", &moov_content);

    let mut out = Vec::new();
    out.extend_from_slice(&ftyp);
    out.extend_from_slice(&moov);
    out
}

/// Builds a real on-disk `ilst` atom (via the crate's own writer) for a single-title tag,
/// by writing it into a fresh minimal file (meta with only a `hdlr` child, no ilst) and then
/// extracting the resulting `ilst` atom's exact bytes back out of the written file.
fn real_ilst_atom_bytes(title: &str) -> Vec<u8> {
    let base = build_mp4_with_meta(&build_hdlr());

    let mut read_cursor = Cursor::new(base.clone());
    let mut file = Mp4File::read_from(&mut read_cursor, ParseOptions::new()).unwrap();

    let mut tag = Ilst::default();
    tag.insert(Atom::new(
        AtomIdent::Fourcc(*b"\xa9nam"),
        AtomData::UTF8(title.to_string()),
    ));
    file.set_ilst(tag);

    let mut write_cursor = Cursor::new(base);
    file.save_to(&mut write_cursor, WriteOptions::default())
        .unwrap();
    let written = write_cursor.into_inner();

    // Find "ilst" fourcc, back up 4 bytes to the length prefix, then slice out the full atom.
    let pos = written
        .windows(4)
        .position(|w| w == b"ilst")
        .expect("ilst atom must be present");
    let atom_start = pos - 4;
    let len = u32::from_be_bytes(written[atom_start..atom_start + 4].try_into().unwrap()) as usize;
    written[atom_start..atom_start + len].to_vec()
}

fn main() {
    let ilst_first = real_ilst_atom_bytes("FIRST");
    let ilst_second = real_ilst_atom_bytes("SECOND");

    println!(
        "built real ilst atoms: first={} bytes, second={} bytes",
        ilst_first.len(),
        ilst_second.len()
    );

    // Malformed-but-plausible layout: `meta` with a real `hdlr` followed by TWO real, distinct
    // `ilst` atoms as siblings (duplicate ilst atoms under one meta).
    let mut meta_content = Vec::new();
    meta_content.extend_from_slice(&build_hdlr());
    meta_content.extend_from_slice(&ilst_first);
    meta_content.extend_from_slice(&ilst_second);

    let original = build_mp4_with_meta(&meta_content);

    // Step 1: read the crafted file and confirm the *first* ilst ("FIRST") is what's parsed,
    // matching moov.rs's read logic (breaks on the first `ilst` match).
    let mut read_cursor = Cursor::new(original.clone());
    let mut file = Mp4File::read_from(&mut read_cursor, ParseOptions::new())
        .expect("read crafted mp4 with duplicate ilst siblings");

    let initial_title = file.ilst().and_then(|i| i.title()).map(|s| s.to_string());
    println!("initial parsed title (should be FIRST): {:?}", initial_title);

    // Step 2: set a brand new tag and save.
    let mut new_tag = Ilst::default();
    new_tag.insert(Atom::new(
        AtomIdent::Fourcc(*b"\xa9nam"),
        AtomData::UTF8("NEW".to_string()),
    ));
    file.set_ilst(new_tag);

    let mut write_cursor = Cursor::new(original);
    file.save_to(&mut write_cursor, WriteOptions::default())
        .expect("save_to should not error");
    let written = write_cursor.into_inner();

    // Step 3: re-read the written bytes through a completely fresh `Mp4File::read_from` call,
    // simulating the file being closed and reopened by any real caller.
    let mut reread_cursor = Cursor::new(written.clone());
    let reread_file = Mp4File::read_from(&mut reread_cursor, ParseOptions::new())
        .expect("re-read written mp4");

    let reread_title = reread_file
        .ilst()
        .and_then(|i| i.title())
        .map(|s| s.to_string());
    println!("title after save+reread (should be NEW): {:?}", reread_title);

    let new_appears_in_bytes = written.windows(3).any(|w| w == b"NEW");
    let first_survives = written.windows(5).any(|w| w == b"FIRST");
    let second_survives = written.windows(6).any(|w| w == b"SECOND");
    println!(
        "raw byte scan: contains 'NEW'={}, contains 'FIRST'={}, contains 'SECOND'={}",
        new_appears_in_bytes, first_survives, second_survives
    );

    if reread_title.as_deref() == Some("NEW") {
        println!("did not reproduce - the new tag was correctly picked up on reread");
        std::process::exit(0);
    }

    eprintln!(
        "REPRODUCED - after set_ilst(\"NEW\") + save_to, re-reading the file still returns the \
         STALE title {:?} instead of \"NEW\" - the write silently updated the wrong (ignored) \
         `ilst` sibling",
        reread_title
    );
    std::process::exit(1);
}
