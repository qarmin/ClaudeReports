use rustix::io::IoSliceMut;
use rustix::net::{recvmsg, RecvAncillaryBuffer, RecvFlags};
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

const MSG_LEN: usize = 16;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("local_addr");

    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept");

        stream.write_all(b"FIRSTMESSAGE1234").expect("write first");
        stream.flush().unwrap();

        thread::sleep(Duration::from_millis(200));

        stream.write_all(b"SECONDMSGABCDEFG").expect("write second");
        stream.flush().unwrap();

        thread::sleep(Duration::from_millis(300));
    });

    thread::sleep(Duration::from_millis(50));
    let client = TcpStream::connect(addr).expect("connect");

    let mut buf = [0u8; MSG_LEN];

    println!("=== Call 1: normal recvmsg (no TRUNC) ===");
    println!("buffer before call 1: {:?}", String::from_utf8_lossy(&buf));
    {
        let mut iov = [IoSliceMut::new(&mut buf)];
        let mut cmsg_buffer = RecvAncillaryBuffer::default();
        let result = recvmsg(&client, &mut iov, &mut cmsg_buffer, RecvFlags::empty())
            .expect("recvmsg 1 failed");
        println!("recvmsg 1: bytes={} flags={:?}", result.bytes, result.flags);
    }
    println!("buffer after call 1: {:?}", String::from_utf8_lossy(&buf));
    assert_eq!(&buf, b"FIRSTMESSAGE1234");

    thread::sleep(Duration::from_millis(300));

    println!();
    println!("=== Call 2: recvmsg with RecvFlags::TRUNC (same buffer, reused) ===");
    println!(
        "buffer before call 2 (should still say FIRSTMESSAGE1234): {:?}",
        String::from_utf8_lossy(&buf)
    );
    {
        let mut iov = [IoSliceMut::new(&mut buf)];
        let mut cmsg_buffer = RecvAncillaryBuffer::default();
        let result = recvmsg(&client, &mut iov, &mut cmsg_buffer, RecvFlags::TRUNC)
            .expect("recvmsg 2 failed");
        println!("recvmsg 2: bytes={} flags={:?}", result.bytes, result.flags);
    }
    println!(
        "buffer after call 2 (STALE DATA CHECK): {:?}",
        String::from_utf8_lossy(&buf)
    );

    if &buf == b"FIRSTMESSAGE1234" {
        println!();
        println!(
            "CONFIRMED: buffer still contains the FIRST message's bytes after a second \
             recvmsg(TRUNC) call that reported new data - stale data leak reproduced."
        );
    } else if buf.iter().all(|&b| b == 0) {
        println!();
        println!("Buffer was zeroed - kernel/backend cleared it (not the suspected bug).");
    } else {
        println!();
        println!("Buffer contains something else entirely: {:?}", buf);
    }

    server.join().unwrap();
}
