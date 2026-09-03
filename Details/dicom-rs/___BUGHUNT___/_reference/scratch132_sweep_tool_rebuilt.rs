//! Sweep: does adding MORE consecutive private UN undefined-length elements ever change the
//! parse outcome versus a single one? (rebuilt from partial recovery for candidate slot 132,
//! `Enet4/dicom-rs#169`)
use dicom_object::InMemDicomObject;
use dicom_transfer_syntax_registry::entries::{
    EXPLICIT_VR_LITTLE_ENDIAN, IMPLICIT_VR_LITTLE_ENDIAN,
};

const UNDEF: [u8; 4] = [0xff, 0xff, 0xff, 0xff];

fn tag(g: u16, e: u16) -> Vec<u8> {
    let mut v = g.to_le_bytes().to_vec();
    v.extend_from_slice(&e.to_le_bytes());
    v
}
fn u32le(x: u32) -> Vec<u8> {
    x.to_le_bytes().to_vec()
}

/// shape 0: item with defined length
/// shape 1: item with undefined length + item delimiter
/// shape 2: no item at all
/// shape 3: two defined-length items
fn priv_un(explicit: bool, g: u16, e: u16, payload: &[u8], shape: u8) -> Vec<u8> {
    let mut v = tag(g, e);
    if explicit {
        v.extend_from_slice(b"UN");
        v.extend_from_slice(&[0, 0]);
    }
    v.extend_from_slice(&UNDEF);
    match shape {
        0 => {
            v.extend(tag(0xfffe, 0xe000));
            v.extend(u32le(payload.len() as u32));
            v.extend_from_slice(payload);
        }
        1 => {
            v.extend(tag(0xfffe, 0xe000));
            v.extend_from_slice(&UNDEF);
            v.extend_from_slice(payload);
            v.extend(tag(0xfffe, 0xe00d));
            v.extend(u32le(0));
        }
        2 => v.extend_from_slice(payload),
        _ => {
            let half = payload.len() / 2;
            for chunk in [&payload[..half], &payload[half..]] {
                v.extend(tag(0xfffe, 0xe000));
                v.extend(u32le(chunk.len() as u32));
                v.extend_from_slice(chunk);
            }
        }
    }
    v.extend(tag(0xfffe, 0xe0dd));
    v.extend(u32le(0));
    v
}

fn repeat_elems(explicit: bool, n: usize, payload: &[u8], shape: u8, creator: bool) -> Vec<u8> {
    let mut v = Vec::new();
    if creator {
        // realistic Philips private creator element preceding the private block
        v.extend(tag(0x2005, 0x0010));
        if explicit {
            v.extend_from_slice(b"LO");
            v.extend_from_slice(&22u16.to_le_bytes());
        } else {
            v.extend(u32le(22));
        }
        v.extend_from_slice(b"Philips Imaging DD 001");
    }
    for i in 0..n {
        v.extend(priv_un(explicit, 0x2005, 0x1080 + i as u16, payload, shape));
    }
    v
}

fn try_parse(explicit: bool, data: &[u8]) -> String {
    let ts = if explicit {
        EXPLICIT_VR_LITTLE_ENDIAN.erased()
    } else {
        IMPLICIT_VR_LITTLE_ENDIAN.erased()
    };
    match InMemDicomObject::read_dataset_with_ts(data, &ts) {
        Ok(obj) => format!("Ok({} elements)", obj.into_iter().count()),
        Err(e) => {
            let mut chain = format!("Err({e})");
            let mut src: &dyn std::error::Error = &e;
            while let Some(s) = std::error::Error::source(src) {
                chain.push_str(" <- ");
                chain.push_str(&s.to_string());
                src = s;
            }
            chain
        }
    }
}

fn run_one(explicit: bool, n: usize, shape: u8, creator: bool) {
    let payload = vec![0xAB; 16];
    let data = repeat_elems(explicit, n, &payload, shape, creator);
    let label = format!(
        "explicit={explicit} n={n} shape={shape} creator={creator} bytes={}",
        data.len()
    );
    let outcome = std::panic::catch_unwind(|| try_parse(explicit, &data));
    match outcome {
        Ok(result) => println!("{label} -> {result}"),
        Err(_) => println!("{label} -> PANIC (caught)"),
    }
}

fn main() {
    let arg_n: Option<usize> = std::env::args().nth(1).and_then(|s| s.parse().ok());

    if let Some(n) = arg_n {
        // single targeted run, e.g. to reproduce a specific n found interesting by the sweep
        for shape in 0..4u8 {
            for &explicit in &[true, false] {
                run_one(explicit, n, shape, true);
            }
        }
        return;
    }

    for &n in &[1usize, 2, 5, 20, 100, 500, 1000, 5000, 20000] {
        for shape in 0..4u8 {
            for &explicit in &[true, false] {
                for &creator in &[true, false] {
                    run_one(explicit, n, shape, creator);
                }
            }
        }
    }
}

