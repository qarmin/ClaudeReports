//! Counts allocations so the benchmark can report them per frame - allocation
//! pressure is the reported problem with large non-virtualized views.
//!
//! Disable with `--no-default-features` (feature `alloc-stats`) when a profile must
//! not carry the two relaxed atomics per allocation.

use std::sync::atomic::{AtomicU64, Ordering};

pub static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);

pub fn allocations() -> u64 {
    ALLOCATIONS.load(Ordering::Relaxed)
}

#[cfg(feature = "alloc-stats")]
mod counting_allocator {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::Ordering;

    pub struct CountingAllocator;

    unsafe impl GlobalAlloc for CountingAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            super::ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
            #[cfg(feature = "alloc-profile")]
            crate::alloc_profile::sample();
            unsafe { System.alloc(layout) }
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            unsafe { System.dealloc(ptr, layout) }
        }

        unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
            super::ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
            unsafe { System.realloc(ptr, layout, new_size) }
        }

        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            super::ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
            unsafe { System.alloc_zeroed(layout) }
        }
    }
}

#[cfg(all(feature = "alloc-stats", not(feature = "mimalloc")))]
#[global_allocator]
static ALLOCATOR: counting_allocator::CountingAllocator = counting_allocator::CountingAllocator;

/// Candidate 07: measures how much of a frame is allocator cost. Allocation counting is off in
/// this configuration, so only the timings mean anything.
#[cfg(feature = "mimalloc")]
#[global_allocator]
static MIMALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

/// Resident set size in bytes, or `None` where /proc is not available.
pub fn resident_bytes() -> Option<u64> {
    #[cfg(target_os = "linux")]
    {
        let statm = std::fs::read_to_string("/proc/self/statm").ok()?;
        let pages: u64 = statm.split_whitespace().nth(1)?.parse().ok()?;
        Some(pages * 4096)
    }
    #[cfg(not(target_os = "linux"))]
    None
}

/// Voluntary/involuntary context switches and minor/major page faults for the whole process so
/// far, from `/proc/self/status` and `/proc/self/stat`. A coarse "was this process rescheduled or
/// migrated across cores" signal, added to help attribute a core-count-dependent result `perf`
/// cannot explain in this environment (`perf_event_open` is blocked; see
/// `___OPTIMIZATIONS___/087.FLAT_SCROLL_CHUNK_SIZE_CORE_COUNT_PROFILE`). Read once at the end of a
/// run, not per frame, so it does not perturb the timing it is meant to help explain.
#[derive(Debug, Clone, Copy)]
pub struct SchedulingStats {
    pub voluntary_ctxt_switches: u64,
    pub nonvoluntary_ctxt_switches: u64,
    pub minor_faults: u64,
    pub major_faults: u64,
}

pub fn scheduling_stats() -> Option<SchedulingStats> {
    #[cfg(target_os = "linux")]
    {
        let status = std::fs::read_to_string("/proc/self/status").ok()?;
        let mut voluntary_ctxt_switches = None;
        let mut nonvoluntary_ctxt_switches = None;
        for line in status.lines() {
            if let Some(rest) = line.strip_prefix("voluntary_ctxt_switches:") {
                voluntary_ctxt_switches = rest.trim().parse().ok();
            } else if let Some(rest) = line.strip_prefix("nonvoluntary_ctxt_switches:") {
                nonvoluntary_ctxt_switches = rest.trim().parse().ok();
            }
        }

        // /proc/self/stat's 2nd field (comm) is parenthesized and may itself contain spaces or
        // parens, so split after the *last* ')' rather than relying on a fixed field count.
        let stat = std::fs::read_to_string("/proc/self/stat").ok()?;
        let after_comm = stat.rsplit_once(')')?.1;
        let fields: Vec<&str> = after_comm.split_whitespace().collect();
        // Fields after `comm` start at `state` (man proc's field 3): state, ppid, pgrp, session,
        // tty_nr, tpgid, flags, minflt, cminflt, majflt, ...
        let minor_faults = fields.get(7)?.parse().ok()?;
        let major_faults = fields.get(9)?.parse().ok()?;

        Some(SchedulingStats {
            voluntary_ctxt_switches: voluntary_ctxt_switches?,
            nonvoluntary_ctxt_switches: nonvoluntary_ctxt_switches?,
            minor_faults,
            major_faults,
        })
    }
    #[cfg(not(target_os = "linux"))]
    None
}
