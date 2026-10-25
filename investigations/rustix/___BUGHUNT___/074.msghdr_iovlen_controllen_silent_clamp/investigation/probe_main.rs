use rustix::net::{socketpair, AddressFamily, SocketType, sendmsg, SendAncillaryBuffer, SendAncillaryMessage, SendFlags};
use rustix::io::IoSlice;
use rustix::fd::BorrowedFd;
use std::mem::MaybeUninit;

fn try_iovcnt(n: usize) {
    let (a, _b) = socketpair(AddressFamily::UNIX, SocketType::DGRAM, rustix::net::SocketFlags::empty(), None)
        .expect("socketpair");
    let byte = [0u8; 1];
    let iov: Vec<IoSlice> = (0..n).map(|_| IoSlice::new(&byte)).collect();
    let mut space = [MaybeUninit::uninit(); 0];
    let mut control = SendAncillaryBuffer::new(&mut space);
    let result = sendmsg(&a, &iov, &mut control, SendFlags::empty());
    println!("[iovcnt test] n={n} (c_int clamp threshold is 2147483647) -> sendmsg result = {:?}", result);
}

fn try_control(n: usize) {
    let (a, b) = socketpair(AddressFamily::UNIX, SocketType::DGRAM, rustix::net::SocketFlags::empty(), None)
        .expect("socketpair");
    let stdin_fd = unsafe { BorrowedFd::borrow_raw(0) };
    let fds: Vec<BorrowedFd> = (0..n).map(|_| stdin_fd).collect();
    let msg = SendAncillaryMessage::ScmRights(&fds);
    let space_size = msg.size();
    let mut space: Vec<MaybeUninit<u8>> = vec![MaybeUninit::uninit(); space_size];
    let mut control = SendAncillaryBuffer::new(&mut space);
    control.push(msg);
    let byte = [0u8; 1];
    let iov = [IoSlice::new(&byte)];
    let result = sendmsg(&a, &iov, &mut control, SendFlags::empty());
    println!("[controllen test] fds={n} control_bytes={space_size} (socklen_t clamp threshold is 4294967295) -> sendmsg result = {:?}", result);
    drop(b);
}

fn main() {
    println!("=== real optmem_max on this system ===");
    println!("{}", std::fs::read_to_string("/proc/sys/net/core/optmem_max").unwrap());

    println!("=== iovcnt boundary (real kernel UIO_MAXIOV is 1024) ===");
    try_iovcnt(1024);
    try_iovcnt(1025);

    println!("=== controllen boundary (real kernel caps via optmem_max) ===");
    try_control(1000);
    try_control(32_500);
    try_control(33_000);
    try_control(100_000);
}
