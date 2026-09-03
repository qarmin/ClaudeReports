//! Standalone reproduction for: `SidxAtom::read` (symphonia-format-isomp4/src/atoms/sidx.rs:58-59)
//! computes the segment index's `first_offset` as `anchor + <first_offset read from the file>`
//! with a plain `+`:
//!
//! ```ignore
//! let anchor = header.pos() + atom_len.get();
//! let (earliest_pts, first_offset) = match version {
//!     0 => (u64::from(it.read_u32()?), anchor + u64::from(it.read_u32()?)),
//!     1 => (it.read_u64()?, anchor + it.read_u64()?),
//!     ...
//! ```
//!
//! `first_offset` is a raw, entirely unvalidated 32-/64-bit field read straight out of the file.
//! `AtomHeader::read` guards its own `atom_pos + atom_len` overflow (atoms/mod.rs:554), so `anchor`
//! itself is safe, but nothing guards this second addition.
//!
//! Both crafted files below are read through the real, public `IsoMp4Reader::try_new` entry point -
//! exactly what a player does when it opens an MP4 - and the crafted file is the only external
//! input (no second participant is being stood in for). `sidx` is a top-level atom handled directly
//! by `IsoMp4Reader::try_new`'s own atom loop (demuxer.rs:189-191), so no moov/moof is needed to
//! reach it.
//!
//! Case A - `assets/sidx_v1_first_offset_max.mp4`
//!   An ordinary, fully-bounded 40-byte version-1 `sidx` (declared size matches the real file size)
//!   whose 64-bit `first_offset` is `u64::MAX`. `anchor` is 56 here, so `56 + u64::MAX` overflows.
//!   Read from a plain seekable `File`, i.e. the completely normal path.
//!
//! Case B - `assets/sidx_v0_unknown_len.mp4`
//!   A version-0 `sidx` whose `first_offset` is only 1, but which declares a 64-bit atom size of
//!   `u64::MAX - 16` (the largest value `AtomHeader::read`'s own guard permits at atom position 16),
//!   making `anchor == u64::MAX`. Read from an unseekable stream (`ReadOnlySource`, what a caller
//!   piping from stdin/network gets), where the atom iterator has no container length to bound the
//!   declared atom size against. Included per CONFIRMATION.md section 7: a second input in the same
//!   bug class, hitting the version-0 arm from the opposite side of the addition.

use std::fs::File;
use std::io::Cursor;

use symphonia_core::formats::FormatOptions;
use symphonia_core::io::{MediaSourceStream, ReadOnlySource};
use symphonia_format_isomp4::IsoMp4Reader;

fn open_seekable(path: &str) {
    println!("=== {path} (seekable File) ===");
    let file = File::open(path).unwrap_or_else(|e| panic!("failed to open {path}: {e}"));
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    report(IsoMp4Reader::try_new(mss, FormatOptions::default()));
}

fn open_unseekable(path: &str) {
    println!("=== {path} (unseekable ReadOnlySource) ===");
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("failed to read {path}: {e}"));
    let src = ReadOnlySource::new(Cursor::new(bytes));
    let mss = MediaSourceStream::new(Box::new(src), Default::default());
    report(IsoMp4Reader::try_new(mss, FormatOptions::default()));
}

fn report(result: symphonia_core::errors::Result<IsoMp4Reader<'_>>) {
    match result {
        Ok(_) => println!("  IsoMp4Reader::try_new => Ok (unexpected: these files have no moov)"),
        Err(e) => println!(
            "  IsoMp4Reader::try_new => Err({e}) \
             (no panic: either the fix is applied and the offset was clamped/rejected, or \
             overflow-checks are off and the addition silently wrapped)"
        ),
    }
}

fn main() {
    let which = std::env::args().nth(1).unwrap_or_else(|| "all".to_string());

    if which == "all" || which == "ok" {
        // Control: the same file shape with a plausible first_offset. Must never panic.
        open_seekable("assets/sidx_v1_ok.mp4");
    }
    if which == "all" || which == "a" {
        open_seekable("assets/sidx_v1_first_offset_max.mp4");
    }
    if which == "all" || which == "b" {
        open_unseekable("assets/sidx_v0_unknown_len.mp4");
    }
}
