// Diff-review repro for candidate 010 (PR #287 cross-check against local bugs #001/#002).
//
// Builds spec-valid lossless JPEG bitstreams (single- and multi-component, interleaved) with a
// restart interval and a chosen predictor selection, computes the ground-truth decoded pixels by
// hand from ITU-T T.81 H.1.2.1/H.1.2.3, and compares against `jpeg_decoder::Decoder`'s real
// public decode() output. Run unmodified against three different trees (see this candidate's
// justfile / RESULT.md): pristine base_commit, base_commit + PR #287's diff only, and
// base_commit + PR #287 + this candidate's own guard fix (patch.diff).
//
// Scenario groups:
//   A, B       - same shape as local candidates 001/002's own repros (single-component restart
//                boundary bugs), kept here as a regression sanity check.
//   C, D       - multi-component interleaved scans with a restart interval that does not evenly
//                divide the image (mid-row restarts), Ra branch (C) and generic branch (D).
//   E, F       - precision/point_transform boundary (point_transform == precision - 1), isolated
//                to a single pixel, no restart involved - tests only the default-prediction guard
//                question, independently of the restart-boundary question. E is the Ra branch
//                (unguarded pre-PR287, guarded and buggy post-PR287, fixed by patch.diff). F is
//                the generic branch (candidate 003's own already-tracked, still-open bug -
//                unaffected by this candidate's patch either way).
//   G, H, I, J - diagnostic scenarios that isolate a separate, NOT fixed by this candidate's
//                patch, anomaly noticed while testing C: with restart_interval=7 (not a divisor
//                of width=6) and ncomp=3, the *last* scan component gets an off-by-one value at
//                each row's x=0 position between restarts, present identically in the pristine
//                tree, the PR #287 tree, and the PR #287 + guard-fix tree alike (so unrelated to
//                the guard question). G/H rule out that it needs restart+multi-component
//                separately; I shows it follows the *array position* (last component index), not
//                particular pixel values; J shows the generic branch (not just Ra) is also
//                affected for this specific restart_interval/width combination. Left uninvestigated
//                further, per this candidate's scope - see RESULT.md's "New suspects noticed".

use std::io::Cursor;

const DC_COUNTS: [u8; 16] = [0, 1, 5, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0];
const DC_VALUES: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

fn huffman_code_for_category(category: u8) -> (u32, u8) {
    let mut code: u32 = 0;
    let mut k = 0usize;
    for (len_idx, &count) in DC_COUNTS.iter().enumerate() {
        let len = (len_idx + 1) as u8;
        for _ in 0..count {
            let sym = DC_VALUES[k];
            if sym == category {
                return (code, len);
            }
            code += 1;
            k += 1;
        }
        code <<= 1;
    }
    panic!("category {} not present in huffman table", category);
}

fn category_for_diff(diff: i32) -> u8 {
    if diff == 0 {
        return 0;
    }
    let mag = diff.unsigned_abs();
    32 - mag.leading_zeros() as u8
}

fn raw_bits_for_diff(diff: i32, category: u8) -> u32 {
    if diff < 0 {
        (diff + ((1i32 << category) - 1)) as u32
    } else {
        diff as u32
    }
}

struct BitWriter {
    buf: Vec<u8>,
    cur: u8,
    nbits: u8,
}

impl BitWriter {
    fn new() -> Self {
        BitWriter { buf: Vec::new(), cur: 0, nbits: 0 }
    }

    fn put_bits(&mut self, value: u32, len: u8) {
        for i in (0..len).rev() {
            let bit = ((value >> i) & 1) as u8;
            self.cur = (self.cur << 1) | bit;
            self.nbits += 1;
            if self.nbits == 8 {
                self.buf.push(self.cur);
                if self.cur == 0xFF {
                    self.buf.push(0x00);
                }
                self.cur = 0;
                self.nbits = 0;
            }
        }
    }

