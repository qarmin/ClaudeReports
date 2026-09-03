// Standalone repro for candidate 023: `atom_tree` (lofty/src/mp4/read/mod.rs), the helper that
// `mp4/ilst/write.rs::save_to_existing` uses to locate an existing `ilst` atom among `meta`'s
// children, cannot distinguish "no child matched `up_to` (`ilst`)" from "the first child (index
// 0) matched `up_to`" - both cases return index `0` (`found_idx` starts at `0` and a
// `saturating_sub(1)` on "never matched" also lands on `0`).
//
// When a real, valid `udta > meta` atom exists with real children but none of them is `ilst`
// (e.g. only a `hdlr` atom, an entirely ordinary "metadata container present but empty/using a
// non-iTunes scheme" layout), `save_to_existing` wrongly treats `tree[0]` (the real `hdlr` atom)
// as if it were the existing `ilst` atom, and silently overwrites/destroys it with the new tag
// data instead of appending the new `ilst` atom after `hdlr` as it should.
//
// This drives the real, public `Mp4File::read_from` / `set_ilst` / `save_to` entry points on a
// real file byte buffer built by hand (a crafted MP4/M4A file on disk is a genuine external
// artifact - see CONFIRMATION.md section 11 - there is no second participant to fake here). The
// verification is an independent raw-byte scan for the `hdlr` atom's distinctive
// "mdirappl"-handler payload, not a reuse of lofty's own MP4 atom reader.
//
// Exit code convention: 0 = did not reproduce (the real `hdlr` atom survived), non-zero =
// reproduced (the real `hdlr` atom was destroyed).

use lofty::config::{ParseOptions, WriteOptions};
use lofty::file::AudioFile;
use lofty::mp4::{Atom, AtomData, AtomIdent, Ilst, Mp4File};

use std::io::Cursor;

fn atom(fourcc: &[u8; 4], content: &[u8]) -> Vec<u8> {
    let len = (content.len() + 8) as u32;
    let mut out = Vec::with_capacity(content.len() + 8);
    out.extend_from_slice(&len.to_be_bytes());
    out.extend_from_slice(fourcc);
    out.extend_from_slice(content);
    out
}

/// A real, spec-shaped `hdlr` atom: predefined(4) + handler_type(4) + reserved(12) + name(1,
/// empty pascal-ish string) - with the "mdirappl" component-subtype text this crate itself
/// writes in `create_meta`, so this is exactly the kind of `hdlr` atom lofty's own writer
/// produces (not an artificial/degenerate one).
fn build_hdlr() -> Vec<u8> {
    let mut content = Vec::new();
    content.extend_from_slice(&[0u8; 4]); // version(1) + flags(3)
    content.extend_from_slice(&[0u8; 4]); // predefined
    content.extend_from_slice(b"mdir"); // handler type
    content.extend_from_slice(b"appl");
    content.extend_from_slice(&[0u8; 12]); // reserved
    content.push(0); // empty name
    atom(b"hdlr", &content)
}

/// Builds a `meta` atom (full, version/flags = 0) containing a real `hdlr` atom followed by a
/// real `free` atom - a plausible "metadata container scaffolded, but no iTunes `ilst` written
/// yet" layout - and crucially no `ilst` atom at all.
fn build_meta_no_ilst() -> Vec<u8> {
    let hdlr = build_hdlr();
    let free = atom(b"free", &[1, 2, 3, 4, 5, 6]);

    let mut meta_content = Vec::new();
    meta_content.extend_from_slice(&0u32.to_be_bytes()); // version + flags = 0 (full meta)
    meta_content.extend_from_slice(&hdlr);
    meta_content.extend_from_slice(&free);

    atom(b"meta", &meta_content)
}

fn build_mp4() -> Vec<u8> {
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

    let udta = atom(b"udta", &build_meta_no_ilst());

    let mut moov_content = Vec::new();
    moov_content.extend_from_slice(&trak);
    moov_content.extend_from_slice(&udta);
    let moov = atom(b"moov", &moov_content);

    let mut out = Vec::new();
    out.extend_from_slice(&ftyp);
    out.extend_from_slice(&moov);
    out
}

fn main() {
    let original = build_mp4();

    let mut read_cursor = Cursor::new(original.clone());
    let mut file = Mp4File::read_from(&mut read_cursor, ParseOptions::new())
        .expect("read crafted mp4 back through the real public entry point");

    let mut tag = Ilst::default();
    tag.insert(Atom::new(
        AtomIdent::Fourcc(*b"\xa9nam"),
        AtomData::UTF8("hello".to_string()),
    ));
    file.set_ilst(tag);

    let mut write_cursor = Cursor::new(original);
    file.save_to(&mut write_cursor, WriteOptions::default())
        .expect("save_to should not error");

    let written = write_cursor.into_inner();

    // Independent raw-byte oracle: search for the meta-level `hdlr` atom's distinctive
    // "mdirappl" handler-type payload directly in the written bytes, without going through
    // lofty's own MP4 atom reader at all.
    let meta_hdlr_survived = written
        .windows(b"mdirappl".len())
        .any(|w| w == b"mdirappl");

    if meta_hdlr_survived {
        println!("did not reproduce - meta's real `hdlr` atom (mdirappl) survived the write");
        std::process::exit(0);
    }

    eprintln!(
        "REPRODUCED - meta's real `hdlr` atom (mdirappl) was destroyed by the `ilst` write; \
         `save_to_existing` mistook it for an existing `ilst` atom that never existed"
    );
    std::process::exit(1);
}
