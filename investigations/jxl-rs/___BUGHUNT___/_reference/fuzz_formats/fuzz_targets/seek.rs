#![no_main]
use jxl::api::*;
use libfuzzer_sys::fuzz_target;

fn alloc(dec: &JxlDecoder) -> Option<(Vec<Vec<u32>>, Vec<(usize, usize)>)> {
    let info = dec.basic_info()?;
    let (w, h) = info.size;
    if w == 0 || h == 0 {
        return None;
    }
    let pf = dec.current_pixel_format()?;
    let mut mem = vec![];
    let mut dims = vec![];
    let mut add = |f: &Option<JxlDataFormat>, spp: usize| {
        if let Some(f) = f {
            let bpr = w * spp * f.bytes_per_sample();
            mem.push(vec![0u32; (bpr * h).div_ceil(4)]);
            dims.push((h, bpr));
        }
    };
    add(&pf.color_data_format, pf.color_type.samples_per_pixel());
    for f in &pf.extra_channel_format {
        add(f, 1);
    }
    Some((mem, dims))
}

fuzz_target!(|data: &[u8]| {
    if data.len() < 8 {
        return;
    }
    let (cfg, file) = data.split_at(8);
    let mut sopts = JxlDecoderOptions::default();
    sopts.scan_frames_only = true;
    sopts.sample_limit = Some(1 << 20);
    let mut scan = JxlDecoder::new(sopts);
    let mut input = file;
    loop {
        match scan.process(&mut input, None, None) {
            Ok(Event::Complete) => break,
            Ok(Event::NeedMoreInput { .. }) if input.is_empty() => break,
            Ok(_) => {}
            Err(_) => break,
        }
    }
    let frames: Vec<_> = scan.scanned_frames().to_vec();
    if frames.is_empty() {
        return;
    }
    let mut opts = JxlDecoderOptions::default();
    opts.sample_limit = Some(1 << 20);
    opts.coalescing = cfg[0] & 1 != 0;
    let mut dec = JxlDecoder::new(opts);
    let mut pos: usize = 0;
    let mut mem: Vec<Vec<u32>> = vec![];
    let mut dims = vec![];
    let mut seeks = 0;
    let mut step = 0usize;
    for _ in 0..5000 {
        let mut input = &file[pos.min(file.len())..];
        let before = input.len();
        let mut bufs: Option<Vec<JxlOutputBuffer>> = if mem.is_empty() {
            None
        } else {
            Some(
                mem.iter_mut()
                    .zip(dims.iter())
                    .map(|(m, &(r, b)): (&mut Vec<u32>, &(usize, usize))| {
                        let s: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(m.as_mut_ptr() as *mut u8, m.len() * 4) };
                        JxlOutputBuffer::new(s, r, b)
                    })
                    .collect(),
            )
        };
        let ev = match dec.process(&mut input, bufs.as_deref_mut(), None) {
            Ok(e) => e,
            Err(_) => return,
        };
        pos += before - input.len();
        step += 1;
        match ev {
            Event::BasicInfo => {
                let Some((m, d)) = alloc(&dec) else { return };
                mem = m;
                dims = d;
            }
            Event::FrameComplete { .. } | Event::FrameHeader => {
                let r = cfg[1 + seeks % 7] as usize;
                if (step + r) % 3 == 0 && seeks < 6 {
                    let k = cfg[1 + (seeks + 3) % 7] as usize % frames.len();
                    seeks += 1;
                    match dec.start_new_frame(frames[k].seek_target) {
                        Ok(off) => pos = off as usize,
                        Err(_) => return,
                    }
                }
            }
            Event::Complete => {
                if seeks < 6 {
                    let k = cfg[1 + (seeks + 3) % 7] as usize % frames.len();
                    seeks += 1;
                    match dec.start_new_frame(frames[k].seek_target) {
                        Ok(off) => pos = off as usize,
                        Err(_) => return,
                    }
                } else {
                    return;
                }
            }
            Event::NeedMoreInput { .. } => {
                if pos >= file.len() {
                    return;
                }
            }
        }
    }
});
