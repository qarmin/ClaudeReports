//! Reproduces: `InMemDicomObject::apply_leaf`'s `AttributeAction::SetVr` arm
//! (`object/src/mem.rs`) reconstructs the element via
//! `DataElement::new(header.tag, new_vr, value)`, which sets `header.len = value.length()`.
//! For a `Value::Sequence(seq)`, `value.length()` reads `DataSetSequence`'s own private
//! `length` field (`core/src/value/mod.rs`) - a field that is only ever set at parse time
//! and is *not* kept in sync by `items_mut()`/`update_value`-based item mutations (those
//! only reset the owning `DataElement`'s `header.len`, not the sequence's own vestigial
//! `length` field, per candidate 109).
//!
//! So: if a caller first mutates a sequence's items via `update_value_at`/`items_mut()`
//! (correctly resetting `header.len` to `Length::UNDEFINED`), then calls
//! `apply(AttributeOp::new(seq_tag, AttributeAction::SetVr(_)))` on that same tag, the
//! stale pre-mutation length resurfaces into the reconstructed `header.len` - silently
//! undoing the earlier, correct invalidation. When written with
//! `ExplicitLengthSqItemStrategy::NoChange`, that stale length is serialized verbatim,
//! producing a data set whose declared sequence length no longer matches its actual
//! encoded content, corrupting the output detectably by round-tripping it back through
//! the crate's own real reader.
//!
//! Two boundary variants are exercised: growing the sequence (push, one more item than
//! the stale length covers) and shrinking it (remove, fewer bytes than the stale length
//! covers) - both are the same root cause, exercised at opposite ends of the same bug
//! class per CONFIRMATION.md section 7.

use dicom_core::header::Tag;
use dicom_core::ops::{ApplyOp, AttributeAction, AttributeOp};
use dicom_core::value::{PrimitiveValue, Value};
use dicom_core::VR;
use dicom_object::InMemDicomObject;
use dicom_parser::dataset::write::{DataSetWriterOptions, ExplicitLengthSqItemStrategy};
use dicom_transfer_syntax_registry::{TransferSyntaxIndex, TransferSyntaxRegistry};

fn item_bytes(content: &[u8]) -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(&[0xFE, 0xFF, 0x00, 0xE0]);
    v.extend_from_slice(&(content.len() as u32).to_le_bytes());
    v.extend_from_slice(content);
    v
}

fn patient_id_element(value: &str) -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(&[0x10, 0x00, 0x20, 0x00]); // tag (0010,0020) PatientID
    v.extend_from_slice(b"LO");
    v.extend_from_slice(&(value.len() as u16).to_le_bytes());
    v.extend_from_slice(value.as_bytes());
    v
}

/// Builds a well-formed dataset with a single top-level SQ element (0008,1140),
/// EXPLICIT (defined) outer length, containing `items` PatientID items.
fn build_input(items: &[&str]) -> Vec<u8> {
    let mut content = Vec::new();
    for it in items {
        content.extend_from_slice(&item_bytes(&patient_id_element(it)));
    }

    let mut bytes = Vec::new();
    bytes.extend_from_slice(&[0x08, 0x00, 0x40, 0x11]); // tag (0008,1140)
    bytes.extend_from_slice(b"SQ");
    bytes.extend_from_slice(&[0x00, 0x00]); // reserved
    bytes.extend_from_slice(&(content.len() as u32).to_le_bytes()); // EXPLICIT length
    bytes.extend_from_slice(&content);
    bytes
}

