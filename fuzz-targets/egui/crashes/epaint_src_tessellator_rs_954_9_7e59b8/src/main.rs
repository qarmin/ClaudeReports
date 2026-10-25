use ecolor::Color32;
use epaint::Mesh;
use epaint::TextureId;
use epaint::tessellator::Path;

fn main() {
    let mut path = Path::default();
    let mut mesh = Mesh::from(Mesh { indices: vec![0u32], vertices: vec![], texture_id: TextureId::default() });
    path.fill_with_uv(0.0f32, Color32::from_rgba_unmultiplied_const(0u8, 0u8, 0u8, 1u8), TextureId::Managed(1u64), |a| a, &mut mesh);
}