    fn align_before_marker(&mut self) {
        if self.nbits > 0 {
            let pad = 8 - self.nbits;
            let byte = (self.cur << pad) | ((1u8 << pad) - 1);
            self.buf.push(byte);
            self.cur = 0;
            self.nbits = 0;
        }
    }

    fn write_marker(&mut self, marker_byte: u8) {
        self.buf.push(0xFF);
        self.buf.push(marker_byte);
    }

    fn into_bytes(self) -> Vec<u8> {
        self.buf
    }
}

/// Predictor Table H.1 (interior samples only; boundary/reset rules handled by caller).
fn predictor_table(selection: u8, ra: i32, rb: i32, rc: i32) -> i32 {
    match selection {
        1 => ra,
        2 => rb,
        3 => rc,
        4 => ra + rb - rc,
        5 => ra + ((rb - rc) >> 1),
        6 => rb + ((ra - rc) >> 1),
        7 => (ra + rb) / 2,
        _ => panic!("unsupported predictor selection {}", selection),
    }
}

struct Scenario {
    name: &'static str,
    width: usize,
    height: usize,
    ncomp: usize,
    precision: u8,
    point_transform: u8,
    restart_interval: u16,
    predictor_selection: u8,
    /// per-component, row-major ground truth grid (already point-transformed-domain values,
    /// i.e. these are the *final* decoded sample values the spec says should come out).
    ground_truth: Vec<Vec<i32>>,
}

fn default_prediction(precision: u8, point_transform: u8) -> i32 {
    // ITU-T T.81 H.1.2.1: unconditionally 1 << (P - Pt - 1). Valid whenever point_transform <
    // precision, which parse_sos enforces at parse time (independent of predictor selection or
    // predictor branch) - see candidate 003's own finding on the too-strict `> 1 + Pt` guard some
    // code paths apply instead.
    1i32 << (precision - point_transform - 1)
}

fn correct_predictor(
    scenario: &Scenario,
    comp: usize,
    x: usize,
    y: usize,
    is_reset: bool,
) -> i32 {
    let grid = &scenario.ground_truth[comp];
    let w = scenario.width;
    let default = default_prediction(scenario.precision, scenario.point_transform);
    if (x == 0 && y == 0) || is_reset {
        default
    } else if y == 0 {
        grid[x - 1] >> scenario.point_transform
    } else if x == 0 {
        grid[(y - 1) * w + x] >> scenario.point_transform
    } else {
        let ra = grid[y * w + x - 1] >> scenario.point_transform;
        let rb = grid[(y - 1) * w + x] >> scenario.point_transform;
        let rc = grid[(y - 1) * w + x - 1] >> scenario.point_transform;
        predictor_table(scenario.predictor_selection, ra, rb, rc)
    }
}

