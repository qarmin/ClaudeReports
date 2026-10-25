//! Standalone reproduction for bug-hunt candidate 090.
//!
//! What this demonstrates: rustix's `src/backend/libc/event/syscalls.rs::port_get` does
//!
//!     let mut event = MaybeUninit::<c::port_event>::uninit();
//!     ret(c::port_get(borrowed_fd(port), event.as_mut_ptr(), timeout as _))?;
//!     Ok(Event(unsafe { event.assume_init() }))
//!
//! trusting the real `port_get(3C)`/`port_getn(3C)` syscall to have initialized every byte of
//! the `port_event_t` it wrote into. `c::port_event` is not a rustix-defined type - rustix's
//! `src/backend/libc/c.rs` does `pub(crate) use libc::*;`, so `c::port_event` *is* the `libc`
//! crate's own `port_event` struct, pinned here to the identical version (0.2.189) rustix's own
//! `Cargo.lock` resolves to at this candidate's `base_commit`.
//!
//! Why this repro can't drive rustix's real public `rustix::event::port::get` end to end: Miri
//! (as of the newest nightly available in this environment, `miri 0.1.0 (5c543b0b8c 2026-09-29)`)
//! has no foreign-function shim for `port_create`/`port_get` on OS "illumos" at all - confirmed
//! directly this session (see output.txt / RESULT.md), identical to what an earlier candidate
//! (050, using an older Miri) found. No real Solaris/illumos hardware is available in this
//! sandbox either. Per CONFIRMATION.md section 11's hardware exception, this repro instead
//! reproduces the exact byte-initialization pattern the real illumos kernel is proven (via cited
//! upstream kernel source, see ../evidence/illumos_gate_port.c_excerpt.txt) to produce, applied
//! to the identical `libc::port_event` type rustix uses internally, and shows - under Miri - that
//! this exact pattern is genuine Rust undefined behavior at the precise `assume_init()` operation
//! rustix's own `port_get` performs.
//!
//! Real illumos-gate source (`usr/src/uts/common/fs/portfs/port.c`, see ../evidence/ for the
//! archived excerpt and fetch URL) shows that `port_getn()`'s internal `port_copy_event()` only
//! ever assigns 4 of `port_event_t`'s 5 fields:
//!
//!     puevp->portev_source = pkevp->portkev_source;
//!     puevp->portev_object = pkevp->portkev_object;
//!     puevp->portev_user   = pkevp->portkev_user;
//!     puevp->portev_events = pkevp->portkev_events;
//!     // portev_pad is never assigned anywhere in this file.
//!
//! and that the backing array it writes into is allocated with the kernel's plain (non-zeroing)
//! allocator - `kevp = kmem_alloc(eventsz * nmax, KM_NOSLEEP);` (or the `KM_SLEEP` retry a few
//! lines below) - not `kmem_zalloc()`. That whole buffer, `portev_pad` included, is then
//! `copyout()`'d directly to the calling userland process. `port_get(3C)` is implemented as
//! `port_getn()` with `max = 1`, so this applies to both.

use std::mem::MaybeUninit;

fn main() {
    let mut event = MaybeUninit::<libc::port_event>::uninit();

    // SAFETY (of the demonstration, not of the operation being tested): this mirrors exactly
    // the 4 fields the real illumos kernel's `port_copy_event()` is proven to write (see the
    // module doc above and ../evidence/), on a buffer whose memory the kernel never zeroed
    // first. `portev_pad` is deliberately left untouched - on the real kernel, those 2 bytes
    // would hold whatever kernel heap data was already in that `kmem_alloc()`'d slot.
    unsafe {
        let ptr = event.as_mut_ptr();
        std::ptr::addr_of_mut!((*ptr).portev_events).write(0);
        std::ptr::addr_of_mut!((*ptr).portev_source).write(0);
        std::ptr::addr_of_mut!((*ptr).portev_object).write(0);
        std::ptr::addr_of_mut!((*ptr).portev_user).write(std::ptr::null_mut());
        // portev_pad: intentionally never written - this is the bug.
    }

    // This is the exact operation rustix's own `port_get`/`port_getn` perform on the real
    // syscall's output: `event.assume_init()`, trusting every byte is initialized.
    let event = unsafe { event.assume_init() };

    // Use the result so the whole thing isn't optimized away.
    println!("portev_events = {}", event.portev_events);
}
