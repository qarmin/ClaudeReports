// Real repro: a real TCP server thread and a real TCP client over a real
// loopback socket. The client calls `rustix::net::recv`'s real public entry
// point with `RecvFlags::TRUNC`, reusing one buffer across two real `recv`
// calls - demonstrating exactly the hazard the doc addition in this
// candidate's patch.diff describes for `recv`/`recvfrom`.
use rustix::net::{RecvFlags, recv};
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

const FIRST: &[u8] = b"FIRST-MESSAGE-16";
const SECOND: &[u8] = b"SECOND-MSG-16-XX";

fn main() {
    assert_eq!(FIRST.len(), SECOND.len());
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream.write_all(FIRST).unwrap();
        stream.write_all(SECOND).unwrap();
    });

    let stream = TcpStream::connect(addr).unwrap();
    let fd = &stream;

    // First real recv() with TRUNC: buffer starts zeroed, gets FIRST's bytes
    // written normally by this call (TRUNC only discards when the *kernel*
    // decides to truncate, which a same-size buffer + no MSG_TRUNC-shaped
    // input does not trigger on the first read).
    let mut buf = vec![0u8; FIRST.len()];
    let (n1, _n) = recv(fd, &mut buf[..], RecvFlags::empty()).unwrap();
    println!("call 1 (no TRUNC): buf = {:?} (Output usize = {})", buf, n1);
    assert_eq!(&buf[..], FIRST);

    // Second real recv() with TRUNC on the SAME reused buffer, now holding
    // SECOND's bytes in the kernel's receive queue.
    let (_out_len, n2) = recv(fd, &mut buf[..], RecvFlags::TRUNC).unwrap();
    println!("call 2 (TRUNC):     buf = {:?}, reported byte count = {}", buf, n2);

    if buf[..] == *FIRST {
        println!("BUG: after a TRUNC recv(), the buffer still holds call 1's data (FIRST), not call 2's (SECOND) - yet the call reported {} bytes received.", n2);
    } else if buf[..] == *SECOND {
        println!("no bug observed: buffer holds the real current message");
    } else {
        println!("buffer holds neither message: {:?}", buf);
    }

    server.join().unwrap();
}
