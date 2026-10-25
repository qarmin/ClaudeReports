use av_data::audiosample::ChannelMap;

fn main() {
    let channel_map = ChannelMap::new();
    let _ = channel_map.get_channel(0usize);
}
