use std::io::Write;
use std::mem::MaybeUninit;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use rustix::net::RecvFlags;

const SENT_PATTERN: [u8; 16] = *b"REALDATA12345678";
const POISON_BYTE: u8 = 0xAA;

fn run_case(buf_len: usize, use_trunc: bool, label: &str) {
    let (tx, rx) = mpsc::channel();

    let server = thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        tx.send(port).unwrap();
        let (mut conn, _) = listener.accept().unwrap();
        conn.write_all(&SENT_PATTERN).unwrap();
        thread::sleep(Duration::from_millis(200));
    });

    let port = rx.recv().unwrap();
    thread::sleep(Duration::from_millis(50));
    let stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    thread::sleep(Duration::from_millis(100));

    let mut buf = vec![MaybeUninit::<u8>::new(POISON_BYTE); buf_len];
    let flags = if use_trunc { RecvFlags::TRUNC } else { RecvFlags::empty() };
    let (data, recv_len) = rustix::net::recv(&stream, &mut buf[..], flags).unwrap();
    let init_data: &[u8] = data.0;

    println!("--- case: {label} (buf_len={buf_len}, TRUNC={use_trunc}) ---");
    println!("  recv_len = {recv_len}");
    println!("  init_data.len() = {}", init_data.len());
    println!("  init_data = {init_data:02x?}");

    let sent_prefix = &SENT_PATTERN[..init_data.len().min(SENT_PATTERN.len())];
    if init_data.is_empty() {
        println!("  => 0 bytes claimed; safe regardless of TRUNC semantics.");
    } else if init_data == sent_prefix {
        println!("  => init_data matches real sent bytes (legitimate).");
    } else if init_data.iter().all(|&b| b == POISON_BYTE) {
        println!("  => BUG: init_data is pure poison, falsely certified as real received data!");
    } else {
        println!("  => BUG (mixed): init_data contains non-sent, non-obviously-poison bytes.");
    }

    server.join().unwrap();
}

fn main() {
    run_case(SENT_PATTERN.len(), true, "equal-size + TRUNC");
    run_case(SENT_PATTERN.len() * 4, true, "larger-buffer + TRUNC");
    run_case(SENT_PATTERN.len() / 2, true, "smaller-buffer + TRUNC");
    run_case(SENT_PATTERN.len(), false, "equal-size, no TRUNC (control)");
}
