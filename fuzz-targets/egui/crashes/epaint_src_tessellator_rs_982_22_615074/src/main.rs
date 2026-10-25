use egui::Visuals;
use epaint::Mesh;
use epaint::TextureId;
use epaint::tessellator::Path;

fn main() {
    let mut path = Path::default();
    let mut mesh = Mesh::with_texture(TextureId::default());
    path.fill_with_uv(1.0f32, Visuals::light().weak_text_color(), TextureId::User(0u64), |a| a, &mut mesh);
}
