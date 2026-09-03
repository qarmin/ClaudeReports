use dicom_core::value::{AsRange, DicomTime};

fn main() {
    // second=60 is a valid DICOM TM value (leap second) - see partial.rs's
    // own `check_component` range for `DateComponent::Second`: `0..=60`.
    let t = DicomTime::from_hms(23, 59, 60).expect("60 should be a valid second value");
    println!("constructed leap-second DicomTime: {t:?}");

    println!("calling earliest()...");
    let earliest = t.earliest();
    println!("earliest() = {earliest:?}");
    earliest.expect("earliest() should succeed for a valid TM value");

    println!("calling latest()...");
    let latest = t.latest();
    println!("latest() = {latest:?}");
    latest.expect("latest() should succeed for a valid TM value");

    println!("done, no error");
}
