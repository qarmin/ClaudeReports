use dicom_object::meta::{FileMetaTable, FileMetaTableBuilder};

fn write_bytes(table: &FileMetaTable) -> Vec<u8> {
    let mut bytes = b"DICM".to_vec();
    table.write(&mut bytes).expect("write should succeed");
    bytes
}

fn main() {
    // 'Ö' (U+00D6) is 1 byte on the DICOM wire (Latin-1 / ISO_8859_1, what
    // DefaultCharacterSetCodec actually uses) but 2 bytes in a Rust UTF-8 String.
    let table1 = FileMetaTableBuilder::new()
        .media_storage_sop_class_uid("1Ö")
        .media_storage_sop_instance_uid("1.2")
        .transfer_syntax("1.2.840.10008.1.2.1")
        .implementation_class_uid("1.2.3.4")
        .build()
        .expect("build should succeed");

    // state "3": first write
    let wire_gen1 = write_bytes(&table1);
    println!("gen1 (first write) bytes: {} ({:?})", wire_gen1.len(), wire_gen1);

    // state "4": re-parse (1) - real FileMetaTable::read_from path (read_str_body, untrimmed,
    // fed straight back into the same padding setters)
    let table2 = FileMetaTable::from_reader(&wire_gen1[..]).expect("reparse 1 should succeed");

    // state "5": write from (4) - must equal state "3" (3==5 invariant)
    let wire_gen2 = write_bytes(&table2);
    println!("gen2 (second write) bytes: {} ({:?})", wire_gen2.len(), wire_gen2);

    let table3 = FileMetaTable::from_reader(&wire_gen2[..]).expect("reparse 2 should succeed");
    let wire_gen3 = write_bytes(&table3);
    println!("gen3 (third write) bytes: {} ({:?})", wire_gen3.len(), wire_gen3);

    let table4 = FileMetaTable::from_reader(&wire_gen3[..]).expect("reparse 3 should succeed");
    let wire_gen4 = write_bytes(&table4);
    println!("gen4 (fourth write) bytes: {} ({:?})", wire_gen4.len(), wire_gen4);

    let mut failed = false;

    if wire_gen1 != wire_gen2 || wire_gen2 != wire_gen3 || wire_gen3 != wire_gen4 {
        eprintln!(
            "REPRODUCED (growth) - writer is not stable across parse/write generations (violates 3==5): lengths {} -> {} -> {} -> {}",
            wire_gen1.len(),
            wire_gen2.len(),
            wire_gen3.len(),
            wire_gen4.len(),
        );
        failed = true;
    }

    // Second, more severe manifestation of the same root cause: a single write -> reparse can
    // fail outright (wrong group-length header) when a field has no ASCII padding to "absorb"
    // the miscount, e.g. two non-ASCII chars back to back.
    let table5 = FileMetaTableBuilder::new()
        .media_storage_sop_class_uid("ÖÖ")
        .media_storage_sop_instance_uid("1.2")
        .transfer_syntax("1.2.840.10008.1.2.1")
        .implementation_class_uid("1.2.3.4")
        .build()
        .expect("build should succeed");
    let wire5 = write_bytes(&table5);
    match FileMetaTable::from_reader(&wire5[..]) {
        Ok(t) => {
            println!("gen5 (ÖÖ single round trip) reparse OK: {:?}", t.media_storage_sop_class_uid());
        }
        Err(e) => {
            eprintln!("REPRODUCED (group-length mismatch) - single write->reparse round trip failed: {:?}", e);
            failed = true;
        }
    }

    if failed {
        std::process::exit(1);
    }

    println!("did not reproduce - writer output stable, all reparses succeeded");
}
