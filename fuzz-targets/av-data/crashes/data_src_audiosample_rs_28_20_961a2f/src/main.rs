use av_data::audiosample::formats::F32;

fn main() {
    let soniton = F32;
    let _ = soniton.get_audio_size(1usize, 0usize);
}
