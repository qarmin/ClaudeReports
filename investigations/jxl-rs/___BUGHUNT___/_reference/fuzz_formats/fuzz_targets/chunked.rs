#![no_main]
use jxl::api::*;
use libfuzzer_sys::fuzz_target;

fn fmt(kind: u8, depth: u8, endian: u8) -> Option<JxlDataFormat> {
    let endianness = if endian & 1 == 0 { Endianness::LittleEndian } else { Endianness::BigEndian };
    match kind % 5 {
        0 => None,
        1 => Some(JxlDataFormat::U8 { bit_depth: depth % 8 + 1 }),
        2 => Some(JxlDataFormat::U16 { endianness, bit_depth: depth % 16 + 1 }),
        3 => Some(JxlDataFormat::F16 { endianness }),
        _ => Some(JxlDataFormat::F32 { endianness }),
    }
}

fuzz_target!(|data: &[u8]| {
    if data.len() < 12 {
        return;
    }
    let (cfg, data) = data.split_at(12);
    let color_type = match cfg[0] % 7 {
        0 => JxlColorType::Grayscale,
        1 => JxlColorType::GrayscaleAlpha,
        2 => JxlColorType::Rgb,
        3 => JxlColorType::Rgba,
        4 => JxlColorType::Bgr,
        5 => JxlColorType::Bgra,
        _ => JxlColorType::Cmyk,
    };
    let mut opts = JxlDecoderOptions::default();
    opts.adjust_orientation = cfg[4] & 1 != 0;
    opts.render_spot_colors = cfg[4] & 2 != 0;
    opts.coalescing = cfg[4] & 4 != 0;
    opts.premultiply_output = cfg[4] & 8 != 0;
    opts.high_precision = cfg[4] & 16 != 0;
    opts.skip_preview = cfg[4] & 32 != 0;
    if cfg[4] & 64 != 0 {
        opts.desired_intensity_target = Some(cfg[5] as f32 * 10.0 + 1.0);
    }
    opts.sample_limit = Some(1 << 21);
    if cfg[9] & 1 != 0 { opts.max_profile_level = ProfileLevel::Main10; }
    if cfg[9] & 2 != 0 {
        opts.request_aux_boxes = vec![JxlAuxBoxType::EXIF, JxlAuxBoxType::XMP, JxlAuxBoxType::JUMBF, JxlAuxBoxType::JPEG_RECONSTRUCTION_DATA, JxlAuxBoxType::HDR_GAIN_MAP];
    }
    let chunk = match cfg[9] >> 2 & 3 { 0 => usize::MAX, 1 => 1 + cfg[10] as usize, 2 => 16 + cfg[10] as usize * 8, _ => 1 + cfg[11] as usize * 64 };
    let mut rest = data;
    let mut step = 0usize;
    let mut dec = JxlDecoder::new(opts);
    let mut input: &[u8] = &data[..0];
    let mut closed = false;
    let mut bufs_mem: Vec<Vec<u32>> = vec![];
    let mut formats: Vec<(usize, usize)> = vec![];
    let mut have_info = false;
    for _ in 0..10000 {
        if input.is_empty() && !rest.is_empty() {
            let n = chunk.min(rest.len());
            input = &rest[..n];
            rest = &rest[n..];
        }
        let consumed_from = input;
        let _ = consumed_from;
        let bufs_ref = if have_info && !bufs_mem.is_empty() { Some(()) } else { None };
        let mut bufs: Option<Vec<JxlOutputBuffer>> = bufs_ref.map(|_| {
            bufs_mem
                .iter_mut()
                .zip(formats.iter())
                .map(|(m, &(rows, bpr))| {
                    let bytes: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(m.as_mut_ptr() as *mut u8, m.len() * 4) };
                    JxlOutputBuffer::new(bytes, rows, bpr)
                })
                .collect()
        });
        let ev = match dec.process(&mut input, bufs.as_deref_mut(), None) {
            Ok(e) => e,
            Err(_) => return,
        };
        match ev {
            Event::BasicInfo => {
                let info = dec.basic_info().unwrap().clone();
                let n = info.extra_channels.len();
                let pf = JxlPixelFormat {
                    color_type,
                    color_data_format: fmt(cfg[1], cfg[2], cfg[3]),
                    extra_channel_format: (0..n)
                        .map(|i| fmt(cfg[6].rotate_right(i as u32 % 8), cfg[7], cfg[8]))
                        .collect(),
                };
                if dec.set_pixel_format(pf.clone()).is_err() {
                    return;
                }
                let (w, h) = info.size;
                if w == 0 || h == 0 {
                    return;
                }
                bufs_mem.clear();
                formats.clear();
                let mut add = |f: &Option<JxlDataFormat>, spp: usize| {
                    if let Some(f) = f {
                        let bpr = w * spp * f.bytes_per_sample();
                        bufs_mem.push(vec![0u32; (bpr * h).div_ceil(4)]);
                        formats.push((h, bpr));
                    }
                };
                add(&pf.color_data_format, color_type.samples_per_pixel());
                for f in &pf.extra_channel_format {
                    add(f, 1);
                }
                have_info = true;
            }
            Event::FrameComplete { .. } => {}
            Event::FrameHeader => {}
            Event::Complete => {
                for t in [JxlAuxBoxType::EXIF, JxlAuxBoxType::XMP, JxlAuxBoxType::JUMBF] {
                    for b in dec.aux_boxes(t) {
                        let _ = b.raw_data().len();
                    }
                }
                return;
            }
            Event::NeedMoreInput { .. } => {
                step += 1;
                if cfg[9] & 32 != 0 && have_info && !bufs_mem.is_empty() {
                    let mut fb: Vec<JxlOutputBuffer> = bufs_mem
                        .iter_mut()
                        .zip(formats.iter())
                        .map(|(m, &(rows, bpr))| {
                            let bytes: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(m.as_mut_ptr() as *mut u8, m.len() * 4) };
                            JxlOutputBuffer::new(bytes, rows, bpr)
                        })
                        .collect();
                    if dec.flush_pixels(&mut fb, None).is_err() {
                        return;
                    }
                }
                if input.is_empty() && rest.is_empty() {
                    if !closed && cfg[9] & 2 != 0 {
                        dec.close_input();
                        closed = true;
                    } else {
                        return;
                    }
                }
            }
        }
    }
});