fn build_jpeg(scenario: &Scenario) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&[0xFF, 0xD8]); // SOI

    // SOF3
    out.extend_from_slice(&[0xFF, 0xC3]);
    let sof_len = 8 + 3 * scenario.ncomp;
    out.extend_from_slice(&(sof_len as u16).to_be_bytes());
    out.push(scenario.precision);
    out.extend_from_slice(&(scenario.height as u16).to_be_bytes());
    out.extend_from_slice(&(scenario.width as u16).to_be_bytes());
    out.push(scenario.ncomp as u8);
    for c in 0..scenario.ncomp {
        out.push((c + 1) as u8); // component id
        out.push(0x11); // h=1,v=1
        out.push(0x00); // quant table index (unused, must be 0)
    }

    // DHT: one shared DC table, index 0
    out.extend_from_slice(&[0xFF, 0xC4]);
    out.extend_from_slice(&[0x00, 0x1F]);
    out.push(0x00);
    out.extend_from_slice(&DC_COUNTS);
    out.extend_from_slice(&DC_VALUES);

    // DRI
    if scenario.restart_interval > 0 {
        out.extend_from_slice(&[0xFF, 0xDD]);
        out.extend_from_slice(&[0x00, 0x04]);
        out.extend_from_slice(&scenario.restart_interval.to_be_bytes());
    }

    // SOS
    out.extend_from_slice(&[0xFF, 0xDA]);
    let sos_len = 6 + 2 * scenario.ncomp;
    out.extend_from_slice(&(sos_len as u16).to_be_bytes());
    out.push(scenario.ncomp as u8);
    for c in 0..scenario.ncomp {
        out.push((c + 1) as u8);
        out.push(0x00); // dc table idx 0, ac unused
    }
    out.push(scenario.predictor_selection); // Ss
    out.push(0x00); // Se
    out.push(scenario.point_transform & 0x0F); // Ah=0 (high nibble), Al=point_transform (low nibble)

    let mut bw = BitWriter::new();
    let mut mcus_left = scenario.restart_interval;
    let mut expected_rst_num: u8 = 0;
    let mut idx = 0usize;
    for y in 0..scenario.height {
        for x in 0..scenario.width {
            if scenario.restart_interval > 0 {
                if mcus_left == 0 {
                    bw.align_before_marker();
                    bw.write_marker(0xD0 + expected_rst_num);
                    expected_rst_num = (expected_rst_num + 1) % 8;
                    mcus_left = scenario.restart_interval;
                }
                mcus_left -= 1;
            }

            let is_reset = scenario.restart_interval > 0
                && idx > 0
                && (idx as u16) % scenario.restart_interval == 0;

            for c in 0..scenario.ncomp {
                let prediction = correct_predictor(scenario, c, x, y, is_reset);
                let sample = scenario.ground_truth[c][y * scenario.width + x] >> scenario.point_transform;
                let diff = sample - prediction;

                let category = category_for_diff(diff);
                let (code, code_len) = huffman_code_for_category(category);
                bw.put_bits(code, code_len);
                if category > 0 {
                    let raw = raw_bits_for_diff(diff, category);
                    bw.put_bits(raw, category);
                }
            }

            idx += 1;
        }
    }
    bw.align_before_marker();
    out.extend_from_slice(&bw.into_bytes());

    out.extend_from_slice(&[0xFF, 0xD9]); // EOI
    out
}

fn run_scenario(scenario: &Scenario) -> bool {
    let jpeg_bytes = build_jpeg(scenario);
    let mut decoder = jpeg_decoder::Decoder::new(Cursor::new(jpeg_bytes));
    let decoded = match decoder.decode() {
        Ok(d) => d,
        Err(e) => {
            println!("[{}] Decode FAILED: {:?}", scenario.name, e);
            return false;
        }
    };
    let info = decoder.info().unwrap();
    let ncomp = scenario.ncomp;

    let mut ok = true;
    let mut mismatches = 0usize;
    for y in 0..scenario.height {
        for x in 0..scenario.width {
            for c in 0..ncomp {
                let expected = scenario.ground_truth[c][y * scenario.width + x];
                let actual = if scenario.precision == 8 {
                    decoded[(y * scenario.width + x) * ncomp + c] as i32
                } else {
                    // native-endian u16 pairs
                    let base = ((y * scenario.width + x) * ncomp + c) * 2;
                    i32::from(u16::from_ne_bytes([decoded[base], decoded[base + 1]]))
                };
                if actual != expected {
                    mismatches += 1;
                    ok = false;
                    if mismatches <= 8 {
                        println!(
                            "[{}] mismatch at (x={},y={},c={}): expected {}, got {}",
                            scenario.name, x, y, c, expected, actual
                        );
                    }
                }
            }
        }
    }
    let total = scenario.width * scenario.height * ncomp;
    println!(
        "[{}] {}x{} ncomp={} precision={} pt={} restart={} predsel={} -> {}/{} mismatched (output {}x{} components={})",
        scenario.name,
        scenario.width,
        scenario.height,
        ncomp,
        scenario.precision,
        scenario.point_transform,
        scenario.restart_interval,
        scenario.predictor_selection,
        mismatches,
        total,
        info.width,
        info.height,
        info.pixel_format as u8,
    );
    ok
}

