use epaint::Mesh;

fn main() {
    let mut mesh = Mesh::from(Mesh::default());
    mesh.reserve_triangles(usize::MAX);
}
