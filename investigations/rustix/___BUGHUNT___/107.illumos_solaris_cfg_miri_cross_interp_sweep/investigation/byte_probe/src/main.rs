// Root-cause probe: poison a MaybeUninit<libc::stat> (illumos layout) with a
// sentinel byte, call the raw libc::fstat() directly (same call rustix makes
// internally), and report exactly how many bytes the real underlying syscall
// actually overwrote versus how large the illumos `struct stat` type is.
use std::mem::MaybeUninit;

fn main() {
    println!("size_of::<libc::stat>() [illumos layout] = {}", std::mem::size_of::<libc::stat>());
    let base = MaybeUninit::<libc::stat>::uninit();
    let base_ptr = base.as_ptr();
    let fstype_ptr = unsafe { std::ptr::addr_of!((*base_ptr).st_fstype) };
    let offset = fstype_ptr as usize - base_ptr as usize;
    println!("offset_of(st_fstype) = {}", offset);

    let f = std::fs::File::open("Cargo.toml").unwrap();
    use std::os::fd::AsRawFd;
    let fd = f.as_raw_fd();

    let mut st = MaybeUninit::<libc::stat>::uninit();
    unsafe {
        std::ptr::write_bytes(st.as_mut_ptr().cast::<u8>(), 0xAB, std::mem::size_of::<libc::stat>());
        let r = libc::fstat(fd, st.as_mut_ptr());
        println!("libc::fstat() returned {}", r);
        let bytes = std::slice::from_raw_parts(st.as_ptr().cast::<u8>(), std::mem::size_of::<libc::stat>());
        let last_written = bytes.iter().rposition(|&b| b != 0xAB);
        println!("last byte offset actually overwritten by the real fstat() call = {:?}", last_written);
        println!("struct size = {}, st_fstype occupies bytes [{}..{})", bytes.len(), offset, bytes.len());
        println!("=> bytes [{}..{}) were left at the 0xAB sentinel (never written by the real syscall)", last_written.map(|x| x+1).unwrap_or(0), bytes.len());
    }
}
