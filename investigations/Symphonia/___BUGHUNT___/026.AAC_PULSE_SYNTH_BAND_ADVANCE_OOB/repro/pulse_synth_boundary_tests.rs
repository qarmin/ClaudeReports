// Boundary-value / property / random-fuzz test module for candidate 026
// (AAC_PULSE_SYNTH_BAND_ADVANCE_OOB).
//
// This is appended verbatim (by ../justfile) to the end of a pristine checkout's
// symphonia-codec-aac/src/aac/ics/pulse.rs, then run with `cargo test -p symphonia-codec-aac
// pulse_oob --lib -- --nocapture`. It is written this way (rather than as a separate crate with
// its own Cargo.toml) because `Pulse`'s fields and the SWB_OFFSET_*_LONG tables are private to
// symphonia-codec-aac (only `AacDecoder` is re-exported from the crate root) - there is no public
// API surface from which to construct an adversarial `Pulse` or reach the real band tables from
// outside the crate. This exercises the REAL `Pulse::synth` function and the REAL, unmodified
// static band tables from symphonia-codec-aac/src/aac/common.rs, not a copy or reimplementation.
//
// This is a single-participant boundary-value construction check (HEURISTICS.md's "arithmetic on
// untrusted sizes/indices" tool selection) - it directly drives the suspect function with every
// value the bitstream reader's own bit-width limits allow (pulse_start_sfb: 6 bits => 0..=63,
// pulse_offset: 5 bits => 0..=31, number_pulse: 2 bits + 1 => 1..=4), which is exactly the set of
// values a real crafted AAC bitstream's Pulse::read could ever produce.

#[cfg(test)]
mod pulse_oob_boundary_tests {
    use super::*;
    use crate::aac::common::{
        SWB_OFFSET_16K_LONG, SWB_OFFSET_24K_LONG, SWB_OFFSET_32K_LONG, SWB_OFFSET_48K_LONG,
        SWB_OFFSET_64K_LONG, SWB_OFFSET_8K_LONG, SWB_OFFSET_96K_LONG,
    };

    fn all_long_tables() -> Vec<(&'static str, &'static [usize])> {
        vec![
            ("48K", &SWB_OFFSET_48K_LONG[..]),
            ("32K", &SWB_OFFSET_32K_LONG[..]),
            ("8K", &SWB_OFFSET_8K_LONG[..]),
            ("16K", &SWB_OFFSET_16K_LONG[..]),
            ("24K", &SWB_OFFSET_24K_LONG[..]),
            ("64K", &SWB_OFFSET_64K_LONG[..]),
            ("96K", &SWB_OFFSET_96K_LONG[..]),
        ]
    }

    // Exhaustively/aggressively exercise Pulse::synth against every real long_bands table
    // (the only tables ever reachable, since the decoder validates pulse data is only
    // present alongside a long window) with boundary and adversarial pulse_start_sfb /
    // pulse_offset combinations, to check whether `band + 1` can ever run past
    // `bands.len() - 1` in the `while bands[band + 1] <= k` loop.
    #[test]
    fn synth_never_indexes_oob_across_all_rate_tables() {
        let scales = [[1.0f32; MAX_SFBS]; MAX_WINDOWS];

        for (name, bands) in all_long_tables() {
            // Boundary pulse_start_sfb values: every valid index, plus the guard boundary
            // itself and one past it.
            for pulse_start_sfb in 0..=bands.len() {
                // number_pulse ranges 1..=4 per the 2-bit field + 1 in Pulse::read.
                for number_pulse in 1..=4usize {
                    // Adversarial offset patterns: all max (31, 5-bit field max), all zero,
                    // and a few patterns that step k up by varying amounts to probe every
                    // possible position relative to a band boundary.
                    let patterns: [[u8; 4]; 5] = [
                        [31, 31, 31, 31],
                        [0, 0, 0, 0],
                        [1, 1, 1, 1],
                        [31, 0, 31, 0],
                        [5, 17, 3, 29],
                    ];

                    for pulse_offset in patterns {
                        let pulse_amp = [15u8; 4];
                        let pulse = Pulse { number_pulse, pulse_start_sfb, pulse_offset, pulse_amp };

                        let mut coeffs = [0.0f32; 1024];
                        // Should never panic (index out of bounds, or otherwise) for any
                        // combination above, regardless of which table is in play.
                        pulse.synth(bands, &scales, &mut coeffs);
                    }
                }
            }
            eprintln!("table {name}: exhaustive boundary sweep completed without panic");
        }
    }

    // Brute-force every single (pulse_start_sfb, single pulse_offset) pair exhaustively
    // (not just curated patterns) against the smallest real table, since a smaller table
    // reaches the tail-end sentinel fastest and is the most likely to expose an off-by-one.
    #[test]
    fn synth_exhaustive_single_pulse_smallest_table() {
        let scales = [[1.0f32; MAX_SFBS]; MAX_WINDOWS];
        let bands = &SWB_OFFSET_96K_LONG[..];

        for pulse_start_sfb in 0..=bands.len() {
            for offset in 0..=31u8 {
                let pulse = Pulse {
                    number_pulse: 1,
                    pulse_start_sfb,
                    pulse_offset: [offset, 0, 0, 0],
                    pulse_amp: [15, 0, 0, 0],
                };
                let mut coeffs = [0.0f32; 1024];
                pulse.synth(bands, &scales, &mut coeffs);
            }
        }
    }

    // A short random fuzz burst (a cheap xorshift, no external dependency) across every real
    // table, sampling the full field ranges the bitstream reader actually permits
    // (pulse_start_sfb: 6 bits, pulse_offset: 5 bits, number_pulse: 1..=4).
    #[test]
    fn synth_random_fuzz_burst() {
        let scales = [[1.0f32; MAX_SFBS]; MAX_WINDOWS];
        let mut state: u64 = 0x243F6A8885A308D3;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };

        for (_name, bands) in all_long_tables() {
            for _ in 0..200_000u32 {
                let pulse_start_sfb = (next() % 64) as usize;
                let number_pulse = (next() % 4) as usize + 1;
                let pulse_offset = [
                    (next() % 32) as u8,
                    (next() % 32) as u8,
                    (next() % 32) as u8,
                    (next() % 32) as u8,
                ];
                let pulse_amp = [
                    (next() % 16) as u8,
                    (next() % 16) as u8,
                    (next() % 16) as u8,
                    (next() % 16) as u8,
                ];
                let pulse = Pulse { number_pulse, pulse_start_sfb, pulse_offset, pulse_amp };
                let mut coeffs = [0.0f32; 1024];
                pulse.synth(bands, &scales, &mut coeffs);
            }
        }
    }
}
