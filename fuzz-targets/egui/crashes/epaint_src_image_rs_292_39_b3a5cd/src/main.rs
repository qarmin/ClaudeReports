use egui::scroll_area::State;
use egui::util::IdTypeMap;
use epaint::ColorImage;

fn main() {
    let id_type_map = IdTypeMap::default();
    let usize_v = id_type_map.count::<u8>();
    let color_image = ColorImage { size: [1usize, 1usize], source_size: State::default().overscroll(), pixels: vec![] };
    let _ = color_image.region_by_pixels([0usize, usize_v], [1usize, 1usize]);
}
