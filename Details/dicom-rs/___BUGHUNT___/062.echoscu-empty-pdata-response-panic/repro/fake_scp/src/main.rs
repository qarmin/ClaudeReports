// A minimal but REAL DICOM SCP: accepts a genuine TCP connection, performs genuine DICOM
// association negotiation via `dicom-ul`'s own real, public `ServerAssociationOptions::establish`
// (not hand-crafted bytes), receives the real C-ECHO-RQ command PDU the peer actually sends, and
// then responds with a `Pdu::PData { data: vec![] }` - a *valid*, legitimately-encoded P-Data-TF
// PDU with zero PDVs (`dicom-ul`'s own real wire writer produces this from an empty Vec, exactly
// as a misbehaving/malicious real SCP could choose to do), sent over the same real socket via the
// association's own real `send()` method.
use dicom_ul::{Pdu, ServerAssociationOptions};
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

    let mut association = options.establish(socket).expect("real association negotiation failed");
    println!("fake SCP: real association established");

    let pdu = association.receive().expect("failed to receive the real C-ECHO-RQ");
    println!("fake SCP: received a real PDU from the peer: {pdu:?}");

    // Respond with a wire-legal but empty PData PDU instead of a normal C-ECHO-RSP - this is
    // dicom-ul's own real `write_pdu` encoding of an empty Vec<PDataValue>, sent over the real
    // socket via the association's own real `send()` method.
    println!("fake SCP: sending a real, valid, but EMPTY P-Data-TF PDU as the response...");
    association
        .send(&Pdu::PData { data: vec![] })
        .expect("failed to send the empty PData PDU");
    println!("fake SCP: done");
}
