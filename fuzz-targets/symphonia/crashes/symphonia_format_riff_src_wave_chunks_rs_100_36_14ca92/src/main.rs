use symphonia_format_riff::WavReader;

fn main() {
    let _ = WavReader::try_new({ let bytes = include_bytes!("data_0.bin").to_vec(); symphonia_core::io::MediaSourceStream::new(Box::new(std::io::Cursor::new(bytes)), Default::default()) }, { symphonia_core::formats::FormatOptions::default() });
}
