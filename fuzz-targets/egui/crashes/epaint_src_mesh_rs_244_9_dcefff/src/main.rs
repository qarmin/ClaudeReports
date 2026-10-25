use epaint::Mesh;
use epaint::TextureId;

fn main() {
    let mesh = Mesh { indices: vec![0u32], vertices: vec![], texture_id: TextureId::User(0u64) };
    let _ = mesh.clone().split_to_u16();
}
