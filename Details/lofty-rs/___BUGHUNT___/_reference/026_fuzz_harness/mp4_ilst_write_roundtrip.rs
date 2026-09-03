#![no_main]

use std::io::Cursor;

use libfuzzer_sys::fuzz_target;
use lofty::config::{ParseOptions, WriteOptions};
use lofty::file::AudioFile;
use lofty::mp4::{Atom, AtomData, AtomIdent, DataType, Ilst, Mp4File};

// Exercises the `mp4/ilst/write.rs` write path (udta/meta/ilst atom-size bookkeeping and
// `atom_tree`) end-to-end: parse a real file, mutate its `ilst` tag using content still
// derived from the same fuzzer-controlled bytes (so libFuzzer's coverage feedback can steer
// both the container layout and the new tag payload together), write it back, then parse the
// result again. Panics anywhere in this pipeline are the signal; a successful re-parse is not
// otherwise checked against the original content (no oracle for that here).
fuzz_target!(|data: &[u8]| {
	if data.len() < 2 {
		return;
	}

	let control = data[0];
	let file_bytes = &data[1..];

	let mut cursor = Cursor::new(file_bytes.to_vec());
	let Ok(mut mp4) = Mp4File::read_from(&mut cursor, ParseOptions::new()) else {
		return;
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
	if mp4.save_to(&mut out, WriteOptions::default()).is_err() {
		return;
	}

	let mut out2 = Cursor::new(out.into_inner());
	let _ = Mp4File::read_from(&mut out2, ParseOptions::new());
});
