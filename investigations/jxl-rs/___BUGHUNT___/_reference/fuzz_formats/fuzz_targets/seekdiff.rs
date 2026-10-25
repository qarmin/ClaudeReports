#![no_main]
use jxl::api::*;
use jxl_fuzz_formats::{FuzzConfig, create_frame_buffers, fuzz_decode, make_output_bufs};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 9 {
        return;
    }
    let (cfg, file) = data.split_at(8);
    let Ok(sequential) = fuzz_decode(file, FuzzConfig::default()) else { return };
    if sequential.is_empty() {
        return;
    }
    let mut sopts = JxlDecoderOptions::default();
    sopts.scan_frames_only = true;
    sopts.sample_limit = Some(1 << 22);
    let mut scan = JxlDecoder::new(sopts);
    let mut input = file;
    loop {
        match scan.process(&mut input, None, None) {
            Ok(Event::Complete) => break,
            Ok(Event::NeedMoreInput { .. }) if input.is_empty() => panic!("scan truncated where decode succeeded"),
            Ok(_) => {}
            Err(e) => panic!("scan failed where decode succeeded: {e:?}"),
        }
    }
    let frames = scan.scanned_frames().to_vec();
    assert_eq!(frames.len(), sequential.len(), "scanned frame count != decoded frame count");

    let mut opts = JxlDecoderOptions::default();
    opts.sample_limit = Some(1 << 22);
    let mut dec = JxlDecoder::new(opts);
    let mut input = file;
    loop {
        match dec.process(&mut input, None, None).expect("seek decoder basic info") {
            Event::BasicInfo => break,
            Event::NeedMoreInput { .. } if input.is_empty() => panic!("truncated"),
            _ => {}
        }
    }
    for s in 0..4 {
        let target = cfg[s] as usize % frames.len();
        let count = 1 + cfg[4 + s] as usize % 3;
        let offset = dec.start_new_frame(frames[target].seek_target).expect("start_new_frame") as usize;
        let mut input = &file[offset.min(file.len())..];
        for i in target..(target + count).min(frames.len()) {
            let ev = dec.process(&mut input, None, None).unwrap_or_else(|e| panic!("seek to {target}, frame {i}: header: {e:?}"));
            assert_eq!(ev, Event::FrameHeader, "seek to {target}, frame {i}");
            let mut out = create_frame_buffers(&dec).expect("alloc");
            let mut bufs = make_output_bufs(&mut out);
            let ev = dec.process(&mut input, Some(&mut bufs), None).unwrap_or_else(|e| panic!("seek to {target}, frame {i}: pixels: {e:?}"));
            assert!(matches!(ev, Event::FrameComplete { .. }), "seek to {target}, frame {i}: {ev:?}");
            drop(bufs);
            for (c, (x, y)) in out.iter().zip(sequential[i].iter()).enumerate() {
                assert_eq!(x.size(), y.size());
                for row in 0..x.size().1 {
                    for (px, (p, q)) in x.row(row).iter().zip(y.row(row).iter()).enumerate() {
                        assert!(p == q || (p.is_nan() && q.is_nan()), "seek to {target}, frame {i}, ch {c} ({px},{row}): {p} vs {q}");
                    }
                }
            }
        }
    }
});
