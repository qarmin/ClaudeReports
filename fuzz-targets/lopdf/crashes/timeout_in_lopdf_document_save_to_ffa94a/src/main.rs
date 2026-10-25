use lopdf::Document;

fn main() {
    let mut document = Document::new_from_prev(&Document::new());
    document.renumber_objects_with(2453330354u32);
    let _ = document.save_to::<Vec<u8>>(&mut include_bytes!("image_0.tiff").to_vec());
}
