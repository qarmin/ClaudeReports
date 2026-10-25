use epaint::textures::TextureMeta;
use epaint::textures::TextureOptions;

fn main() {
    let texture_meta = TextureMeta { name: "".to_string(), size: [1000000000usize, 1000000000usize], bytes_per_pixel: 1000usize, retain_count: 0usize, options: TextureOptions::NEAREST_MIRRORED_REPEAT };
    let _ = texture_meta.bytes_used();
}
