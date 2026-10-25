use jxl::api::*;

struct Out {
    w: usize,
    h: usize,
    spp: usize,
    data: Vec<f64>, // normalized color pixels (interleaved) 
    ec: Vec<Vec<f64>>,
}

fn decode(bytes: &[u8], ct: JxlColorType, df: JxlDataFormat, premul: bool, orient: bool) -> Result<Out, String> {
    let mut o = JxlDecoderOptions::default();
    o.premultiply_output = premul;
    o.adjust_orientation = orient;
    let mut dec = JxlDecoder::new(o);
    let mut input = bytes;
    let mut mem: Vec<Vec<u32>> = vec![];
    let mut dims = vec![];
    let mut size = (0, 0);
    let mut spp = 0;
    let mut nec = 0;
    let mut fmts: Vec<JxlDataFormat> = vec![];
    let mut done_frame = false;
    loop {
        let mut bufs: Option<Vec<JxlOutputBuffer>> = if mem.is_empty() { None } else {
            Some(mem.iter_mut().zip(dims.iter()).map(|(m, &(r, b))| {
                let s: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(m.as_mut_ptr() as *mut u8, m.len() * 4) };
                JxlOutputBuffer::new(s, r, b)
            }).collect())
        };
        let ev = dec.process(&mut input, bufs.as_deref_mut(), None).map_err(|e| format!("{e:?}"))?;
        match ev {
            Event::BasicInfo => {
                let info = dec.basic_info().unwrap().clone();
                nec = info.extra_channels.len();
                let gray = dec.embedded_color_profile().unwrap().channels() == 1;
                let ct = if gray && !matches!(ct, JxlColorType::Grayscale | JxlColorType::GrayscaleAlpha) {
                    if ct.has_alpha() { JxlColorType::GrayscaleAlpha } else { JxlColorType::Grayscale }
                } else { ct };
                if info.animation.is_some() { return Err("anim".into()); }
                spp = ct.samples_per_pixel();
                size = info.size;
                let pf = JxlPixelFormat { color_type: ct, color_data_format: Some(df), extra_channel_format: vec![Some(df); nec] };
                dec.set_pixel_format(pf).map_err(|e| format!("{e:?}"))?;
                fmts.push(df);
                for c in 0..=nec {
                    let s = if c == 0 { spp } else { 1 };
                    let bpr = size.0 * s * df.bytes_per_sample();
                    mem.push(vec![0u32; (bpr * size.1).div_ceil(4)]);
                    dims.push((size.1, bpr));
                }
            }
            Event::FrameComplete { .. } => { done_frame = true; }
            Event::Complete => break,
            Event::NeedMoreInput { .. } => { if input.is_empty() { return Err("truncated".into()); } }
            _ => {}
        }
        if done_frame { break; }
    }
    let conv = |m: &Vec<u32>, n: usize| -> Vec<f64> {
        let b: &[u8] = unsafe { std::slice::from_raw_parts(m.as_ptr() as *const u8, m.len() * 4) };
        match df {
            JxlDataFormat::U8 { bit_depth } => b[..n].iter().map(|&x| x as f64 / ((1u32 << bit_depth) - 1) as f64).collect(),
            JxlDataFormat::U16 { bit_depth, .. } => (0..n / 2).map(|i| u16::from_ne_bytes([b[2 * i], b[2 * i + 1]]) as f64 / ((1u32 << bit_depth) - 1) as f64).collect(),
            JxlDataFormat::F32 { .. } => (0..n / 4).map(|i| f32::from_ne_bytes(b[4 * i..4 * i + 4].try_into().unwrap()) as f64).collect(),
            JxlDataFormat::F16 { .. } => (0..n / 2).map(|i| half_to_f64(u16::from_ne_bytes([b[2 * i], b[2 * i + 1]]))).collect(),
        }
    };
    let n0 = size.0 * size.1 * spp * df.bytes_per_sample();
    let data = conv(&mem[0], n0);
    let ec = (1..=nec).map(|c| conv(&mem[c], size.0 * size.1 * df.bytes_per_sample())).collect();
    Ok(Out { w: size.0, h: size.1, spp, data, ec })
}

