use dicom_core::value::fragments::Fragments;
use dicom_core::value::{InMemFragment, PixelFragmentSequence};

fn main() {
    // f32 cannot exactly represent 16_777_217 (2^24 + 1) - the f32-based
    // fragment count computation rounds down, silently dropping the
    // trailing byte(s) of `data`. This is an entirely ordinary input size
    // for real pixel data (a ~16MB frame, e.g. a single 4096x4096 16-bit
    // grayscale image is 32MB).
    let len = 16_777_217usize;
    let data = vec![0xABu8; len];
    println!("input data length: {len}");

    let fragments = vec![Fragments::new(data, 2)];
    let seq: PixelFragmentSequence<InMemFragment> = fragments.into();
    let total: usize = seq.fragments().iter().map(|f| f.len()).sum();
    println!("total bytes covered by fragments: {total}");

    // fragment_size is rounded up to an even number (2) and data is padded to
    // a whole multiple of it, so the correct total is len rounded up to the
    // next even number.
    let expected = len.div_ceil(2) * 2;
    assert_eq!(
        total, expected,
        "fragments should cover every byte of input data (plus at most 1 padding byte); \
         got {total}, expected {expected} - {} byte(s) of pixel data were silently dropped",
        expected.saturating_sub(total)
    );
    println!("all input data accounted for, no truncation");
}
