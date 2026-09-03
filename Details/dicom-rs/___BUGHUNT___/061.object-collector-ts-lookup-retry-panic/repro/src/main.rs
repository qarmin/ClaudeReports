use dicom_object::{DicomCollector, InMemDicomObject};
use std::io::{BufReader, Cursor};

fn main() {
    // arbitrary raw bytes - contents don't matter, since the panic happens
    // before any actual parsing of the data set body
    let encoded: Vec<u8> = vec![0u8; 16];
    let reader = BufReader::new(Cursor::new(encoded));

    // an unrecognized transfer syntax UID, set explicitly (bypassing file-meta
    // guessing) so the first read attempt fails with UnrecognizedTransferSyntax
    let mut collector = DicomCollector::new_with_ts(reader, "1.2.3.4.not.a.real.ts");

    let mut dset = InMemDicomObject::new_empty();

    println!("first read_dataset_to_end call (expected to fail cleanly)...");
    let first = collector.read_dataset_to_end(&mut dset);
    println!("first: {:?}", first.is_ok());

    println!("second read_dataset_to_end call (retry after failure)...");
    let second = collector.read_dataset_to_end(&mut dset);
    println!("second: {:?}", second.is_ok());

    println!("done, no panic");
}