fn half_to_f64(h: u16) -> f64 {
    let s = if h >> 15 != 0 { -1.0 } else { 1.0 };
    let e = ((h >> 10) & 31) as i32;
    let m = (h & 1023) as f64;
    if e == 0 { s * m * 2f64.powi(-24) } else if e == 31 { f64::NAN } else { s * (1.0 + m / 1024.0) * 2f64.powi(e - 15) }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args[0] == "ec" { ecinfo(&args[1]); return; }
    if args[0] == "ecd" { ecdiff(&args[1]); return; }
    if args[0] == "pm" { for p in &args[1..] { premul(p); } println!("done"); return; }
    if args[0] == "pmd" { pmdump(&args[1], JxlDataFormat::U8 { bit_depth: 8 }, 12); return; }
    if args[0] == "try" { for p in &args[1..] { let b = std::fs::read(p).unwrap(); match decode(&b, JxlColorType::Rgb, JxlDataFormat::f32(), false, true) { Ok(o) => println!("{p}: OK {}x{}", o.w, o.h), Err(e) => println!("{p}: {e}") } } return; }
    if args[0] == "or" { for p in &args[1..] { orient(p); } println!("done"); return; }
    if args[0] == "mis" { misalign(&args[1]); return; }
    if args[0] == "misuse" { misuse(&args[1]); return; }
    if args[0] == "misuse2" { misuse2(&args[1]); return; }
    if args[0] == "misuse3" { misuse3(&args[1]); return; }
    if args[0] == "stats" { stats(&args[1]); return; }
    let fmts = [
        JxlDataFormat::U8 { bit_depth: 8 },
        JxlDataFormat::U16 { endianness: Endianness::native(), bit_depth: 16 },
        JxlDataFormat::F16 { endianness: Endianness::native() },
        JxlDataFormat::F32 { endianness: Endianness::native() },
    ];
    for path in &args {
        let bytes = std::fs::read(path).unwrap();
        for premul in [false, true] {
        let mut refs = vec![];
        for &df in &fmts {
            match decode(&bytes, JxlColorType::Rgb, df, premul, true) {
                Ok(o) => refs.push(o),
                Err(e) => { println!("{path} {df:?}: ref fail {e}"); refs.clear(); break; }
            }
        }
        if refs.len() != 4 { continue; }
        let gray = refs[0].spp == 1;
        let colors = if gray { vec![JxlColorType::Grayscale, JxlColorType::GrayscaleAlpha] } else { vec![JxlColorType::Rgb, JxlColorType::Rgba, JxlColorType::Bgr, JxlColorType::Bgra] };
        for (fi, &df) in fmts.iter().enumerate() {
            let r = &refs[fi];
            for ct in &colors {
                let o = match decode(&bytes, *ct, df, premul, true) { Ok(o) => o, Err(e) => { println!("{path} {ct:?} {df:?}: FAIL {e}"); continue } };
                let cs = if gray { 1 } else { 3 };
                let spp = ct.samples_per_pixel();
                let mut worst: f64 = 0.0; let mut wi = 0;
                for p in 0..r.w * r.h {
                    for c in 0..cs {
                        let oc = if matches!(ct, JxlColorType::Bgr | JxlColorType::Bgra) { 2 - c } else { c };
                        let d = (o.data[p * spp + oc] - r.data[p * r.spp + c]).abs();
                        if d > worst || d.is_nan() { worst = d; wi = p; }
                    }
                }
                // premultiplied output with alpha interleaved should differ from non-alpha ref; only compare when no alpha
                if ct.has_alpha() && premul { continue; }
                if worst > 0.0 || worst.is_nan() { println!("{path} premul={premul} {ct:?} {df:?}: differs from Rgb ref of same format: worst={worst} px {wi}"); }
            }
            // cross-format consistency vs F32 (only meaningful when same output colour space)
        }
        // u8 vs u16
        let (a, b) = (&refs[0], &refs[1]);
        let mut worst = 0.0f64; let mut wi = 0;
        for (i, (x, y)) in a.data.iter().zip(b.data.iter()).enumerate() { let d = (x - y).abs(); if d > worst { worst = d; wi = i; } }
        if worst > 1.0 / 255.0 * 0.51 + 1e-6 { println!("{path} premul={premul}: U8 vs U16 worst={worst} at {wi}"); }
        // f16 vs f32
        let (a, b) = (&refs[2], &refs[3]);
        let mut worst = 0.0f64; let mut wi = 0;
        for (i, (x, y)) in a.data.iter().zip(b.data.iter()).enumerate() { let d = (x - y).abs() / (1.0 + y.abs()); if d > worst || d.is_nan() { worst = d; wi = i; } }
        if worst > 2e-3 || worst.is_nan() { println!("{path} premul={premul}: F16 vs F32 worst-rel={worst} at {wi}"); }
        // extra channels agree across formats
        for k in 0..refs[3].ec.len() {
            let (a, b) = (&refs[0].ec[k], &refs[3].ec[k]);
            let mut worst = 0.0f64; let mut wi = 0;
            for (i, (x, y)) in a.iter().zip(b.iter()).enumerate() { let d = (x - y).abs(); if d > worst { worst = d; wi = i; } }
            if worst > 0.51 / 255.0 + 1e-6 { println!("{path} premul={premul}: EC{k} U8 vs F32 worst={worst} at {wi}"); }
        }
        }
    }
    println!("done");
}
#[allow(dead_code)]
pub fn stats(path: &str) {
    let bytes = std::fs::read(path).unwrap();
    let a = decode(&bytes, JxlColorType::Rgb, JxlDataFormat::U8 { bit_depth: 8 }, false, true).unwrap();
    let b = decode(&bytes, JxlColorType::Rgb, JxlDataFormat::F32 { endianness: Endianness::native() }, false, true).unwrap();
    let c = decode(&bytes, JxlColorType::Rgb, JxlDataFormat::U16 { endianness: Endianness::native(), bit_depth: 16 }, false, true).unwrap();
    let mut hist = std::collections::BTreeMap::new();
    let mut hist2 = std::collections::BTreeMap::new();
    for ((x, y), z) in a.data.iter().zip(c.data.iter()).zip(b.data.iter()) {
        *hist.entry(((x - y) * 255.0 * 4.0).round() as i32).or_insert(0usize) += 1;
        let _ = z;
        *hist2.entry(((x * 255.0) - 0.0).round() as i32 - (y * 255.0).round() as i32).or_insert(0usize) += 1;
    }
    println!("{path} u8-u16 (quarter levels): {hist:?}\n levels vs round(u16/257): {hist2:?}");
}
#[allow(dead_code)]
pub fn ecinfo(path: &str) {
    let bytes = std::fs::read(path).unwrap();
    let mut dec = JxlDecoder::new(JxlDecoderOptions::default());
    let mut input: &[u8] = &bytes;
    loop {
        if let Event::BasicInfo = dec.process(&mut input, None, None).unwrap() { break; }
    }
    let i = dec.basic_info().unwrap(); println!("{:?}", i.extra_channels.iter().map(|e| (e.ec_type, e.alpha_associated)).collect::<Vec<_>>()); println!("bd {}", i.bit_depth.bits_per_sample());
}
#[allow(dead_code)]
pub fn ecdiff(path: &str) {
    let bytes = std::fs::read(path).unwrap();
    let mut outs = vec![];
    for df in [JxlDataFormat::U8 { bit_depth: 8 }, JxlDataFormat::U16 { endianness: Endianness::native(), bit_depth: 16 }, JxlDataFormat::F32 { endianness: Endianness::native() }] {
        outs.push(decode(&bytes, JxlColorType::Rgb, df, false, true).unwrap());
    }
    let (w, _h) = (outs[0].w, outs[0].h);
    let mut n = 0;
    for i in 0..outs[0].ec[0].len() {
        let (a, b, c) = (outs[0].ec[0][i], outs[1].ec[0][i], outs[2].ec[0][i]);
        if (a - c).abs() > 0.004 || (b - c).abs() > 1e-4 {
            if n < 10 { println!("px ({},{}) u8={:.4} u16={:.4} f32={:.5}", i % w, i / w, a, b, c); }
            n += 1;
        }
    }
    println!("{n} differing of {}", outs[0].ec[0].len());
}
#[allow(dead_code)]
pub fn premul(path: &str) {
    let bytes = std::fs::read(path).unwrap();
    for df in [JxlDataFormat::U8 { bit_depth: 8 }, JxlDataFormat::U16 { endianness: Endianness::native(), bit_depth: 16 }, JxlDataFormat::F32 { endianness: Endianness::native() }] {
      for ct in [JxlColorType::Rgba, JxlColorType::Bgra] {
        let (Ok(a), Ok(b)) = (decode(&bytes, ct, df, false, true), decode(&bytes, ct, df, true, true)) else { return };
        if a.spp != 4 { return; }
        let mut worst = 0.0f64; let mut wi = 0; let mut aw = 0.0f64;
        for p in 0..a.w * a.h {
            let al = a.data[p * 4 + 3];
            aw = aw.max((al - b.data[p * 4 + 3]).abs());
            for c in 0..3 {
                let d = (a.data[p * 4 + c] * al - b.data[p * 4 + c]).abs();
                if d > worst { worst = d; wi = p; }
            }
        }
        if worst > 0.012 || aw > 0.0 { println!("{path} {df:?} {ct:?}: premul worst={worst} at {wi} alpha_diff={aw}"); }
      }
    }
}
#[allow(dead_code)]
pub fn pmdump(path: &str, df: JxlDataFormat, n: usize) {
    let bytes = std::fs::read(path).unwrap();
    let a = decode(&bytes, JxlColorType::Rgba, df, false, true).unwrap();
    let b = decode(&bytes, JxlColorType::Rgba, df, true, true).unwrap();
    let mut shown = 0;
    for p in 0..a.w * a.h {
        let al = a.data[p * 4 + 3];
        for c in 0..3 {
            let d = (a.data[p * 4 + c] * al - b.data[p * 4 + c]).abs();
            if d > 0.2 && shown < n { shown += 1; println!("px ({},{}) c{c}: straight={:.4} alpha={:.4} premul_out={:.4} expected={:.4}", p % a.w, p / a.w, a.data[p*4+c], al, b.data[p*4+c], a.data[p*4+c]*al); }
        }
    }
}
fn orient_info(bytes: &[u8]) -> Option<Orientation> {
    let mut dec = JxlDecoder::new(JxlDecoderOptions::default());
    let mut input: &[u8] = bytes;
    loop {
        match dec.process(&mut input, None, None).ok()? {
            Event::BasicInfo => return Some(dec.basic_info().unwrap().orientation),
            Event::NeedMoreInput { .. } if input.is_empty() => return None,
            _ => {}
        }
    }
}
pub fn orient(path: &str) {
    let bytes = std::fs::read(path).unwrap();
    let Some(o) = orient_info(&bytes) else { return };
    for df in [JxlDataFormat::U8 { bit_depth: 8 }, JxlDataFormat::U16 { endianness: Endianness::native(), bit_depth: 16 }, JxlDataFormat::F16 { endianness: Endianness::native() }, JxlDataFormat::F32 { endianness: Endianness::native() }] {
        for ct in [JxlColorType::Rgb, JxlColorType::Rgba, JxlColorType::Bgr, JxlColorType::Bgra, JxlColorType::Grayscale, JxlColorType::GrayscaleAlpha] {
            let (Ok(raw), Ok(adj)) = (decode(&bytes, ct, df, false, false), decode(&bytes, ct, df, false, true)) else { println!("{path} {df:?} {ct:?}: decode failed"); continue };
            let (w, h) = (raw.w, raw.h);
            let (ow, oh) = if matches!(o, Orientation::Transpose | Orientation::AntiTranspose | Orientation::Rotate90Cw | Orientation::Rotate90Ccw) { (h, w) } else { (w, h) };
            if (adj.w, adj.h) != (ow, oh) || raw.spp != adj.spp { println!("{path} {df:?} {ct:?}: size {}x{} vs expected {ow}x{oh}", adj.w, adj.h); continue; }
            let spp = raw.spp;
            let mut bad = 0; let mut first = None;
            for y in 0..h { for x in 0..w {
                let (tx, ty) = match o {
                    Orientation::Identity => (x, y), Orientation::FlipHorizontal => (w - 1 - x, y), Orientation::Rotate180 => (w - 1 - x, h - 1 - y),
                    Orientation::FlipVertical => (x, h - 1 - y), Orientation::Transpose => (y, x), Orientation::Rotate90Cw => (h - 1 - y, x),
                    Orientation::AntiTranspose => (h - 1 - y, w - 1 - x), Orientation::Rotate90Ccw => (y, w - 1 - x),
                };
                for c in 0..spp {
                    let a = raw.data[(y * w + x) * spp + c]; let b = adj.data[(ty * ow + tx) * spp + c];
                    if !(a == b || (a.is_nan() && b.is_nan())) { bad += 1; if first.is_none() { first = Some((x, y, c, a, b)); } }
                }
            } }
            for k in 0..raw.ec.len() {
                for y in 0..h { for x in 0..w {
                    let (tx, ty) = match o {
                        Orientation::Identity => (x, y), Orientation::FlipHorizontal => (w - 1 - x, y), Orientation::Rotate180 => (w - 1 - x, h - 1 - y),
                        Orientation::FlipVertical => (x, h - 1 - y), Orientation::Transpose => (y, x), Orientation::Rotate90Cw => (h - 1 - y, x),
                        Orientation::AntiTranspose => (h - 1 - y, w - 1 - x), Orientation::Rotate90Ccw => (y, w - 1 - x),
                    };
                    let a = raw.ec[k][y * w + x]; let b = adj.ec[k][ty * ow + tx];
                    if !(a == b || (a.is_nan() && b.is_nan())) { bad += 1; if first.is_none() { first = Some((x, y, 99 + k, a, b)); } }
                } }
            }
            if bad > 0 { println!("{path} {o:?} {df:?} {ct:?}: {bad} mismatches, first {first:?}"); }
        }
    }
}
pub fn misalign(path: &str) {
    let bytes = std::fs::read(path).unwrap();
    for df in [JxlDataFormat::U16 { endianness: Endianness::native(), bit_depth: 16 }, JxlDataFormat::F32 { endianness: Endianness::native() }, JxlDataFormat::F16 { endianness: Endianness::native() }] {
        for off in [0usize, 1, 2, 3] {
            let r = std::panic::catch_unwind(|| {
                let mut dec = JxlDecoder::new(JxlDecoderOptions::default());
                let mut input: &[u8] = &bytes;
                let mut mem: Vec<u8> = vec![];
                let mut dims = (0, 0);
                loop {
                    let ev = if mem.is_empty() { dec.process(&mut input, None, None).unwrap() } else {
                        let b = &mut mem[off..off + dims.0 * dims.1];
                        let mut bufs = [JxlOutputBuffer::new(b, dims.1, dims.0)];
                        dec.process(&mut input, Some(&mut bufs), None).unwrap()
                    };
                    match ev {
                        Event::BasicInfo => {
                            let i = dec.basic_info().unwrap().clone();
                            let n = i.extra_channels.len();
                            dec.set_pixel_format(JxlPixelFormat { color_type: JxlColorType::Rgb, color_data_format: Some(df), extra_channel_format: vec![None; n] }).unwrap();
                            dims = (i.size.0 * 3 * df.bytes_per_sample(), i.size.1);
                            mem = vec![0u8; dims.0 * dims.1 + 8];
                        }
                        Event::FrameComplete { .. } => break,
                        _ => {}
                    }
                }
                mem[off..off + dims.0 * dims.1].to_vec()
            });
            match r { Ok(n) => { println!("{df:?} off={off}: completed {:?}", n.iter().map(|b| *b as u32).sum::<u32>()); if off > 0 { println!("   hash {:x}", n.iter().fold(0u64, |a, b| a.wrapping_mul(131).wrapping_add(*b as u64))); } else { println!("   hash {:x}", n.iter().fold(0u64, |a, b| a.wrapping_mul(131).wrapping_add(*b as u64))); } }, Err(_) => println!("{df:?} off={off}: panicked") }
        }
    }
}
pub fn misuse(path: &str) {
    let bytes = std::fs::read(path).unwrap();
    let f = JxlDataFormat::U8 { bit_depth: 8 };
    for ct in [JxlColorType::Rgb, JxlColorType::Rgba, JxlColorType::Grayscale, JxlColorType::GrayscaleAlpha, JxlColorType::Cmyk, JxlColorType::Bgra] {
        for ecn in [0usize, 1, 2, 3] {
            for ecf in [Some(f), None] {
                for colorf in [Some(f), None] {
                    let r = std::panic::catch_unwind(|| {
                        let mut dec = JxlDecoder::new(JxlDecoderOptions::default());
                        let mut input: &[u8] = &bytes;
                        let mut mem: Vec<Vec<u8>> = vec![];
                        let mut dims: Vec<(usize, usize)> = vec![];
                        loop {
                            let ev = if mem.is_empty() { dec.process(&mut input, None, None).map_err(|e| format!("{e:?}"))? } else {
                                let mut bufs: Vec<JxlOutputBuffer> = mem.iter_mut().zip(dims.iter()).map(|(m, d)| JxlOutputBuffer::new(m, d.1, d.0)).collect();
                                dec.process(&mut input, Some(&mut bufs), None).map_err(|e| format!("{e:?}"))?
                            };
                            match ev {
                                Event::BasicInfo => {
                                    let i = dec.basic_info().unwrap().clone();
                                    let pf = JxlPixelFormat { color_type: ct, color_data_format: colorf, extra_channel_format: vec![ecf; ecn] };
                                    dec.set_pixel_format(pf.clone()).map_err(|e| format!("set: {e:?}"))?;
                                    if let Some(f) = colorf { mem.push(vec![0; i.size.0 * ct.samples_per_pixel() * f.bytes_per_sample() * i.size.1]); dims.push((i.size.0 * ct.samples_per_pixel() * f.bytes_per_sample(), i.size.1)); }
                                    for _ in 0..ecn { if let Some(f) = ecf { mem.push(vec![0; i.size.0 * f.bytes_per_sample() * i.size.1]); dims.push((i.size.0 * f.bytes_per_sample(), i.size.1)); } }
                                }
                                Event::FrameComplete { .. } => return Ok::<(), String>(()),
                                Event::NeedMoreInput { .. } if input.is_empty() => return Err("truncated".into()),
                                _ => {}
                            }
                        }
                    });
                    match r { Ok(Ok(())) => {}, Ok(Err(e)) => println!("{ct:?} ecn={ecn} ecf={ecf:?} color={colorf:?}: Err {}", &e[..e.len().min(90)]), Err(_) => println!("{ct:?} ecn={ecn} ecf={ecf:?} color={colorf:?}: PANIC") }
                }
            }
        }
    }
}
pub fn misuse2(path: &str) {
    let bytes = std::fs::read(path).unwrap();
    let r = std::panic::catch_unwind(|| {
        let mut dec = JxlDecoder::new(JxlDecoderOptions::default());
        let mut input: &[u8] = &bytes;
        loop { if dec.process(&mut input, None, None).unwrap() == Event::Complete { break; } }
        for i in 0..3 { let r = dec.process(&mut input, None, None); println!("after complete #{i}: {:?}", r.map_err(|e| format!("{e:?}"))); }
        let mut empty: [JxlOutputBuffer; 0] = [];
        println!("flush after complete: {:?}", dec.flush_pixels(&mut empty, None).map_err(|e| format!("{e:?}")));
    });
    println!("after-complete: {}", if r.is_ok() { "ok" } else { "PANIC" });
    // truncated stream, then keep calling
    let r = std::panic::catch_unwind(|| {
        let cut = &bytes[..bytes.len() / 2];
        let mut dec = JxlDecoder::new(JxlDecoderOptions::default());
        let mut input: &[u8] = cut;
        for i in 0..6 { let r = dec.process(&mut input, None, None); println!("truncated #{i}: {:?}", r.map_err(|e| format!("{e:?}"))); }
        let mut input2: &[u8] = &bytes[bytes.len() / 2..];
        for i in 0..4 { let r = dec.process(&mut input2, None, None); println!("rest #{i}: {:?}", r.map_err(|e| format!("{e:?}"))); }
    });
    println!("truncated: {}", if r.is_ok() { "ok" } else { "PANIC" });
    // garbage then reuse
    let r = std::panic::catch_unwind(|| {
        let mut dec = JxlDecoder::new(JxlDecoderOptions::default());
        let mut bad: &[u8] = &[0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc];
        for i in 0..3 { let r = dec.process(&mut bad, None, None); println!("garbage #{i}: {:?}", r.map_err(|e| format!("{e:?}"))); }
        let mut input: &[u8] = &bytes;
        for i in 0..3 { let r = dec.process(&mut input, None, None); println!("valid-after-garbage #{i}: {:?}", r.map_err(|e| format!("{e:?}"))); }
    });
    println!("garbage: {}", if r.is_ok() { "ok" } else { "PANIC" });
}
pub fn misuse3(path: &str) {
    let bytes = std::fs::read(path).unwrap();
    for variant in 0..6 {
        let r = std::panic::catch_unwind(|| {
            let cut = &bytes[..bytes.len() * 95 / 100];
            let mut dec = JxlDecoder::new(JxlDecoderOptions::default());
            let mut input: &[u8] = cut;
            loop {
                match dec.process(&mut input, None, None).unwrap() { Event::FrameHeader => break, Event::NeedMoreInput { .. } if input.is_empty() => panic!("trunc"), _ => {} }
            }
            let info = dec.basic_info().unwrap().clone();
            let (w, h) = info.size;
            let n = 1 + info.extra_channels.len();
            let mut mems: Vec<Vec<u8>> = match variant {
                0 => vec![],                                            // no buffers
                1 => (0..n + 1).map(|_| vec![0u8; w * 3 * 4 * h]).collect(), // too many
                2 => (0..n).map(|_| vec![0u8; w * 3 * 4 * h / 2 + 4]).collect(),      // wrong size (half)
                3 => (0..n).map(|i| vec![0u8; if i == 0 { w * 3 * 4 * h } else { w * 4 * h }]).collect(), // correct
                4 => (0..n).map(|_| vec![0u8; 4]).collect(),            // tiny
                _ => (0..n).map(|i| vec![0u8; if i == 0 { w * 3 * 4 * h } else { w * 4 * h * 2 }]).collect(), // ec wrong
            };
            let mut bufs: Vec<JxlOutputBuffer> = mems.iter_mut().enumerate().map(|(i, m)| {
                let bpr = match variant { 0 | 1 | 3 => if i == 0 { w * 3 * 4 } else { w * 4 }, 2 => (w * 3 * 4) / 2, 4 => 4, _ => if i == 0 { w * 3 * 4 } else { w * 8 } };
                let rows = m.len() / bpr;
                JxlOutputBuffer::new(m, rows.max(1), bpr)
            }).collect();
            let r = dec.flush_pixels(&mut bufs, None);
            format!("{:?}", r.map_err(|e| format!("{e:?}")))
        });
        match r { Ok(s) => println!("variant {variant}: {s}"), Err(_) => println!("variant {variant}: PANIC") }
    }
}
