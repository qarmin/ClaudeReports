// Standalone reproduction for candidate 027: a real MP4/M4A file (found by candidate 026's
// corpus-driven `cargo fuzz` campaign, in the 120s post-fix validation burst run against that
// candidate's `atom_info.rs` fix) whose `moov.udta.meta.ilst` tree contains a `co64` atom
// declaring a chunk offset far outside any offset that a real file of this size could contain
// (attacker/corruption-controlled `u64`, not validated against the file's real length).
//
// `lofty/src/mp4/ilst/write.rs::update_offsets` rewrites every `co64` chunk offset at or past the
// `ilst` atom by `difference` (how many bytes the file grew/shrank because of the tag rewrite)
// using `(read_offset as i64 + difference) as u64`. Reinterpreting an arbitrary `u64` as `i64`
// (rather than checking it fits) produces a value close to `i64::MAX`/`i64::MIN` for large inputs
// completely unrelated to `difference`'s realistic small magnitude, so the subsequent `+` overflows
// `i64` and panics in a debug-assertions build (`attempt to add with overflow`).
//
// This drives the real, public `Mp4File::read_from` -> `set_ilst`/`Ilst::default()` -> `save_to`
// API end-to-end - exactly what any real caller adding/editing MP4 metadata on this file would do.
// No second participant is involved (single input file, single process), so section 11 of
// CONFIRMATION.md doesn't require anything beyond this.
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present).

use std::io::Cursor;

use lofty::config::{ParseOptions, WriteOptions};
use lofty::file::AudioFile;
use lofty::mp4::{Atom, AtomData, AtomIdent, DataType, Ilst, Mp4File};

// The first byte selects which tag mutation to apply (as found by the fuzzer alongside the file
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
			println!("did not reproduce - save_to returned a graceful error instead of panicking: {e:?}");
		},
		Err(panic_payload) => {
			eprintln!("REPRODUCED - panicked: {panic_payload:?}");
			std::process::exit(1);
		},
	}
}