/// Runs one variant of the repro: parse `initial_items`, mutate via the public
/// `items_mut()` path to `after_mutation_items`, then call SetVr on the same tag,
/// then round-trip through the real writer (NoChange strategy) and real reader.
/// Returns true if the bug reproduced (round-trip corrupted/failed).
fn run_variant(name: &str, initial_items: &[&str], mutate: impl FnOnce(&mut InMemDicomObject)) -> bool {
    let ts = TransferSyntaxRegistry.get("1.2.840.10008.1.2.1").unwrap();
    let seq_tag = Tag(0x0008, 0x1140);

    let input = build_input(initial_items);
    println!(
        "[{name}] input: {} bytes, {} item(s), explicit outer length",
        input.len(),
        initial_items.len()
    );

    let mut obj = InMemDicomObject::read_dataset_with_ts(input.as_slice(), ts)
        .expect("should parse well-formed input dataset");

    // Step 1: mutate the sequence's items directly via the public items_mut() path.
    mutate(&mut obj);

    let len_after_mutation = obj.element(seq_tag).unwrap().header().len;
    println!("[{name}] header.len after items_mut() mutation: {len_after_mutation:?}");
    assert!(
        len_after_mutation.is_undefined(),
        "[{name}] precondition failed: items_mut() should have reset header.len to UNDEFINED"
    );

    // Step 2: call SetVr on the same tag via the real public AttributeOp/apply API.
    obj.apply(AttributeOp::new(seq_tag, AttributeAction::SetVr(VR::SQ)))
        .expect("SetVr should succeed");

    let len_after_setvr = obj.element(seq_tag).unwrap().header().len;
    let item_count_after_setvr = match obj.element(seq_tag).unwrap().value() {
        Value::Sequence(seq) => seq.items().len(),
        _ => unreachable!(),
    };
    println!("[{name}] header.len after SetVr: {len_after_setvr:?}");
    println!("[{name}] item count after SetVr: {item_count_after_setvr}");

    // Step 3: round-trip through the real writer/reader with the one strategy that
    // writes cached explicit lengths verbatim.
    let mut out = Vec::new();
    let options = DataSetWriterOptions::default()
        .explicit_length_sq_item_strategy(ExplicitLengthSqItemStrategy::NoChange);
    obj.write_dataset_with_ts_options(&mut out, ts, options)
        .expect("write should succeed regardless of the bug");

    println!("[{name}] output: {} bytes", out.len());

    match InMemDicomObject::read_dataset_with_ts(out.as_slice(), ts) {
        Ok(reparsed) => {
            let reparsed_count = match reparsed.element(seq_tag).unwrap().value() {
                Value::Sequence(seq) => seq.items().len(),
                _ => unreachable!(),
            };
            println!(
                "[{name}] re-parsed OK: item count = {reparsed_count} (expected {item_count_after_setvr})"
            );
            if reparsed_count != item_count_after_setvr {
                println!("[{name}] BUG REPRODUCED: item count mismatch after round-trip");
                return true;
            }
            println!("[{name}] OK: round-trip preserved the sequence correctly");
            false
        }
        Err(e) => {
            println!("[{name}] BUG REPRODUCED: re-parsing the writer's own output failed: {e}");
            true
        }
    }
}

fn main() {
    let mut any_bug = false;

    // Variant A: growing the sequence (push one more item than the stale length covers).
    any_bug |= run_variant("push-growth", &["1234"], |obj| {
        let new_item = InMemDicomObject::from_element_iter(vec![dicom_core::DataElement::new(
            Tag(0x0010, 0x0020),
            VR::LO,
            PrimitiveValue::from("5678"),
        )]);
        obj.update_value_at(Tag(0x0008, 0x1140), |v| {
            if let Value::Sequence(seq) = v {
                seq.items_mut().push(new_item.clone());
            }
        })
        .expect("selector should resolve");
    });

    println!();

    // Variant B: shrinking the sequence (remove an item, fewer bytes than the stale
    // length covers).
    any_bug |= run_variant("remove-shrink", &["1234", "5678"], |obj| {
        obj.update_value_at(Tag(0x0008, 0x1140), |v| {
            if let Value::Sequence(seq) = v {
                seq.items_mut().remove(1);
            }
        })
        .expect("selector should resolve");
    });

    println!();
    if any_bug {
        println!("RESULT: bug reproduced in at least one variant");
        std::process::exit(1);
    } else {
        println!("RESULT: no bug reproduced (fix appears to be in place)");
    }
}
