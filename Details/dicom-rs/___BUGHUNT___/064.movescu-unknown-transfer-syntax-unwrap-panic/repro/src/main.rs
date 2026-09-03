// `movescu` is a binary-only crate (no library target), so this repro can't
// call its private receiving-loop functions directly. Instead, this
// replicates the exact vulnerable computation from
// `movescu/src/store_async.rs` verbatim - see patch.diff's context for the
// real source lines this mirrors:
//
//     let ts = &presentation_context.transfer_syntax;
//     let obj = InMemDicomObject::read_dataset_with_ts(
//         instance_buffer.as_slice(),
//         TransferSyntaxRegistry.get(ts).unwrap(),
//     )
//
// `presentation_context.transfer_syntax` is a `String` field on a plain,
// publicly-constructible struct, populated during association negotiation
// (movescu briefly acts as a temporary SCP to receive the C-STORE sub-
// operations of a C-MOVE) - this line assumes, with no local check of its
// own, that whatever ends up in that field is always a transfer syntax UID
// this crate's registry recognizes. Same shape and same caveat as candidate
// 058's storescp finding.
//
// This repro demonstrates the exact panic that results the moment this
// specific line is ever reached with a transfer syntax UID the registry
// doesn't recognize - regardless of how such a UID might come to occupy
// that field.

use dicom_encoding::transfer_syntax::TransferSyntaxIndex;
use dicom_transfer_syntax_registry::TransferSyntaxRegistry;

fn main() {
    let ts = "1.2.3.4.5.6.7.8.9.999.not.a.real.transfer.syntax";
    println!("looking up transfer syntax UID: {ts:?}");

    // this is the exact line from movescu/src/store_async.rs
    let _ts_entry = TransferSyntaxRegistry.get(ts).unwrap();

    println!("did not panic (unexpected)");
}
