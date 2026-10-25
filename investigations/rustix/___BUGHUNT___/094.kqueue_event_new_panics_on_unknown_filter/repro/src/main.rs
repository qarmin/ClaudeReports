//! Standalone reproduction for candidate 094.
//!
//! `rustix::event::kqueue::Event::filter()` decodes a real `kevent()`-returned event whose
//! `filter` field is one of the many real kqueue filter types rustix doesn't have typed support
//! for (e.g. Apple's `EVFILT_MACHPORT`/`EVFILT_VM`/`EVFILT_EXCEPT`/`EVFILT_AIO`) into the public,
//! `#[non_exhaustive]` `EventFilter::Unknown` variant. Feeding that decoded `Unknown` back into
//! `Event::new()` - exactly what a generic "decode an event, then re-register/re-submit it"
//! pipeline would do - panics instead of returning an error (pre-patch).
//!
//! This only runs on a real BSD/Apple target (`kqueue`/`kevent` don't exist on Linux at all), and
//! is interpreted under Miri cross-interpreting `x86_64-apple-darwin` from a Linux host, since no
//! real Apple/BSD hardware is available in the environment this was investigated in - see
//! RESULT.md for why that's the right substitute here: this exercises the real, unmodified,
//! shipped `Event::filter()`/`Event::new()` logic (no mock of rustix's own code), the only thing
//! simulated is the "kernel wrote this into the eventlist buffer" step, which a real `kevent()`
//! syscall cannot be executed to produce in this sandbox but also isn't `rustix`'s own code to
//! begin with - it's standing in for a single external producer (the kernel), not a second
//! rustix-side participant, the same way a parser repro would feed in a byte buffer standing in
//! for a real file's bytes.
//!
//! Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present).

use rustix::event::kqueue::{Event, EventFilter, EventFlags};
#[cfg(not(feature = "fixed"))]
use std::panic;

// Real Apple kqueue filter constants (`libc` crate's `src/unix/bsd/apple/mod.rs`) that
// `Event::filter()` does not handle in its `match` - real values a genuine `kevent()` syscall can
// and does produce, which fall through to `_ => EventFilter::Unknown`.
const EVFILT_AIO: std::os::raw::c_short = -3;
const EVFILT_MACHPORT: std::os::raw::c_short = -8;
const EVFILT_VM: std::os::raw::c_short = -12;
const EVFILT_EXCEPT: std::os::raw::c_short = -15;

/// Stands in for what a real `kevent()` syscall writes into the caller's eventlist buffer.
///
/// SAFETY: `Event` is `#[repr(transparent)]` over `libc::kevent` (`src/event/kqueue.rs`), so this
/// transmute reproduces exactly the bit pattern a real syscall would have written; the only thing
/// being substituted is the kernel itself (unavailable in this sandbox), not any rustix code.
fn make_raw_event(filter: std::os::raw::c_short) -> Event {
    let raw = libc::kevent {
        ident: 42,
        filter,
        flags: 0,
        fflags: 0,
        data: 0,
        udata: std::ptr::null_mut(),
    };
    unsafe { std::mem::transmute(raw) }
}

fn main() {
    #[cfg(not(feature = "fixed"))]
    let mut reproduced = false;

    for (name, filter) in [
        ("EVFILT_AIO", EVFILT_AIO),
        ("EVFILT_MACHPORT", EVFILT_MACHPORT),
        ("EVFILT_VM", EVFILT_VM),
        ("EVFILT_EXCEPT", EVFILT_EXCEPT),
    ] {
        let event = make_raw_event(filter);
        let decoded = event.filter();
        let is_unknown = matches!(decoded, EventFilter::Unknown);
        println!("{name} ({filter}) decodes via Event::filter() to Unknown: {is_unknown}");
        if !is_unknown {
            eprintln!("UNEXPECTED: {name} did not decode to EventFilter::Unknown");
            std::process::exit(2);
        }

        #[cfg(not(feature = "fixed"))]
        {
            let result = panic::catch_unwind(|| {
                Event::new(EventFilter::Unknown, EventFlags::ADD, std::ptr::null_mut())
            });
            match result {
                Err(payload) => {
                    let msg = payload
                        .downcast_ref::<&str>()
                        .copied()
                        .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
                        .unwrap_or("<non-string panic payload>");
                    println!("REPRODUCED for {name}: Event::new(EventFilter::Unknown, ..) panicked: {msg:?}");
                    reproduced = true;
                }
                Ok(_) => {
                    println!("did not reproduce for {name}: Event::new(EventFilter::Unknown, ..) did not panic");
                }
            }
        }

        #[cfg(feature = "fixed")]
        {
            match Event::new(EventFilter::Unknown, EventFlags::ADD, std::ptr::null_mut()) {
                Err(e) if e == rustix::io::Errno::INVAL => {
                    println!("FIXED for {name}: Event::new(EventFilter::Unknown, ..) returned Err(INVAL), no panic");
                }
                Err(e) => {
                    eprintln!("UNEXPECTED Err for {name}: {e:?}");
                    std::process::exit(2);
                }
                Ok(_) => {
                    eprintln!("UNEXPECTED Ok for {name}");
                    std::process::exit(2);
                }
            }
        }
    }

    #[cfg(not(feature = "fixed"))]
    {
        if reproduced {
            eprintln!("REPRODUCED - see panics above");
            std::process::exit(1);
        } else {
            println!("did not reproduce - suspect code path completed normally for every input");
        }
    }

    #[cfg(feature = "fixed")]
    println!("did not reproduce - fix verified, no panic for any tested input");
}
