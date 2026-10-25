use ecolor::Color32;
use emath::Rect;
use epaint::Mesh;
use epaint::TextureId;

fn main() {
    let mut mesh = Mesh::with_texture(TextureId::User(1u64));
    mesh.add_colored_rect(Rect::ZERO, Color32::DARK_BLUE);
}