fn grid_from_fn(width: usize, height: usize, f: impl Fn(usize, usize) -> i32) -> Vec<i32> {
    let mut v = vec![0; width * height];
    for y in 0..height {
        for x in 0..width {
            v[y * width + x] = f(x, y);
        }
    }
    v
}

fn main() {
    let mut all_ok = true;

    // Scenario A: single-component, predictor 4 (generic branch), restart_interval=5,
    // 4x4 - same shape as candidate 001's own repro (regression sanity check).
    let gt_a = grid_from_fn(4, 4, |x, y| (10 + x as i32 * 3 + y as i32 * 20) % 250);
    all_ok &= run_scenario(&Scenario {
        name: "A_single_generic_restart5",
        width: 4,
        height: 4,
        ncomp: 1,
        precision: 8,
        point_transform: 0,
        restart_interval: 5,
        predictor_selection: 4,
        ground_truth: vec![gt_a],
    });

    // Scenario B: single-component, predictor 1 (Ra branch), restart_interval=5, 4x4 - candidate
    // 002 shape.
    let gt_b = grid_from_fn(4, 4, |x, y| (15 + x as i32 * 7 + y as i32 * 11) % 240);
    all_ok &= run_scenario(&Scenario {
        name: "B_single_ra_restart5",
        width: 4,
        height: 4,
        ncomp: 1,
        precision: 8,
        point_transform: 0,
        restart_interval: 5,
        predictor_selection: 1,
        ground_truth: vec![gt_b],
    });

    // Scenario C: 3-component interleaved, predictor 1 (Ra), restart_interval=7 (does not evenly
    // divide width*height=6x5=30), mid-row restart boundaries.
    let mut gt_c = Vec::new();
    for c in 0..3 {
        gt_c.push(grid_from_fn(6, 5, |x, y| {
            (20 + c as i32 * 30 + x as i32 * 5 + y as i32 * 13) % 230
        }));
    }
    all_ok &= run_scenario(&Scenario {
        name: "C_multicomp_ra_restart7_midrow",
        width: 6,
        height: 5,
        ncomp: 3,
        precision: 8,
        point_transform: 0,
        restart_interval: 7,
        predictor_selection: 1,
        ground_truth: gt_c,
    });

    // Scenario D: 3-component interleaved, predictor 4 (generic branch), restart_interval=4,
    // 5x5, mid-row restarts.
    let mut gt_d = Vec::new();
    for c in 0..3 {
        gt_d.push(grid_from_fn(5, 5, |x, y| {
            (5 + c as i32 * 17 + x as i32 * 9 + y as i32 * 6) % 245
        }));
    }
    all_ok &= run_scenario(&Scenario {
        name: "D_multicomp_generic_restart4_midrow",
        width: 5,
        height: 5,
        ncomp: 3,
        precision: 8,
        point_transform: 0,
        restart_interval: 4,
        predictor_selection: 4,
        ground_truth: gt_d,
    });

    // Scenario E: precision/point_transform boundary (point_transform == precision - 1),
    // single-component, predictor 1 (Ra), NO restart interval, 1x1 image so only pixel (0,0)
    // exists - isolates *only* the default-prediction guard question (a separate, unrelated
    // effect exists for point_transform != 0 at pixel indices beyond (0,0); keeping this
    // scenario to a single pixel avoids that confound entirely).
    // precision=4, point_transform=3 -> default should be 1 << (4-3-1) = 1 << 0 = 1 -> output 8.
    let gt_e = grid_from_fn(1, 1, |_x, _y| 1i32 << 3);
    all_ok &= run_scenario(&Scenario {
        name: "E_precision_pt_boundary_ra_no_restart",
        width: 1,
        height: 1,
        ncomp: 1,
        precision: 4,
        point_transform: 3,
        restart_interval: 0,
        predictor_selection: 1,
        ground_truth: vec![gt_e],
    });

    // Scenario F: same boundary, but generic branch (predictor 4) - candidate 003's own case
    // for reference/regression cross-check, also isolated to a single pixel.
    let gt_f = grid_from_fn(1, 1, |_x, _y| 1i32 << 3);
    all_ok &= run_scenario(&Scenario {
        name: "F_precision_pt_boundary_generic_no_restart",
        width: 1,
        height: 1,
        ncomp: 1,
        precision: 4,
        point_transform: 3,
        restart_interval: 0,
        predictor_selection: 4,
        ground_truth: vec![gt_f],
    });

    // Scenario I: same as C, but with component order reversed (c=2's own value formula now at
    // array index 0 instead of 2) - checks whether the bug follows the *last component index*
    // specifically, or a particular value range.
    let mut gt_i = Vec::new();
    for c in [2i32, 1, 0] {
        gt_i.push(grid_from_fn(6, 5, |x, y| {
            (20 + c * 30 + x as i32 * 5 + y as i32 * 13) % 230
        }));
    }
    all_ok &= run_scenario(&Scenario {
        name: "I_multicomp_ra_restart7_reversed",
        width: 6,
        height: 5,
        ncomp: 3,
        precision: 8,
        point_transform: 0,
        restart_interval: 7,
        predictor_selection: 1,
        ground_truth: gt_i,
    });

    // Scenario J: identical grid/restart to scenario C, but predictor_selection=4 (generic
    // branch) instead of 1 (Ra) - apples-to-apples check of whether this is Ra-branch-specific.
    let mut gt_j = Vec::new();
    for c in 0..3 {
        gt_j.push(grid_from_fn(6, 5, |x, y| {
            (20 + c as i32 * 30 + x as i32 * 5 + y as i32 * 13) % 230
        }));
    }
    all_ok &= run_scenario(&Scenario {
        name: "J_multicomp_generic_restart7_midrow_same_grid_as_C",
        width: 6,
        height: 5,
        ncomp: 3,
        precision: 8,
        point_transform: 0,
        restart_interval: 7,
        predictor_selection: 4,
        ground_truth: gt_j,
    });

    // Scenario H: same as C but restart_interval=0 (no restarts at all), to check whether the
    // c=2 crosstalk mismatch is restart-related or a pre-existing general multi-component bug.
    let mut gt_h = Vec::new();
    for c in 0..3 {
        gt_h.push(grid_from_fn(6, 5, |x, y| {
            (20 + c as i32 * 30 + x as i32 * 5 + y as i32 * 13) % 230
        }));
    }
    all_ok &= run_scenario(&Scenario {
        name: "H_multicomp_ra_norestart",
        width: 6,
        height: 5,
        ncomp: 3,
        precision: 8,
        point_transform: 0,
        restart_interval: 0,
        predictor_selection: 1,
        ground_truth: gt_h,
    });

    // Scenario G: isolate component c=2's own value formula from scenario C as a *single*
    // component scan, same width/height/restart, to check whether the off-by-one is a
    // multi-component crosstalk bug or purely a value-specific encoding edge case.
    let gt_g = grid_from_fn(6, 5, |x, y| (80 + x as i32 * 5 + y as i32 * 13) % 230);
    all_ok &= run_scenario(&Scenario {
        name: "G_single_comp2values_ra_restart7_midrow",
        width: 6,
        height: 5,
        ncomp: 1,
        precision: 8,
        point_transform: 0,
        restart_interval: 7,
        predictor_selection: 1,
        ground_truth: vec![gt_g],
    });

    println!("\nOVERALL: {}", if all_ok { "ALL SCENARIOS MATCHED" } else { "MISMATCHES FOUND" });
    std::process::exit(if all_ok { 0 } else { 1 });
}
