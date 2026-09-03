use lofty::config::{ParseOptions, WriteOptions};
use lofty::file::AudioFile;
use lofty::mp4::Mp4File;
use lofty::mp4::Ilst;
use lofty::tag::Accessor;

use std::io::{Cursor, Seek, SeekFrom};
use std::panic::{self, AssertUnwindSafe};

// Offsets (big-endian u32 size fields) taken from manual atom-tree inspection of
// tests/taglib/data/has-tags.m4a:
//   moov @ 1489 size=3619
//   udta @ 2769 size=2339   (moov's LAST child; real remaining budget before it = 2339, exact fit)
//   meta @ 2777 size=2331   (udta's ONLY child; exact fit)
//   ilst @ 2822 size=644    (meta's 2nd child, after hdlr; real remaining budget before it = 644+1642(free)=2286)
const MOOV_SIZE_OFF: usize = 1489;
const UDTA_SIZE_OFF: usize = 2769;
const META_SIZE_OFF: usize = 2777;
const ILST_SIZE_OFF: usize = 2822;

fn patch_u32(data: &mut [u8], off: usize, delta: i64) {
    let cur = u32::from_be_bytes(data[off..off + 4].try_into().unwrap());
    let new = (cur as i64 + delta) as u32;
    data[off..off + 4].copy_from_slice(&new.to_be_bytes());
}

fn try_write(mut data: Vec<u8>, label: &str) {
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let mut cursor = Cursor::new(&mut data);
        let mut f = Mp4File::read_from(&mut cursor, ParseOptions::new())?;
        cursor.seek(SeekFrom::Start(0)).unwrap();

        let mut ilst = Ilst::new();
        ilst.set_artist(String::from("Mutated"));
        f.set_ilst(ilst);

        f.save_to(&mut cursor, WriteOptions::default())
    }));

    match result {
        Ok(Ok(())) => println!("{label}: OK (no panic, write succeeded)"),
        Ok(Err(e)) => println!("{label}: OK (no panic, write returned Err: {e})"),
        Err(_) => println!("{label}: *** PANIC ***"),
    }
}

fn main() {
    let base = std::fs::read("/home/rafal/Projekty/Obce/lofty-rs/lofty/tests/taglib/data/has-tags.m4a").unwrap();

    // Sanity: unmodified file must round-trip fine.
    try_write(base.clone(), "baseline (unmodified)");

    for delta in [0i64, 1, 2, 4, 7, 8, 9, 16] {
        let mut data = base.clone();
        patch_u32(&mut data, UDTA_SIZE_OFF, delta);
        try_write(data, &format!("udta size += {delta}"));
    }

    for delta in [0i64, 1, 2, 4, 7, 8, 9, 16] {
        let mut data = base.clone();
        patch_u32(&mut data, META_SIZE_OFF, delta);
        try_write(data, &format!("meta size += {delta}"));
    }

    for delta in [0i64, 1, 2, 4, 7, 8, 9, 16] {
        let mut data = base.clone();
        patch_u32(&mut data, ILST_SIZE_OFF, delta);
        try_write(data, &format!("ilst size += {delta}"));
    }

    // Combined: inflate moov itself too, to allow udta more room.
    for delta in [0i64, 1, 4, 8, 9, 16] {
        let mut data = base.clone();
        patch_u32(&mut data, MOOV_SIZE_OFF, -8); // shrink moov so udta overshoots it
        try_write(data, &format!("moov size -= 8 (delta unused: {delta})"));
    }
}
