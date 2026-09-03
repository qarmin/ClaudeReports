// PARTIAL RECOVERY ONLY - 2026-09-01. The original `_scratch132/` sweep tool and its
// `core.115360` crash dump were destroyed when the worktree holding them
// (`.claude/worktrees/agent-a3ba99d09a37f6449`) was removed via `git worktree remove --force`
// during post-usage-limit-reset recovery cleanup, BEFORE this content was salvaged - a real
// process mistake (should have copied the file and core dump out first, per SKILL.md's "never
// leave the *only* copy of a promising fix/progress sitting unreverted - capture it before
// discarding"). This is everything that survived in the recovering session's own tool-output
// history (the first ~100 lines of the source file; the rest, plus the `Cargo.toml`/`Cargo.lock`
// and the actual crash core dump, are gone and cannot be recovered).
//
// Known facts about the run that crashed (from `gdb` on the core before it was deleted):
// - invocation: `target/release/scratch132 20000` (release build)
// - signal: SIGABRT (a real core dump was generated, i.e. not a clean panic=unwind path)
// - gdb could NOT produce a symbolized backtrace: "exec file is newer than core file" / build-id
//   mismatch (the binary on disk had been rebuilt after the core was written) - so the actual
//   crash site is genuinely unknown, only that it happened somewhere during the n=20000 sweep.
//
// A future session should treat this as "reconstruct and re-run", not "resume" - rebuild a
// sweep tool with this same shape (see IDEAS.md's resume note for the full description: private
// UN-VR undefined-length elements under a Philips private creator block, 4 item-shapes, swept
// element count n, both Explicit and Implicit VR LE), run across n in {100, 1000, 5000, 20000},
// and this time keep the binary untouched and copy the core dump out immediately on crash,
// symbolizing with `gdb -batch -ex "bt full" -ex "thread apply all bt" target/release/scratch132
// core.*` before doing anything else.

//! Sweep: does adding MORE consecutive private UN undefined-length elements ever change the
//! parse outcome versus a single one?
use dicom_object::InMemDicomObject;
use dicom_transfer_syntax_registry::entries::{
    EXPLICIT_VR_LITTLE_ENDIAN, IMPLICIT_VR_LITTLE_ENDIAN,
};
use std::collections::HashMap;

const UNDEF: [u8; 4] = [0xff, 0xff, 0xff, 0xff];

fn tag(g: u16, e: u16) -> Vec<u8> {
    let mut v = g.to_le_bytes().to_vec();
    v.extend_from_slice(&e.to_le_bytes());
    v
}
fn u32le(x: u32) -> Vec<u8> {
    x.to_le_bytes().to_vec()
}

/// shape 0: item with defined length
/// shape 1: item with undefined length + item delimiter
/// shape 2: no item at all
/// shape 3: two defined-length items
fn priv_un(explicit: bool, g: u16, e: u16, payload: &[u8], shape: u8) -> Vec<u8> {
    let mut v = tag(g, e);
    if explicit {
        v.extend_from_slice(b"UN");
        v.extend_from_slice(&[0, 0]);
    }
    v.extend_from_slice(&UNDEF);
    match shape {
        0 => {
            v.extend(tag(0xfffe, 0xe000));
            v.extend(u32le(payload.len() as u32));
            v.extend_from_slice(payload);
        }
        1 => {
            v.extend(tag(0xfffe, 0xe000));
            v.extend_from_slice(&UNDEF);
            v.extend_from_slice(payload);
            v.extend(tag(0xfffe, 0xe00d));
            v.extend(u32le(0));
        }
        2 => v.extend_from_slice(payload),
        _ => {
            let half = payload.len() / 2;
            for chunk in [&payload[..half], &payload[half..]] {
                v.extend(tag(0xfffe, 0xe000));
                v.extend(u32le(chunk.len() as u32));
                v.extend_from_slice(chunk);
            }
        }
    }
    v.extend(tag(0xfffe, 0xe0dd));
    v.extend(u32le(0));
    v
}

fn repeat_elems(explicit: bool, n: usize, payload: &[u8], shape: u8, creator: bool) -> Vec<u8> {
    let mut v = Vec::new();
    if creator {
        // realistic Philips private creator element preceding the private block
        v.extend(tag(0x2005, 0x0010));
        if explicit {
            v.extend_from_slice(b"LO");
            v.extend_from_slice(&22u16.to_le_bytes());
        } else {
            v.extend(u32le(22));
        }
        v.extend_from_slice(b"Philips Imaging DD 001");
    }
    for i in 0..n {
        v.extend(priv_un(explicit, 0x2005, 0x1080 + i as u16, payload, shape));
    }
    v
}

fn normalize(s: &str) -> String {
    let mut out = String::new();
    let mut prev_digit = false;
    for c in s.chars() {
        if c.is_ascii_digit() || c.is_ascii_hexdigit() && c.is_ascii_uppercase() {
            if !prev_digit {
                out.push('#');
                prev_digit = true;
            }
        } else {
            prev_digit = false;
            out.push(c);
        }
    }
    out
}

// --- REST OF FILE (outcome() body, main()) NOT RECOVERED - lost with the deleted worktree. ---
