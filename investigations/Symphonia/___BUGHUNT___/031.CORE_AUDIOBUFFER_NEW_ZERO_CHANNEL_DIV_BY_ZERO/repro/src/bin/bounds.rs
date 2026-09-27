//! Supplementary boundary-value check for candidate 031's fix (CONFIRMATION.md section 7): the
//! patched `AudioBuffer::new` must still reject an over-large capacity with the intended
//! "capacity too large" assertion for every non-zero channel count, and must only stop panicking
//! for the zero-channel case.
//!
//! Run with `cargo run --bin bounds`.

use std::panic::{self, AssertUnwindSafe};

use symphonia::core::audio::sample::Sample;
use symphonia::core::audio::{Audio, AudioBuffer, AudioSpec, Channels, Position};

fn check<S: Sample>(label: &str, channels: Channels, capacity: usize) {
    let spec = AudioSpec::new(44100, channels);

    let result = panic::catch_unwind(AssertUnwindSafe(move || {
        let buf = AudioBuffer::<S>::new(spec, capacity);
        (buf.num_planes(), buf.capacity())
    }));

    match result {
        Ok((planes, cap)) => println!("  {label}: ok (num_planes = {planes}, capacity = {cap})"),
        Err(_) => println!("  {label}: panicked"),
    }
}

fn main() {
    panic::set_hook(Box::new(|info| {
        let msg = info
            .payload()
            .downcast_ref::<&str>()
            .map(|s| (*s).to_string())
            .or_else(|| info.payload().downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "<non-string panic payload>".to_string());
        eprintln!("    [panic] {msg}");
    }));

    let mono = Channels::Positioned(Position::FRONT_LEFT);
    let stereo = Channels::Positioned(Position::FRONT_LEFT | Position::FRONT_RIGHT);

    println!("Over-large capacity with a non-zero channel count (must still panic):");
    check::<f32>("f32 stereo, capacity = usize::MAX", stereo.clone(), usize::MAX);
    check::<f32>("f32 stereo, capacity = usize::MAX / 8 + 1", stereo.clone(), usize::MAX / 8 + 1);
    check::<i32>("i32 mono,   capacity = usize::MAX / 4 + 1", mono.clone(), usize::MAX / 4 + 1);
    check::<f64>("f64 mono,   capacity = usize::MAX / 8 + 1", mono.clone(), usize::MAX / 8 + 1);
    check::<f32>("f32 24ch,   capacity = usize::MAX / 4", Channels::Discrete(24), usize::MAX / 4);

    println!("Ordinary capacity with a non-zero channel count (must still succeed):");
    check::<f32>("f32 stereo, capacity = 1024", stereo, 1024);
    check::<f32>("f32 mono,   capacity = 0", mono, 0);

    println!("Zero channel count (this candidate's case - must no longer divide by zero):");
    check::<f32>("f32 Channels::None,         capacity = 1024", Channels::None, 1024);
    check::<f32>("f32 Channels::Discrete(0),  capacity = 0", Channels::Discrete(0), 0);
    check::<f32>(
        "f32 Channels::Custom([]),   capacity = 1024",
        Channels::Custom(Vec::new().into_boxed_slice()),
        1024,
    );
    check::<f32>("f32 Channels::None,         capacity = usize::MAX", Channels::None, usize::MAX);
}
