use allsorts::font_specimen::SpecimenOptions;

fn main() {
    let _ = allsorts::font_specimen::specimen("", &include_bytes!("data_0.bin").to_vec(), SpecimenOptions::default());
}
