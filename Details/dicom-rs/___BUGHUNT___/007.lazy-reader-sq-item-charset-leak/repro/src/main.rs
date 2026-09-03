use dicom_dictionary_std::tags;
use dicom_object::meta::FileMetaTableBuilder;
use dicom_object::{DicomCollector, InMemDicomObject};
use std::io::Write;

fn el_short(group: u16, elem: u16, vr: &[u8; 2], value: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&group.to_le_bytes());
    out.extend_from_slice(&elem.to_le_bytes());
    out.extend_from_slice(vr);
    out.extend_from_slice(&(value.len() as u16).to_le_bytes());
    out.extend_from_slice(value);
    out
}

fn build_dataset_bytes() -> Vec<u8> {
    let mut wire = Vec::new();
    wire.extend_from_slice(&0x0008u16.to_le_bytes());
    wire.extend_from_slice(&0x1110u16.to_le_bytes());
    wire.extend_from_slice(b"SQ");
    wire.extend_from_slice(&[0, 0]);
    wire.extend_from_slice(&0xFFFFFFFFu32.to_le_bytes());

    let mut item = Vec::new();
    item.extend(el_short(0x0008, 0x0005, b"CS", b"ISO_IR 126"));
    item.extend(el_short(0x0010, 0x0010, b"PN", &[0xE1, 0x20]));

    wire.extend_from_slice(&0xFFFEu16.to_le_bytes());
    wire.extend_from_slice(&0xE000u16.to_le_bytes());
    wire.extend_from_slice(&(item.len() as u32).to_le_bytes());
    wire.extend_from_slice(&item);

    wire.extend_from_slice(&0xFFFEu16.to_le_bytes());
    wire.extend_from_slice(&0xE0DDu16.to_le_bytes());
    wire.extend_from_slice(&0u32.to_le_bytes());

    wire.extend(el_short(0x0010, 0x0010, b"PN", &[0xE1, 0x20]));
    wire
}

fn main() {
    let meta = FileMetaTableBuilder::new()
        .media_storage_sop_class_uid("1.2.840.10008.5.1.4.1.1.7")
        .media_storage_sop_instance_uid("1.2.3.4")
        .transfer_syntax("1.2.840.10008.1.2.1")
        .implementation_class_uid("1.2.3.4")
        .build()
        .expect("meta should build");

    let mut file_bytes = vec![0u8; 128];
    file_bytes.extend_from_slice(b"DICM");
    meta.write(&mut file_bytes).expect("meta should write");
    file_bytes.extend(build_dataset_bytes());

    let dir = std::env::temp_dir();
    let path = dir.join("bughunt_007_lazy_charset_leak.dcm");
    std::fs::File::create(&path)
        .and_then(|mut f| f.write_all(&file_bytes))
        .expect("should write temp file");

    let mut collector = DicomCollector::open_file(&path).expect("should open file");
    let _fmi = collector.read_file_meta();

    let mut obj = InMemDicomObject::new_empty();
    collector
        .read_dataset_to_end(&mut obj)
        .expect("should read the rest of the DICOM");

    std::fs::remove_file(&path).ok();

    let inner = obj
        .get(tags::REFERENCED_STUDY_SEQUENCE)
        .expect("sequence should be present")
        .items()
        .expect("should have items")[0]
        .element(tags::PATIENT_NAME)
        .expect("item should have PatientName")
        .to_str()
        .expect("should decode as string")
        .to_string();

    let top = obj
        .element(tags::PATIENT_NAME)
        .expect("top-level PatientName should be present")
        .to_str()
        .expect("should decode as string")
        .to_string();

    println!("item-scoped PatientName (Greek, ISO_IR 126 declared inside the item): {:?}", inner);
    println!("top-level PatientName (no charset declared at top level, should be default/Latin-1): {:?}", top);

    if top == inner {
        eprintln!(
            "REPRODUCED - LazyDataSetReader: the item's own SpecificCharacterSet leaked past the \
             item boundary: top-level PatientName decoded identically to the item-scoped one \
             ({:?}) instead of under the default character repertoire",
            top
        );
        std::process::exit(1);
    }

    println!("did not reproduce - top-level element correctly used the outer/default charset scope");
}
