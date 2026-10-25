use gdk4::DmabufTextureBuilder;

fn main() {
    let dmabuf_texture_builder = DmabufTextureBuilder::new();
    let dmabuf_texture_builder2 = dmabuf_texture_builder.clone().set_stride(0u32, 0u32);
    let dmabuf_texture_builder3 = dmabuf_texture_builder2.clone().set_stride(0u32, 1u32);
    let _ = dmabuf_texture_builder3.display();
}
