use epaint::Mesh;
use epaint::Shape;
use epaint::TextureId;

fn main() {
    let _ = Shape::mesh(std::sync::Arc::new(Mesh { indices: vec![1u32], vertices: vec![], texture_id: TextureId::Managed(0u64) }));
}
