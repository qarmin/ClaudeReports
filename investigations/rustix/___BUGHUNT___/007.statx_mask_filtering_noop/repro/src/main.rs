use rustix::fs::{statx, AtFlags, StatxFlags, CWD};

fn main() {
    let all_bits = StatxFlags::all().bits();
    println!("StatxFlags::all().bits()      = {:#010x}", all_bits);
    println!("u32::MAX                      = {:#010x}", u32::MAX);
    println!("all() == u32::MAX ?             {}", all_bits == u32::MAX);

    // Bit 20 (0x0010_0000): clearly beyond every currently-named StatxFlags
    // variant (highest named bit is STATX_DIOALIGN = 0x2000, bit 13), and
    // distinct from STATX__RESERVED (bit 31, 0x8000_0000) which both
    // backends special-case and reject with EINVAL before the masking
    // line runs.
    const UNKNOWN_BIT: u32 = 1 << 20;

    let requested = StatxFlags::BASIC_STATS | StatxFlags::from_bits_retain(UNKNOWN_BIT);
    println!();
    println!("requested mask                = {:#010x}", requested.bits());

    // This is the exact expression used (before the fix) at both:
    //   src/backend/linux_raw/fs/syscalls.rs:840  `let mask = mask & StatxFlags::all();`
    //   src/backend/libc/fs/syscalls.rs:2091       `let mask = mask & StatxFlags::all();`
    let filtered = requested & StatxFlags::all();
    println!("mask & StatxFlags::all()      = {:#010x}", filtered.bits());
    println!("unknown bit survived filter?    {}", filtered.bits() & UNKNOWN_BIT != 0);
    println!("named BASIC_STATS survived?     {}", filtered.contains(StatxFlags::BASIC_STATS));

    // Real end-to-end call: the public statx() API still succeeds either
    // way (the kernel on this host silently ignores the unknown bit today -
    // the whole point of the guard is protecting against a FUTURE kernel
    // that assigns it a real meaning).
    println!();
    match statx(CWD, "/", AtFlags::empty(), requested) {
        Ok(r) => println!(
            "live statx(\"/\") succeeded, kernel-returned stx_mask = {:#010x}",
            r.stx_mask
        ),
        Err(e) => println!("live statx(\"/\") failed: {:?}", e),
    }
}
