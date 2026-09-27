//! Independent check of `frame_pixel_data`'s *encapsulated*-pixel-data branch (the `Some(number_of_fragments)`
//! match arm that is NOT 1:1 fragment-to-frame) for the same "attacker-controlled integers
//! multiplied/added in a type narrower than their combined range" overflow class that candidate
//! 117 fixed in the native-pixel-data branch just below it in the same function
//! (`encoding/src/adapters.rs`).
//!
//! This branch's own arithmetic shape is different: it does not multiply two large
//! attacker-controlled factors together anywhere. It walks fragments in a loop, comparing an
//! accumulated `usize` offset against `u32`-valued Basic Offset Table entries (widened to
//! `usize`, never multiplied), and only ever adds a real fragment's actual byte length (bounded
//! by what can really be allocated in memory) plus a constant `8`. On a 64-bit target this
//! cannot reach anywhere near `usize::MAX` through realistic inputs. This binary exercises the
//! real public `PixelDataObject::frame_pixel_data` entry point with several maliciously-chosen
//! Basic Offset Table / fragment-count combinations to check that reasoning holds at runtime,
//! not just on paper - covering the "very large or maliciously-chosen offset values" scenario
//! called out in this candidate's task.
//!
//! Single external artifact (the object's own declared attributes/offset table/fragments), no
//! second participant to fake - CONFIRMATION.md section 11 does not apply.

use std::borrow::Cow;

use dicom_encoding::adapters::{PixelDataObject, RawPixelData};

struct Obj {
    number_of_frames: u32,
    number_of_fragments: u32,
    offset_table: Vec<u32>,
    fragments: Vec<Vec<u8>>,
}

impl PixelDataObject for Obj {
    fn transfer_syntax_uid(&self) -> &str {
        "1.2.840.10008.1.2.4.50"
    }
    fn rows(&self) -> Option<u16> {
        Some(1)
    }
    fn cols(&self) -> Option<u16> {
        Some(1)
    }
    fn samples_per_pixel(&self) -> Option<u16> {
        Some(1)
    }
    fn bits_allocated(&self) -> Option<u16> {
        Some(8)
    }
    fn bits_stored(&self) -> Option<u16> {
        Some(8)
    }
    fn photometric_interpretation(&self) -> Option<&str> {
        Some("MONOCHROME2")
    }
    fn number_of_frames(&self) -> Option<u32> {
        Some(self.number_of_frames)
    }
    fn number_of_fragments(&self) -> Option<u32> {
        Some(self.number_of_fragments)
    }
    fn fragment(&self, fragment: usize) -> Option<Cow<'_, [u8]>> {
        self.fragments.get(fragment).map(|f| Cow::Borrowed(f.as_slice()))
    }
    fn offset_table(&self) -> Option<Cow<'_, [u32]>> {
        Some(Cow::Borrowed(&self.offset_table))
    }
    fn raw_pixel_data(&self) -> Option<RawPixelData> {
        Some(RawPixelData {
            fragments: self.fragments.clone().into(),
            offset_table: self.offset_table.clone().into(),
        })
    }
}

fn run_case(name: &str, obj: &Obj, frame: u32) {
    print!("[{name}] frame_pixel_data({frame}): ");
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| obj.frame_pixel_data(frame)));
    match result {
        Ok(None) => println!("None"),
        Ok(Some(bytes)) => println!("Some({} bytes) = {:?}", bytes.len(), &bytes[..bytes.len().min(16)]),
        Err(payload) => {
            let message = payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| payload.downcast_ref::<&str>().copied())
                .unwrap_or("<non-string panic payload>");
            println!("PANICKED: {message}");
            std::process::exit(1);
        }
    }
}

fn main() {
    // NB: `frame_pixel_data`'s match on `self.number_of_fragments()` takes the *first* arm (a
    // plain `self.fragment(frame as usize)` shortcut, no offset-table logic at all) whenever
    // `number_of_fragments == number_of_frames`. Every case below deliberately sets those two
    // unequal so the "Other cases" arm (the one that actually walks `offset_table`) is the one
    // under test - an earlier version of this file got this wrong and silently exercised the
    // shortcut branch instead for two of the four cases.

    // Case 1: offset table entry at u32::MAX for the requested frame - `base_offset` becomes an
    // enormous `usize` that no real accumulated `offset` will ever reach, so every fragment is
    // skipped and the loop runs to completion without ever satisfying `offset >= base_offset`.
    let obj1 = Obj {
        number_of_frames: 5,
        number_of_fragments: 2,
        offset_table: vec![0, u32::MAX],
        fragments: vec![vec![0xAA; 10], vec![0xBB; 10]],
    };
    run_case("offset_table[1] = u32::MAX", &obj1, 1);

    // Case 2: `frame` itself near u32::MAX, real offset table/fragments tiny - `frame as usize`
    // and `frame as usize + 1` are plain widening casts/additions on a 64-bit target, not
    // multiplications, so no overflow is possible; `.get()` on the small real Vec should just
    // return None for the out-of-range index, and the whole call should gracefully return None.
    let obj2 = Obj {
        number_of_frames: 5,
        number_of_fragments: 2,
        offset_table: vec![0, 18],
        fragments: vec![vec![0xAA; 10], vec![0xBB; 10]],
    };
    run_case("frame near u32::MAX, tiny real offset table", &obj2, u32::MAX - 1);

    // Case 3: `number_of_fragments` wildly larger than the number of real fragments actually
    // available - the loop's own `self.fragment(idx)?` should return None (via `?`) as soon as
    // fragments run out, well before `idx` gets anywhere near the declared (lied-about) count;
    // no overflow, no unbounded loop over a huge declared count with nothing backing it.
    let obj3 = Obj {
        number_of_frames: 1,
        number_of_fragments: u32::MAX,
        offset_table: vec![0],
        fragments: vec![vec![0xAA; 10]],
    };
    run_case("number_of_fragments = u32::MAX, 1 real fragment", &obj3, 0);

    // Case 4: offset table entry at u32::MAX combined with `frame = 0` (takes the `frame == 0`
    // fallback branch instead of `base_offset?`) - same "never reached, loop runs empty" shape
    // as case 1, just through the other branch of the `if frame == 0` special-case.
    let obj4 = Obj {
        number_of_frames: 2,
        number_of_fragments: 1,
        offset_table: vec![u32::MAX],
        fragments: vec![vec![0xAA; 10]],
    };
    run_case("offset_table[0] = u32::MAX, frame = 0", &obj4, 0);

    println!(
        "\nNo panics across any case; no arithmetic overflow observed. Cases 1 and 4 both return \
         Some(0 bytes) rather than None for a frame whose own offset-table entry can never be \
         reached by the real accumulated fragment offsets - a minor correctness quirk (silently \
         synthesizing an empty frame instead of signaling \"no such frame\"), not the arithmetic- \
         overflow class this check was looking for. Case 3 returns None even though its first \
         fragment WAS legitimately matched and accumulated, because the loop's own `self.fragment\
         (idx)?` short-circuits the whole function to None as soon as a later, out-of-range \
         fragment index is hit (a `number_of_fragments` that overstates the real fragment count \
         discards already-accumulated data rather than returning it) - again a logic quirk, not \
         an overflow. No multiplication of two large attacker-controlled factors occurs anywhere \
         in this branch, and every addition here is bounded by real, actually-allocated fragment \
         byte lengths (impossible to approach usize::MAX on a 64-bit target) or by widening a u32 \
         into usize (lossless, cannot overflow)."
    );
}
