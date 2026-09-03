// A minimal but REAL DICOM SCP: accepts a genuine TCP connection, performs genuine DICOM
// association negotiation via `dicom-ul`'s own real, public `ServerAssociationOptions::establish`
// (not hand-crafted bytes), receives the real C-STORE-RQ, then sends back a real,
// legitimately-encoded C-STORE-RSP whose Status field is 0xA700 ("Out of Resources") - an
// ordinary DICOM failure status, exactly matching `storescp`'s own real response-building code
// (reused here) except for the status value.
use dicom_core::{dicom_value, DataElement, VR};
use dicom_dictionary_std::tags;
use dicom_object::InMemDicomObject;
use dicom_ul::{pdu::PDataValueType, Pdu, ServerAssociationOptions};
use std::net::TcpListener;

fn create_cstore_failure_response(
    message_id: u16,
    sop_class_uid: &str,
    sop_instance_uid: &str,
) -> InMemDicomObject {
    InMemDicomObject::command_from_element_iter([
        DataElement::new(tags::AFFECTED_SOP_CLASS_UID, VR::UI, dicom_value!(Str, sop_class_uid)),
        DataElement::new(tags::COMMAND_FIELD, VR::US, dicom_value!(U16, [0x8001])),
        DataElement::new(tags::MESSAGE_ID_BEING_RESPONDED_TO, VR::US, dicom_value!(U16, [message_id])),
        DataElement::new(tags::COMMAND_DATA_SET_TYPE, VR::US, dicom_value!(U16, [0x0101])),
        // 0xA700 = "Out of Resources" - a real DICOM failure status code
        DataElement::new(tags::STATUS, VR::US, dicom_value!(U16, [0xA700])),
        DataElement::new(tags::AFFECTED_SOP_INSTANCE_UID, VR::UI, dicom_value!(Str, sop_instance_uid)),
    ])
}

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

    // receive the real command PDU (and dataset, if bundled together)
    let pdu = association.receive().expect("failed to receive the real command PDU");
    println!("fake SCP: received a real PDU from the peer");
    if let Pdu::PData { data } = &pdu {
        // if the data set was sent separately, drain it too
        if data.iter().all(|v| v.value_type == PDataValueType::Command) {
            let _ = association.receive();
            println!("fake SCP: received the separately-sent data set PDU too");
        }
    }

    let ts = dicom_transfer_syntax_registry::entries::IMPLICIT_VR_LITTLE_ENDIAN.erased();
    let obj = create_cstore_failure_response(1, "1.2.840.10008.5.1.4.1.1.7", "1.2.3.4.5.6.7.8.9");
    let mut obj_data = Vec::new();
    obj.write_dataset_with_ts(&mut obj_data, &ts).unwrap();

    let pc_id = association.presentation_contexts()[0].id;
    let pdu_response = Pdu::PData {
        data: vec![dicom_ul::pdu::PDataValue {
            presentation_context_id: pc_id,
            value_type: PDataValueType::Command,
            is_last: true,
            data: obj_data,
        }],
    };
    association.send(&pdu_response).expect("failed to send the real failure response");
    println!("fake SCP: sent a real C-STORE-RSP with a FAILURE status (0xA700)");

    let pdu = association.receive();
    if let Ok(dicom_ul::Pdu::ReleaseRQ) = pdu {
        let _ = association.send(&dicom_ul::Pdu::ReleaseRP);
    }
    println!("fake SCP: done");
}
