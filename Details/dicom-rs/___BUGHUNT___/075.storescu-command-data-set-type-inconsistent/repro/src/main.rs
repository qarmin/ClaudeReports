// `storescu` is a binary-only crate (no library target), and `store_req_command` is a private
// function, so this repro can't call the vulnerable code path directly. Instead, it replicates
// the exact vulnerable computation verbatim - see patch.diff's context for the real source
// lines this mirrors (storescu/src/main.rs's `store_req_command`):
//
//     DataElement::new(tags::COMMAND_DATA_SET_TYPE, VR::US, dicom_value!(U16, [0x0000])),
//
// This demonstrates the same value storescu currently sends in its C-STORE-RQ command, compared
// against what its own sibling tools (findscu, movescu) send for the identical situation (a
// command that carries an accompanying data set).

use dicom_core::{dicom_value, DataElement, VR};
use dicom_dictionary_std::tags;
use dicom_object::InMemDicomObject;

fn main() {
    // storescu's current (buggy) CommandDataSetType value
    let storescu_cmd = InMemDicomObject::command_from_element_iter([DataElement::new(
        tags::COMMAND_DATA_SET_TYPE,
        VR::US,
        dicom_value!(U16, [0x0000]),
    )]);
    let storescu_value = storescu_cmd
        .element(tags::COMMAND_DATA_SET_TYPE)
        .unwrap()
        .to_int::<u16>()
        .unwrap();

    // findscu's and movescu's own value for the identical situation (a command
    // that carries an accompanying data set) - see findscu/src/main.rs:435 and
    // movescu/src/main.rs:507, both use dicom_value!(U16, [0x0001])
    let sibling_value: u16 = 0x0001;

    println!("storescu's CommandDataSetType:            {storescu_value:#06x}");
    println!("findscu/movescu's CommandDataSetType:     {sibling_value:#06x}");

    // Per PS3.7 Table 6.3-1, only 0x0101 specifically means "no Data Set
    // present"; any other value technically means "Data Set present" - but
    // storescu's own sibling tools in this exact codebase both settled on
    // 0x0001 for a C-STORE-RQ/C-FIND-RQ/C-MOVE-RQ-style command that carries
    // a data set, and strict/non-conformant SCP implementations in practice
    // may reject 0x0000 specifically.
    assert_eq!(
        storescu_value, sibling_value,
        "storescu's CommandDataSetType ({storescu_value:#06x}) is inconsistent with its own \
         sibling tools' value ({sibling_value:#06x}) for the identical situation"
    );
    println!("consistent with sibling tools");
}
