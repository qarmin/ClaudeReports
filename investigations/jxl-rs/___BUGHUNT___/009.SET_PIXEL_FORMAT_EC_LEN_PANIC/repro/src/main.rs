use jxl::api::*;

fn main() {
    let data = include_bytes!("../input.jxl");
    let mut input: &[u8] = data;
    let mut dec = JxlDecoder::new(JxlDecoderOptions::default());
    loop {
        match dec.process(&mut input, None, None).expect("decode error") {
            Event::BasicInfo => {
                let n = dec.basic_info().unwrap().extra_channels.len();
                println!("image has {n} extra channel(s); requesting a pixel format with 0 entries");
                // Wrong length: set_pixel_format is documented to return a Result, but accepts this.
                match dec.set_pixel_format(JxlPixelFormat::rgb8(0)) {
                    Ok(()) => println!("set_pixel_format returned Ok"),
                    Err(e) => {
                        println!("set_pixel_format returned Err: {e:?}");
                        return;
                    }
                }
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
