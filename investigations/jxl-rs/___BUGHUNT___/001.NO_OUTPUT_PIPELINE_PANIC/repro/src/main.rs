use jxl::api::*;

fn main() {
    let data = include_bytes!("../input.jxl");
    let mut input: &[u8] = data;
    let mut dec = JxlDecoder::new(JxlDecoderOptions::default());
    loop {
        match dec.process(&mut input, None, None).expect("decode error") {
            Event::BasicInfo => {
                let n = dec.basic_info().unwrap().extra_channels.len();
                // None = "ignore this channel": ask for no pixel output at all.
                dec.set_pixel_format(JxlPixelFormat {
                    color_type: JxlColorType::Rgb,
                    color_data_format: None,
                    extra_channel_format: vec![None; n],
                })
                .unwrap();
            }
            Event::Complete => {
                println!("completed normally");
                return;
            }
            Event::NeedMoreInput { .. } if input.is_empty() => panic!("truncated"),
            _ => {}
        }
    }
}
