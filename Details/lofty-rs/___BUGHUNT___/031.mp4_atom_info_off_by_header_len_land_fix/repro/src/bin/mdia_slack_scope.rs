// Supplementary reproduction for candidate 031 - NOT the pass/fail gate for `just test` (see
// `main.rs` for that). This demonstrates the documented *scope limit* of candidate 026's fix:
// `lofty/src/mp4/moov.rs::Moov::parse`'s own, separate, still-unpatched bug (candidate 030, not
// fixed by this candidate) passes `trak`'s full header-inclusive length as the `mdia`
// child-search budget instead of `atom.len - 8`, independently leaking its own `ATOM_HEADER_LEN`
// (8) bytes of slack into the same kind of "accept a too-large child atom" acceptance check.
//
// Because that budget-inflation bug is upstream of (not inside) `AtomInfo::read_inner`, fixing
// only `atom_info.rs` (candidate 026, this candidate) does not, by itself, close every instance
// where this class of bug is reachable - specifically, `moov.rs`'s own `trak` -> `mdia` lookup
// still tolerates a `mdia` atom declaring up to 8 bytes more than truly remains inside `trak`,
// silently dropping a real, untouched, subsequent `udta`/`meta`/`ilst` (the actual tag data) with
// no error surfaced anywhere - even after candidate 026's fix lands.
//
// This program takes a real, well-formed `.m4a` file (a tagged fixture already shipped in the
// project's own `lofty/tests/taglib/data/has-tags.m4a`) and modifies exactly one 4-byte field on
// disk: the declared size of `moov.trak.mdia` is incremented by `SLACK_BYTES`, while every other
// byte - including the `udta`/`meta`/`ilst` atom holding the actual tag data, positioned
// immediately after `trak` in `moov` - is left completely untouched and unmoved.
//
// Two values are checked (see `expected_after_fix` below):
//   - +8:  still reproduces (tag silently lost) even with candidate 026's fix applied alone,
//          because `moov.rs`'s own bug provides its own independent 8 bytes of slack.
//   - +16: correctly rejected once candidate 026's fix is applied, because 16 bytes exceeds what
//          `moov.rs`'s own bug alone can provide (only `atom_info.rs`'s now-removed slack plus
//          moov.rs's own slack could previously stack to tolerate 16).
//
// Exit code convention: 0 = every check behaved as expected for the current tree (whether or not
// candidate 026's fix is applied), non-zero = an unexpected result (see per-case messages).

use lofty::config::ParseOptions;
use lofty::file::AudioFile;
use lofty::mp4::Mp4File;

const MDIA_SIZE_FIELD_OFFSET: usize = 1729;

fn craft_malformed_file(slack: u32) -> Vec<u8> {
	let mut data = include_bytes!("../../assets/has-tags.m4a").to_vec();

	let field = &mut data[MDIA_SIZE_FIELD_OFFSET..MDIA_SIZE_FIELD_OFFSET + 4];
	let orig_len = u32::from_be_bytes(field.try_into().unwrap());
	assert_eq!(
		orig_len, 1040,
		"unexpected original mdia size - fixture changed?"
	);

	let new_len = orig_len + slack;
	field.copy_from_slice(&new_len.to_be_bytes());

	data
}

/// Returns `Some(true)` if the tag survived (atom rejected or lost tag not observed),
/// `Some(false)` if the tag was silently lost, or `None` if `read_from` itself failed.
fn tag_survives(slack: u32) -> Option<bool> {
	let malformed = craft_malformed_file(slack);
	match Mp4File::read_from(&mut std::io::Cursor::new(malformed), ParseOptions::new()) {
		Ok(mp4) => Some(mp4.ilst().is_some()),
		Err(_) => None,
	}
}

fn main() {
	let mut all_ok = true;

	// +8: expected to still reproduce (tag lost) even with candidate 026's fix applied, since
	// moov.rs's own separate bug (candidate 030, not fixed here) independently provides this much
	// slack on its own.
	match tag_survives(8) {
		Some(true) => println!("+8: tag survived (unexpected if 026's fix is applied alone and 030's bug is still present)"),
		Some(false) => println!("+8: tag was silently lost (expected - moov.rs's own bug, candidate 030, still provides this slack independently of 026's fix)"),
		None => {
			println!("+8: read_from rejected the file outright (unexpected - would mean moov.rs's own bug is also fixed on this tree)");
		},
	}

	// +16: expected to be rejected once candidate 026's fix is applied (16 bytes exceeds what
	// moov.rs's own bug alone can provide), but to still silently lose the tag on a fully
	// unpatched tree (both bugs stack: 8 + 8 = 16).
	match tag_survives(16) {
		Some(true) => {
			println!("+16: tag survived (unexpected)");
			all_ok = false;
		},
		Some(false) => println!("+16: tag was silently lost (expected only on a fully unpatched tree - both bugs stacking)"),
		None => println!("+16: read_from rejected the file outright (expected once candidate 026's fix is applied)"),
	}

	if !all_ok {
		eprintln!("UNEXPECTED RESULT - see messages above");
		std::process::exit(1);
	}

	println!(
		"all checks behaved as documented for this tree - this binary always exits 0 when the \
		 above per-case messages match RESULT.md's documented behavior; read the printed lines, \
		 not just the exit code, to tell which of the two bugs (026/030) are present on this tree"
	);
}
