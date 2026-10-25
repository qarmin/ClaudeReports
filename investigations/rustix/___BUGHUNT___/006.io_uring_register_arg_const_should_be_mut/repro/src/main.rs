use rustix::io_uring::{
    io_uring_ptr, io_uring_register, io_uring_rsrc_update, io_uring_setup, IoringRegisterOp,
};
use std::ffi::c_void;

fn main() {
    let mut params = rustix::io_uring::io_uring_params::default();
    let ring_fd = unsafe { io_uring_setup(4, &mut params) }.expect("io_uring_setup failed");

    // `offset == u32::MAX` (i.e. `-1` as `u32`) is the kernel's documented
    // "auto-allocate a slot" request value for IORING_REGISTER_RING_FDS.
    const SENTINEL: u32 = u32::MAX;

    let mut update = io_uring_rsrc_update::default();
    update.offset = SENTINEL;
    update.data = io_uring_ptr::new(std::os::fd::AsRawFd::as_raw_fd(&ring_fd) as usize as *mut c_void);

    println!("offset BEFORE syscall: 0x{:08X} ({})", update.offset, update.offset);

    // Left untyped (not annotated `*const c_void`) so this same source file
    // compiles unchanged against both the pre-fix API (`arg: *const c_void`,
    // via an implicit `*mut T -> *const T` coercion at the call site) and
    // the fixed API (`arg: *mut c_void`, an exact match) - see patch.diff.
    let arg_ptr = (&mut update as *mut io_uring_rsrc_update).cast::<c_void>();

    let ret = unsafe {
        io_uring_register(&ring_fd, IoringRegisterOp::RegisterRingFds, arg_ptr, 1)
    };

    match ret {
        Ok(n) => println!("io_uring_register(RegisterRingFds) returned Ok({n})"),
        Err(e) => {
            println!("io_uring_register(RegisterRingFds) failed: {e}");
            return;
        }
    }

    println!("offset AFTER  syscall: 0x{:08X} ({})", update.offset, update.offset);

    if update.offset == SENTINEL {
        println!("RESULT: offset was NOT mutated by the kernel (unexpected).");
    } else {
        println!(
            "RESULT: offset WAS mutated by the kernel through a pointer rustix's \
             public API declares as `*const c_void` (immutable). This is the \
             registered ring-fd slot index the kernel allocated: {}",
            update.offset
        );
    }
}
