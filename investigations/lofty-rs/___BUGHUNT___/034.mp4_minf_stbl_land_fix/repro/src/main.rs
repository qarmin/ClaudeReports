// Reproduction for candidate 034 (landing candidate 033's already-verified fix for real, the same
// way candidate 032 landed candidate 030's `moov.rs` fix): `lofty/src/mp4/properties.rs`'s
// `Minf::parse` looks up
// `minf`'s `stbl` child via `find_child_atom(reader, len, *b"stbl", ...)`, passing `minf`'s
// *full*, header-inclusive declared length (`minf_info.len`, unmodified from the caller) as the
// search budget - even though the reader has already been seeked 8 bytes forward, past `minf`'s
// own header, by the caller (`reader.seek(SeekFrom::Start(minf_info.start + 8))` in
// `read_properties`) before `Minf::parse` ever runs. That leaks exactly `ATOM_HEADER_LEN` (8)
// bytes of extra slack into the acceptance check for the child `stbl` atom's declared size, on
// top of whatever slack `AtomInfo::read_inner`'s own, separate, already-documented off-by-
// `ATOM_HEADER_LEN` bounds check (candidate 026, not fixed here) leaks independently. The two
// stack: an unpatched checkout tolerates a lie of up to 16 bytes here, not just 8 - the exact
// same shape candidate 030 already confirmed and fixed for `moov.rs`'s sibling `trak`->`mdia`
// call, examined here for the first time at this separate call site as flagged in candidate 030's
// own "what's not done" section.
//
// This program takes a real, well-formed `.m4a` file (`assets/has-tags.m4a`, a tagged fixture
// already shipped in the project's own `lofty/tests/taglib/data/`), and modifies exactly one
// 4-byte field on disk: the declared size of `moov.trak.mdia.minf.stbl` is incremented by 16,
// while every other byte - including the `udta`/`meta`/`ilst` atom holding the actual tag data,
// and the audio properties data inside `stbl` itself - is left completely untouched and unmoved.
// This +16 value is deliberately chosen to isolate *this* candidate's fix: it lands past the
// slack that candidate 026's separate, still-unpatched bug provides on its own (which tops out at
// +8), so only this candidate's own fix (patch.diff) - not 026's - determines whether it's
// rejected.
//
// Because of the header-inclusive-budget bug above, `find_child_atom` accepts this lie. In this
// fixture, `stbl` happens to be `minf`'s (and `mdia`'s, and `trak`'s) last real child, so the
// resulting oversized `stbl.len` makes `Minf::parse`'s own inner loop
// (`while read < stbl.len { ... }`) attempt to read one atom's worth of data past `stbl`'s real
// end - landing on the real, valid `udta` atom header that immediately follows in the file. That
// phantom "child of stbl" is skipped harmlessly here (it happens to consume exactly the bytes up
// to the real end of file), so `read_from` still returns `Ok` with the malformed `stbl.len`
// silently tolerated instead of being rejected as it should be - a strictly weaker file than the
// original was silently accepted as valid.
//
// Exit code convention: 0 = did not reproduce (fixed - the malformed input is correctly
// rejected), non-zero = reproduced (the malformed, oversized `stbl.len` was silently accepted).

use lofty::config::ParseOptions;
use lofty::file::AudioFile;
use lofty::mp4::Mp4File;

use std::io::Cursor;

const STBL_SIZE_FIELD_OFFSET: usize = 1862;
const SLACK_BYTES: u32 = 16;

fn craft_malformed_file() -> Vec<u8> {
    let mut data = include_bytes!("../assets/has-tags.m4a").to_vec();

    let field = &mut data[STBL_SIZE_FIELD_OFFSET..STBL_SIZE_FIELD_OFFSET + 4];
    let orig_len = u32::from_be_bytes(field.try_into().unwrap());
    assert_eq!(orig_len, 907, "unexpected original stbl size - fixture changed?");

    let new_len = orig_len + SLACK_BYTES;
    field.copy_from_slice(&new_len.to_be_bytes());

    data
}

fn main() {
    let original = include_bytes!("../assets/has-tags.m4a").to_vec();
    let original_mp4 =
        Mp4File::read_from(&mut Cursor::new(original), ParseOptions::new()).expect("original file must parse");
    println!(
        "original file: duration={:?} channels={:?}",
        original_mp4.properties().duration(),
        original_mp4.properties().channels()
    );

    let malformed = craft_malformed_file();
    let result = Mp4File::read_from(&mut Cursor::new(malformed), ParseOptions::new());

    match result {
        Ok(mp4) => {
            println!(
                "malformed file (stbl size +{SLACK_BYTES}, everything else untouched): read_from -> Ok, \
                 duration={:?} channels={:?}",
                mp4.properties().duration(),
                mp4.properties().channels()
            );

            eprintln!(
                "REPRODUCED - a `stbl` atom lying about its own size by {SLACK_BYTES} bytes past \
                 the real remaining space inside `minf` was silently accepted instead of \
                 rejected, because `find_child_atom`'s search budget for `stbl` within `minf` is \
                 `minf`'s full, header-inclusive declared length even though the reader has \
                 already been seeked past `minf`'s own 8-byte header by this point."
            );
            std::process::exit(1);
        },
        Err(e) => {
            println!("malformed file: read_from -> Err: {e:?}");
            println!("did not reproduce - the malformed input was rejected instead of being silently accepted");
            std::process::exit(0);
        },
    }
}
