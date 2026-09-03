use dicom_ul::pdu::{
    AssociationRQ, Pdu, UserIdentity, UserIdentityType, UserVariableItem, read_pdu, write_pdu,
};
use dicom_ul::pdu::MAXIMUM_PDU_SIZE;

fn main() {
    // A field the public API happily accepts (UserIdentity's primary_field is an
    // unbounded Vec<u8>), sized just past what the wire format's u16 Item-length
    // field for this sub-item can represent (65535).
    let big_len = 70_000usize;
    let primary_field = vec![0x41u8; big_len];

    let user_identity =
        UserIdentity::new(false, UserIdentityType::SamlAssertion, primary_field, vec![]);

    let pdu = Pdu::AssociationRQ(AssociationRQ {
        protocol_version: 1,
        calling_ae_title: "SCU".to_string(),
        called_ae_title: "SCP".to_string(),
        application_context_name: "1.2.840.10008.3.1.1.1".to_string(),
        presentation_contexts: vec![],
        user_variables: vec![UserVariableItem::UserIdentityItem(user_identity)],
    });

    let mut out = vec![];
    let write_result = write_pdu(&mut out, &pdu);

    println!("primary_field original length: {big_len}");
    println!("write_pdu result: {write_result:?}");

    match &write_result {
        Ok(()) => {
            println!("encoded PDU total length: {} bytes", out.len());
            // Try to read the PDU back with this crate's own reader - a truncated
            // length prefix desyncs parsing of everything after this field.
            let mut cursor = std::io::Cursor::new(out.as_slice());
            let read_result = read_pdu(&mut cursor, MAXIMUM_PDU_SIZE, false);
            println!("read_pdu result (should fail to make sense of it): {read_result:?}");
            std::process::exit(1);
        }
        Err(_) => {
            println!("write_pdu correctly rejected the oversized field.");
        }
    }
}
