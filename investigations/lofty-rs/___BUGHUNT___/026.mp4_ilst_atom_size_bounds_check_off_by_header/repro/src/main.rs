// Standalone reproduction for candidate 026: a real MP4 (M4A) file, found by a corpus-driven
// `cargo fuzz` campaign against `mp4/ilst/write.rs`'s write path, whose top-level `moov` atom
// declares a size exactly `ATOM_HEADER_LEN` (8) bytes larger than what's physically present in
// the file. `AtomInfo::read_inner`'s bounds check (`lofty/src/mp4/atom_info.rs`) accepts this
// because it compares the atom's content-only size (`len - ATOM_HEADER_LEN`) against a budget
// (`reader_size`) that still includes the atom's own header - letting any atom's declared size
// overrun the real remaining bytes by up to `ATOM_HEADER_LEN`. `Mp4File::read_from` accepts the
// file with no error, but the inflated `meta` atom bounds this produces later make
// `mp4/ilst/write.rs::save_to_existing`'s "insert a new `ilst` atom" path compute an insertion
// offset (`meta.start + meta.len`) that lies past the physical end of the in-memory write
// buffer, and `Vec::splice`/`Vec::drain` panics on the out-of-range range.
//
// This drives the real, public `Mp4File::read_from` -> `set_ilst` -> `save_to` API end-to-end -
// exactly what any real caller adding/editing MP4 metadata on a slightly malformed/truncated
// file would do.
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present).

use std::io::Cursor;

use lofty::config::{ParseOptions, WriteOptions};
use lofty::file::AudioFile;
use lofty::mp4::{Atom, AtomData, AtomIdent, DataType, Ilst, Mp4File};

// The first byte selects which tag mutation to apply (found by the fuzzer alongside the file
// bytes); the rest is the real MP4/M4A file content.
static CRASH_INPUT: &[u8] = include_bytes!("../crash.bin");

fn main() {
	let control = CRASH_INPUT[0];
	let file_bytes = &CRASH_INPUT[1..];

	let mut cursor = Cursor::new(file_bytes.to_vec());
	let mut mp4 = match Mp4File::read_from(&mut cursor, ParseOptions::new()) {
		Ok(mp4) => mp4,
		Err(e) => {
			println!(
				"did not reproduce - suspect code path completed normally (read_from rejected the malformed file: {e:?})"
			);
			return;
		},
	};

	if control & 1 == 0 {
		let mut tag = Ilst::default();
		let n = usize::from(control >> 1) % 8;
		let len = file_bytes.len().max(1);
		for i in 0..n {
			let start = (i.saturating_mul(7)) % len;
			let end = (start + 24).min(file_bytes.len());
			let chunk = file_bytes.get(start..end).unwrap_or(&[]);
			let s = String::from_utf8_lossy(chunk).into_owned();

			let fourccs: [[u8; 4]; 4] = [*b"\xa9nam", *b"\xa9ART", *b"\xa9alb", *b"covr"];
			let fourcc = fourccs[i % fourccs.len()];

			if fourcc == *b"covr" {
				tag.insert(Atom::new(
					AtomIdent::Fourcc(fourcc),
					AtomData::Unknown {
						code: DataType::Other(13),
						data: chunk.to_vec(),
					},
				));
			} else {
				tag.insert(Atom::new(AtomIdent::Fourcc(fourcc), AtomData::UTF8(s)));
			}
		}
		mp4.set_ilst(tag);
	} else {
		mp4.set_ilst(Ilst::default());
	}

	let mut out = Cursor::new(file_bytes.to_vec());

	let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
		mp4.save_to(&mut out, WriteOptions::default())
	}));

	match result {
		Ok(Ok(())) => {
			println!("did not reproduce - save_to completed normally");
		},
		Ok(Err(e)) => {
			println!(
				"did not reproduce - save_to returned a graceful error instead of panicking: {e:?}"
			);
		},
		Err(panic_payload) => {
			eprintln!("REPRODUCED - panicked: {panic_payload:?}");
			std::process::exit(1);
		},
	}
}
