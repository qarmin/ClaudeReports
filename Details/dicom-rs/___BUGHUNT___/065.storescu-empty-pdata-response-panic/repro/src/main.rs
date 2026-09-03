use dicom_ul::pdu::{read_pdu, Pdu};

fn main() {
    // A wire-level P-DATA-TF PDU (type 0x04) with PDU-length 0 - a completely
    // valid, minimal encoding that a misbehaving or malicious SCP could send
    // in response to a C-STORE-RQ. It legitimately parses to an empty PDV
    // list. Same mechanism as candidate 062's echoscu finding.
    let bytes: [u8; 6] = [0x04, 0x00, 0x00, 0x00, 0x00, 0x00];
    let pdu = read_pdu(&mut &bytes[..], 16384, true)
        .expect("should parse")
        .expect("should be a complete PDU");
    println!("parsed from the wire: {pdu:?}");

    match pdu {
        Pdu::PData { data } => {
            println!("data.len() = {}", data.len());
            // this is the exact pattern storescu/src/store_sync.rs and
            // store_async.rs use on the response PDU to a C-STORE-RQ
            println!("indexing data[0] as storescu does...");
            let data_value = &data[0];
            println!("{:?}", data_value.presentation_context_id);
        }
        _ => unreachable!(),
    }
    println!("done, no panic");
}
