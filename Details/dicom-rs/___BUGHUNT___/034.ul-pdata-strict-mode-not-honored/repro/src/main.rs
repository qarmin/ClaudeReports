use bytes::BytesMut;
use dicom_ul::association::PDataReader;
use dicom_ul::pdu::{MINIMUM_PDU_SIZE, PDataValue, PDataValueType, Pdu};
use dicom_ul::write_pdu;

fn main() {
    // Simulate a peer sending a single P-Data-TF PDU whose PDV payload is far
    // larger than the max_pdu_length this side would have negotiated for the
    // association.
    let oversized_len = 50_000usize;
    let pdu = Pdu::PData {
        data: vec![PDataValue {
            presentation_context_id: 1,
            value_type: PDataValueType::Data,
            is_last: true,
            data: vec![0xABu8; oversized_len],
        }],
    };

    let mut wire = vec![];
    write_pdu(&mut wire, &pdu).expect("write_pdu should succeed");
    println!("encoded PDU size: {} bytes", wire.len());

    // The smallest legal negotiated max_pdu_length (PS3.8's own protocol
    // minimum) - as would come out of association negotiation with a
    // conservative peer.
    let negotiated_max_pdu_length: u32 = MINIMUM_PDU_SIZE;

    let mut read_buffer = BytesMut::new();
    let mut reader = PDataReader::new(
        std::io::Cursor::new(wire.as_slice()),
        negotiated_max_pdu_length,
        &mut read_buffer,
    );

    let mut out = vec![0u8; oversized_len];
    let n = std::io::Read::read(&mut reader, &mut out).expect("PDataReader::read should not error");

    println!(
        "PDataReader accepted {n} bytes of PDV payload despite a negotiated max_pdu_length of {negotiated_max_pdu_length}"
    );

    if n > negotiated_max_pdu_length as usize {
        println!(
            "BUG: negotiated max_pdu_length was not enforced on incoming P-Data-TF PDU."
        );
        std::process::exit(1);
    } else {
        println!("max_pdu_length was enforced (read was capped/rejected).");
    }
}
