use epaint::Mesh;
use epaint::TextureId;

fn main() {
    let mut mesh = Mesh::with_texture(TextureId::User(0u64));
    mesh.reserve_triangles(10000000000000usize);
}
