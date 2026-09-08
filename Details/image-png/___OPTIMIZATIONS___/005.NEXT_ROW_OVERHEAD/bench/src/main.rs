//! Pure measurement candidate: how much overhead does the `next_row`/`next_interlaced_row` path
//! add over `read_row` (which writes directly into a caller-provided buffer, no `mem::take`/
//! scratch-buffer dance, no extra `Option`/struct wrapping)? IDEAS.md's backlog framed this as
//! "the measurable question" left after noting `read_row` already exists as the zero-copy
//! alternative (issue #417). No patch - this only measures the existing public API.

use std::hint::black_box;
use std::io::Cursor;
use std::time::{Duration, Instant};
use std::{env, fs};

use png::{Decoder, Reader};

fn create_reader(data: &[u8]) -> Reader<Cursor<&[u8]>> {
    Decoder::new(Cursor::new(data)).read_info().unwrap()
}

fn decode_via_next_frame(data: &[u8]) -> Vec<u8> {
    let mut reader = create_reader(data);
    let mut image = vec![0u8; reader.output_buffer_size().unwrap()];
    reader.next_frame(&mut image).unwrap();
    image
}

fn decode_via_read_row(data: &[u8], bytes_per_row: usize, total_len: usize) -> Vec<u8> {
    let mut reader = create_reader(data);
    let mut image = vec![0u8; total_len];
    for row in image.chunks_exact_mut(bytes_per_row) {
        reader.read_row(row).unwrap().unwrap();
    }
    image
}

fn decode_via_next_row(data: &[u8], total_len: usize) -> Vec<u8> {
    let mut reader = create_reader(data);
    let mut image = Vec::with_capacity(total_len);
    while let Some(row) = reader.next_row().unwrap() {
        image.extend_from_slice(row.data());
    }
    image
}

fn time_it<F: Fn() -> Vec<u8>>(f: F) -> Duration {
    let start = Instant::now();
    let out = f();
    black_box(&out);
    start.elapsed()
}

fn p50(mut d: Vec<Duration>) -> f64 {
    d.sort();
    d[d.len() / 2].as_secs_f64() * 1e3
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let iters: usize = args
        .iter()
        .position(|a| a == "--iters")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(200);
    let path = args
        .iter()
        .position(|a| a == "--input")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "../../../tests/benches/kodim02.png".to_string());

    let data = fs::read(&path).unwrap();
    let reader = create_reader(&data);
    let bytes_per_row = reader.output_line_size(reader.info().width).unwrap();
    let total_len = reader.output_buffer_size().unwrap();
    drop(reader);

    // Correctness: all three paths must produce byte-identical pixels.
    let reference = decode_via_next_frame(&data);
    let via_read_row = decode_via_read_row(&data, bytes_per_row, total_len);
    let via_next_row = decode_via_next_row(&data, total_len);
    let verify = if via_read_row == reference && via_next_row == reference {
        "OK"
    } else {
        "FAIL"
    };
    if verify == "FAIL" {
        println!("RESULT file={path} verify=FAIL(outputs differ across next_frame/read_row/next_row)");
        return;
    }

    let warmup = iters / 10;
    let mut t_read_row = Vec::with_capacity(iters);
    let mut t_next_row = Vec::with_capacity(iters);
    let mut t_next_frame = Vec::with_capacity(iters);

    for i in 0..iters {
        // Rotate the order every iteration to cancel drift across all 3 variants.
        let order = i % 3;
        let (a, b, c) = if order == 0 {
            (
                time_it(|| decode_via_read_row(&data, bytes_per_row, total_len)),
                time_it(|| decode_via_next_row(&data, total_len)),
                time_it(|| decode_via_next_frame(&data)),
            )
        } else if order == 1 {
            let b = time_it(|| decode_via_next_row(&data, total_len));
            let c = time_it(|| decode_via_next_frame(&data));
            let a = time_it(|| decode_via_read_row(&data, bytes_per_row, total_len));
            (a, b, c)
        } else {
            let c = time_it(|| decode_via_next_frame(&data));
            let a = time_it(|| decode_via_read_row(&data, bytes_per_row, total_len));
            let b = time_it(|| decode_via_next_row(&data, total_len));
            (a, b, c)
        };
        if i >= warmup {
            t_read_row.push(a);
            t_next_row.push(b);
            t_next_frame.push(c);
        }
    }

    let n = t_read_row.len();
    let p_read_row = p50(t_read_row);
    let p_next_row = p50(t_next_row);
    let p_next_frame = p50(t_next_frame);
    println!(
        "RESULT file={path} n={n} next_frame_p50={p_next_frame:.4}ms read_row_p50={p_read_row:.4}ms next_row_p50={p_next_row:.4}ms next_row_vs_read_row={:+.1}% verify={verify}",
        (p_next_row / p_read_row - 1.0) * 100.0,
    );
}
