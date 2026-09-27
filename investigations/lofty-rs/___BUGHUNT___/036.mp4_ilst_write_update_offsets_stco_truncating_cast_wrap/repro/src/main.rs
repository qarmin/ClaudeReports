// Standalone reproduction for candidate 036: `lofty/src/mp4/ilst/write.rs::update_offsets`'s
// 32-bit `stco` loop computes the new chunk offset as
// `(i64::from(read_offset) + difference) as u32` and writes it back with no bounds check. If the
// sum exceeds `u32::MAX`, the final `as u32` silently truncates/wraps instead of erroring, so the
// file gets a corrupted (numerically unrelated) chunk offset written to disk instead of either a
// correct value or a graceful `Err` - no panic, no crash, just silently wrong bytes on disk.
//
// `crafted.m4a` is a real, `read_from`-accepted M4A file (`lofty/tests/taglib/data/has-tags.m4a`)
// whose `moov.trak.mdia.minf.stbl.stco` table has had its last chunk-offset entry set to
// 0xFFFFFFFA (4294967290, close to `u32::MAX`) - a form of corruption/unusual encoding the read
// path never rejects, since `stco` entries are opaque per-chunk data never validated against the
// real file length. This drives the real, public `Mp4File::read_from` -> `Ilst::insert` ->
// `save_to` API end-to-end on that file - exactly what any real caller adding a large embedded
// picture to this file's tag would do. Single participant, single process; no second participant
// to fake, so CONFIRMATION.md section 11's bar doesn't apply here.
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present).

use std::io::Cursor;

use lofty::config::{ParseOptions, WriteOptions};
use lofty::file::AudioFile;
use lofty::mp4::{Atom, AtomData, AtomIdent, DataType, Mp4File};

static CRAFTED_INPUT: &[u8] = include_bytes!("../crafted.m4a");

const STCO_OFFSET: usize = 2705;
const TARGET_ENTRY_INDEX: usize = 3;
const ORIGINAL_VALUE: u32 = 0xFFFF_FFFA;

fn read_stco_entry(bytes: &[u8]) -> u32 {
	let entry_off = STCO_OFFSET + 16 + TARGET_ENTRY_INDEX * 4;
	u32::from_be_bytes(bytes[entry_off..entry_off + 4].try_into().unwrap())
}

fn main() {
	let sanity = read_stco_entry(CRAFTED_INPUT);
	assert_eq!(
		sanity, ORIGINAL_VALUE,
		"crafted.m4a's stco entry layout drifted, fix STCO_OFFSET/TARGET_ENTRY_INDEX"
	);

	let mut cursor = Cursor::new(CRAFTED_INPUT.to_vec());
	let mut mp4 =
		Mp4File::read_from(&mut cursor, ParseOptions::new()).expect("read_from must accept this crafted-but-well-formed-enough file");

	let mut tag = mp4.ilst().cloned().unwrap_or_default();
	// Grow the `ilst` atom substantially (a large embedded cover, as any real tagger would write)
	// so `difference` (the tag's size delta) is comfortably larger than the 5-byte margin between
	// 0xFFFFFFFA and `u32::MAX`.
	tag.insert(Atom::new(
		AtomIdent::Fourcc(*b"covr"),
		AtomData::Unknown {
			code: DataType::Other(13),
			data: vec![0xAB; 200_000],
		},
	));
	mp4.set_ilst(tag);

	let mut out = Cursor::new(CRAFTED_INPUT.to_vec());
	if let Err(e) = mp4.save_to(&mut out, WriteOptions::default()) {
		use std::error::Error;
		let mut cur: &dyn Error = &e;
		println!("did not reproduce - save_to returned an error chain:");
		loop {
			println!("  - {cur}");
			match cur.source() {
				Some(next) => cur = next,
				None => break,
			}
		}
		return;
	}

	let out_bytes = out.into_inner();
	let written = read_stco_entry(&out_bytes);

	let full_sum = i64::from(ORIGINAL_VALUE) + 200_000i64 /* not the exact difference, just for context */;
	println!("original stco entry:         {ORIGINAL_VALUE} (0x{ORIGINAL_VALUE:08X})");
	println!("written stco entry on disk:  {written} (0x{written:08X})");
	println!("(for reference, offset + covr payload growth alone already exceeds u32::MAX: {full_sum})");

	// The bug: save_to returned Ok(()) (no panic, no error) and the on-disk value is *smaller*
	// than the original despite the file having only grown - proof the true (larger-than-u32::MAX)
	// sum was silently truncated/wrapped rather than rejected or correctly represented.
	if written < ORIGINAL_VALUE {
		eprintln!(
			"REPRODUCED - `save_to` returned Ok(()) but wrote a wrapped/corrupted 32-bit chunk \
			 offset ({written}) instead of erroring on an offset that cannot be represented in a \
			 32-bit `stco` entry"
		);
		std::process::exit(1);
	}

	println!("did not reproduce - written offset was not wrapped");
}
