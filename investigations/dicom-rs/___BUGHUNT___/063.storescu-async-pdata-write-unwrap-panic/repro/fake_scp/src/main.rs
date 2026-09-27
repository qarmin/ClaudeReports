// A minimal but REAL DICOM SCP: accepts a genuine TCP connection, performs genuine DICOM
// association negotiation via `dicom-ul`'s own real, public `ServerAssociationOptions::establish`
// (not hand-crafted bytes), receives the real C-STORE-RQ command PDU the peer actually sends,
// then abruptly drops the connection - a real TCP disconnect (RST) - instead of reading the
// large P-Data payload the peer is about to send. This is exactly what a real network failure
// (a crashed/misbehaving peer, a dropped connection) looks like from the sender's side.
use dicom_ul::ServerAssociationOptions;
use std::net::TcpListener;

fn main() {
    let port: u16 = std::env::args().nth(1).unwrap().parse().unwrap();
    let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();
    println!("fake SCP listening on 127.0.0.1:{port}");

    let (socket, _addr) = listener.accept().unwrap();
    println!("fake SCP accepted a real TCP connection");

    let options = ServerAssociationOptions::new()
        .accept_any()
        .with_transfer_syntax("1.2.840.10008.1.2")
        .with_transfer_syntax("1.2.840.10008.1.2.1")
        .promiscuous(true);

    let mut association = options
        .establish(socket)
        .expect("real association negotiation failed");
    println!("fake SCP: real association established");

    let _pdu = association
        .receive()
        .expect("failed to receive the real command PDU");
    println!("fake SCP: received the real C-STORE-RQ command PDU from the peer");

    println!("fake SCP: abruptly closing the connection before reading P-Data...");
    drop(association);
    println!("fake SCP: done");
}
