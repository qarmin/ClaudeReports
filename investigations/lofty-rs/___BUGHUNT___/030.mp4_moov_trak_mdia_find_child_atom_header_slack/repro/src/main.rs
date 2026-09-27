// Reproduction for candidate 030: `lofty/src/mp4/moov.rs`'s `Moov::parse` looks up `trak`'s
// `mdia` child via `find_child_atom(reader, atom.len, *b"mdia", ...)`, passing `trak`'s *full*,
// header-inclusive declared length as the search budget - even though the reader has already
// been seeked past `trak`'s own 8-byte header by the time this call runs. That leaks exactly
// `ATOM_HEADER_LEN` (8) bytes of extra slack into the acceptance check for the child `mdia`
// atom's declared size, on top of whatever slack `AtomInfo::read_inner`'s own, separate,
// already-documented off-by-`ATOM_HEADER_LEN` bounds check (candidate 026, not fixed here) leaks
// independently. The two stack: an unpatched checkout tolerates a lie of up to 16 bytes here, not
// just 8.
//
// This program takes a real, well-formed `.m4a` file (`assets/has-tags.m4a`, a tagged fixture
// already shipped in the project's own `lofty/tests/taglib/data/`), and modifies exactly one
// 4-byte field on disk: the declared size of `moov.trak.mdia` is incremented by 16, while every
// other byte - including the `udta`/`meta`/`ilst` atom holding the actual tag data, positioned
// immediately after `trak` in `moov` - is left completely untouched and unmoved. This +16 value
// is deliberately chosen to isolate *this* candidate's fix: it lands past the slack that
// candidate 026's separate, still-unpatched bug provides on its own (which tops out at +8), so
// only this candidate's own fix (patch.diff) - not 026's - determines whether it's rejected.
//
// Because of the header-inclusive-budget bug above, `find_child_atom` accepts this lie. The
// resulting oversized `mdia.len` is then used to skip forward past `trak`'s real end, landing
// inside `udta`'s own header. `Moov::parse`'s top-level `while let Ok(Some(atom)) = reader.next()`
// then tries to read moov's next child starting from that garbage, misaligned position, fails to
// recognize anything valid there, and silently stops - so `udta`/`ilst` (and anything after it)
// is dropped with no error at all. The whole tag is lost even though every byte of it is
// physically present and untouched in the file.
//
// Exit code convention: 0 = did not reproduce (fixed - the tag survives, or the malformed input
// is correctly rejected), non-zero = reproduced (the tag was silently lost with no error).

use lofty::config::ParseOptions;
use lofty::file::AudioFile;
use lofty::mp4::Mp4File;

use std::io::Cursor;

const MDIA_SIZE_FIELD_OFFSET: usize = 1729;
const SLACK_BYTES: u32 = 16;

fn craft_malformed_file() -> Vec<u8> {
    let mut data = include_bytes!("../assets/has-tags.m4a").to_vec();

    let field = &mut data[MDIA_SIZE_FIELD_OFFSET..MDIA_SIZE_FIELD_OFFSET + 4];
    let orig_len = u32::from_be_bytes(field.try_into().unwrap());
    assert_eq!(orig_len, 1040, "unexpected original mdia size - fixture changed?");

    let new_len = orig_len + SLACK_BYTES;
    field.copy_from_slice(&new_len.to_be_bytes());

    data
}

fn main() {
    let original = include_bytes!("../assets/has-tags.m4a").to_vec();
    let original_mp4 =
        Mp4File::read_from(&mut Cursor::new(original), ParseOptions::new()).expect("original file must parse");
    let original_has_tag = original_mp4.ilst().is_some();
    println!("original file: ilst tag present = {original_has_tag}");
    assert!(original_has_tag, "fixture is expected to have an ilst tag");

    let malformed = craft_malformed_file();
    let result = Mp4File::read_from(&mut Cursor::new(malformed), ParseOptions::new());

    match result {
        Ok(mp4) => {
            let has_tag = mp4.ilst().is_some();
            println!(
                "malformed file (mdia size +{SLACK_BYTES}, udta/ilst bytes untouched): read_from -> Ok, ilst tag present = {has_tag}"
            );

            if has_tag {
                println!("did not reproduce - tag survived the malformed mdia size");
                std::process::exit(0);
            }

            eprintln!(
                "REPRODUCED - tag was silently lost: the file parsed successfully (no error at \
                 all) but the untouched, still-present ilst tag data was never found because \
                 `find_child_atom`'s header-inclusive budget let a malformed `mdia` atom skip \
                 past `trak`'s real end into `udta`'s header, desyncing the rest of the moov \
                 parse."
            );
            std::process::exit(1);
        },
        Err(e) => {
            println!("malformed file: read_from -> Err: {e:?}");
            println!("did not reproduce - the malformed input was rejected instead of silently losing the tag");
            std::process::exit(0);
        },
    }
}