#[cfg(test)]
mod control {
    use super::*;

    #[test]
    fn control_plain_element() {
        // (0008,0060) CS "OT" - a completely ordinary element, no private/UN involved
        let mut v = tag(0x0008, 0x0060);
        v.extend_from_slice(b"CS");
        v.extend_from_slice(&2u16.to_le_bytes());
        v.extend_from_slice(b"OT");
        println!("control -> {}", try_parse(true, &v));
    }
}

#[cfg(test)]
mod stress {
    use super::*;

    #[test]
    fn stress_large_n() {
        for &n in &[50000usize, 100000, 200000] {
            for shape in 0..4u8 {
                let payload = vec![0xABu8; 16];
                let data = repeat_elems(true, n, &payload, shape, true);
                println!("n={n} shape={shape} bytes={} -> starting", data.len());
                let r = try_parse(true, &data);
                println!("n={n} shape={shape} -> {r}");
            }
        }
    }
}

#[cfg(test)]
mod stress2 {
    use super::*;

    #[test]
    fn stress_zero_payload_large_n() {
        for &n in &[1000usize, 20000, 100000, 500000] {
            for shape in 0..2u8 {
                let payload = vec![0u8; 16];
                let data = repeat_elems(true, n, &payload, shape, true);
                println!("n={n} shape={shape} bytes={} -> starting", data.len());
                let r = try_parse(true, &data);
                println!("n={n} shape={shape} -> {} chars of result", r.len());
            }
        }
    }
}

#[cfg(test)]
mod stress3 {
    use super::*;

    #[test]
    fn find_threshold() {
        for &n in &[5000usize] {
            let payload = vec![0u8; 16];
            let data = repeat_elems(true, n, &payload, 1, true);
            println!("n={n} bytes={} -> starting", data.len());
            std::io::Write::flush(&mut std::io::stdout()).ok();
            let r = try_parse(true, &data);
            println!("n={n} -> {} chars", r.len());
        }
    }
}

#[cfg(test)]
mod inspect {
    use super::*;

    #[test]
    fn inspect_small() {
        let payload = vec![0u8; 16];
        let data = repeat_elems(true, 3, &payload, 1, true);
        let ts = EXPLICIT_VR_LITTLE_ENDIAN.erased();
        match InMemDicomObject::read_dataset_with_ts(data.as_slice(), &ts) {
            Ok(obj) => {
                println!("top-level entries: {}", obj.clone().into_iter().count());
                for e in &obj {
                    println!("{:?}", e);
                }
            }
            Err(e) => println!("Err: {e}"),
        }
    }
}

#[cfg(test)]
mod inspect2 {
    use super::*;

    #[test]
    fn inspect_various() {
        for &n in &[3usize, 10, 50, 100, 500, 1000, 1500] {
            let payload = vec![0u8; 16];
            let data = repeat_elems(true, n, &payload, 1, true);
            let r = try_parse(true, &data);
            println!("n={n} -> {r}");
        }
    }
}

#[cfg(test)]
mod real_nesting {
    use super::*;

    fn nested_sq(depth: usize) -> Vec<u8> {
        // (0040,0555) SQ, Acquisition Context Sequence - a real dictionary SQ tag,
        // explicit VR, undefined length, containing one item, containing one more
        // such element, recursively, `depth` levels deep, closed properly at the end.
        let mut v = Vec::new();
        for _ in 0..depth {
            v.extend(tag(0x0040, 0x0555));
            v.extend_from_slice(b"SQ");
            v.extend_from_slice(&[0, 0]);
            v.extend_from_slice(&UNDEF);
            v.extend(tag(0xfffe, 0xe000));
            v.extend_from_slice(&UNDEF);
        }
        // innermost item is empty, then close every item + sequence delimiter pair
        for _ in 0..depth {
            v.extend(tag(0xfffe, 0xe00d));
            v.extend(u32le(0));
            v.extend(tag(0xfffe, 0xe0dd));
            v.extend(u32le(0));
        }
        v
    }

    #[test]
    fn real_nested_sq_overflow() {
        for &depth in &[100usize, 1000, 5000, 20000] {
            let data = nested_sq(depth);
            println!("depth={depth} bytes={} -> starting", data.len());
            std::io::Write::flush(&mut std::io::stdout()).ok();
            let r = try_parse(true, &data);
            println!("depth={depth} -> {r}");
        }
    }
}
