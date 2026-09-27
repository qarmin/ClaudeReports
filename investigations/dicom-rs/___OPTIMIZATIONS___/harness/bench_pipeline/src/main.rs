use std::io::Cursor;
use std::path::PathBuf;
use std::time::Instant;

// Build with `--features diag-mimalloc` to swap the global allocator to mimalloc instead of the
// system default - a diagnostic tool, not the harness's normal configuration, used to
// distinguish "real regression" from "regression specific to this box's default allocator's
// behavior on this access pattern" (see candidates 005/016's RESULT.md).
#[cfg(feature = "diag-mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

// Build with `--features diag-jemalloc` for the same diagnostic purpose as diag-mimalloc above,
// but with jemalloc instead - candidate 041's follow-up on the still-open "root-cause the exact
// glibc mechanism" IDEAS.md item, which only ever tried mimalloc. Mutually exclusive with both
// diag-mimalloc and diag-alloccount (all three define #[global_allocator]).
#[cfg(feature = "diag-jemalloc")]
#[global_allocator]
static GLOBAL_JEMALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

// Build with `--features diag-alloccount` to count total alloc/dealloc calls (not just peak
// bytes, which `/proc/self/status`'s VmHWM already covers via the `*-peak-mem` modes) - for
// measuring an allocation-*count*-reducing fix (e.g. candidate 016's VecDeque reuse) whose effect
// a byte-volume metric can't see at all. Mutually exclusive with `diag-mimalloc` (both define
// `#[global_allocator]`); don't build with both features at once.
#[cfg(feature = "diag-alloccount")]
mod alloccount {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicU64, Ordering};

    static ALLOC_COUNT: AtomicU64 = AtomicU64::new(0);
    static DEALLOC_COUNT: AtomicU64 = AtomicU64::new(0);

    pub struct CountingAllocator;

    unsafe impl GlobalAlloc for CountingAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
            unsafe { System.alloc(layout) }
        }
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            DEALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
            unsafe { System.dealloc(ptr, layout) }
        }
    }

    pub fn reset() {
        ALLOC_COUNT.store(0, Ordering::Relaxed);
        DEALLOC_COUNT.store(0, Ordering::Relaxed);
    }

    pub fn counts() -> (u64, u64) {
        (
            ALLOC_COUNT.load(Ordering::Relaxed),
            DEALLOC_COUNT.load(Ordering::Relaxed),
        )
    }
}

#[cfg(feature = "diag-alloccount")]
#[global_allocator]
static GLOBAL_ALLOCCOUNT: alloccount::CountingAllocator = alloccount::CountingAllocator;

use dicom_core::header::HasLength;
use dicom_core::{DataElement, PrimitiveValue, VR};
use dicom_dictionary_std::tags;
use dicom_encoding::text::TextCodec;
use dicom_object::{from_reader, InMemDicomObject};
use dicom_pixeldata::PixelDecoder;

fn resolve_path(s: &str) -> PathBuf {
    if let Some(rest) = s.strip_prefix("pydicom/") {
        dicom_test_files::path(&format!("pydicom/{rest}"))
            .expect("dicom-test-files fixture should exist")
    } else {
        let literal = PathBuf::from(s);
        if literal.exists() {
            literal
        } else {
            dicom_test_files::path(s).expect("dicom-test-files fixture should exist")
        }
    }
}

fn stats(mut samples: Vec<f64>, unit: &str) -> String {
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = samples.len();
    let warmup = (n / 10).max(1).min(n.saturating_sub(1).max(1));
    let used = &samples[warmup.min(n - 1)..];
    let min = used[0];
    let max = used[used.len() - 1];
    let p50 = used[used.len() / 2];
    let p95 = used[((used.len() as f64 * 0.95) as usize).min(used.len() - 1)];
    let mean: f64 = used.iter().sum::<f64>() / used.len() as f64;
    format!(
        "n={} min={:.3}{u} p50={:.3}{u} mean={:.3}{u} p95={:.3}{u} max={:.3}{u}",
        used.len(),
        min,
        p50,
        mean,
        p95,
        max,
        u = unit
    )
}

fn fnv1a(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in data {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// Candidate 049: real multi-byte sample text for each of the remaining
/// generic-encoding-crate-backed codecs (chosen to actually exercise their multi-byte lead-byte
/// paths, not just ASCII passthrough).
fn multibyte_sample_text(code: &str) -> String {
    match code {
        "ISO_IR 13" => "患者の体温は正常です。心拍数も安定しています。".to_string(),
        "ISO_IR 87" => "患者の体温は正常です。心拍数も安定しています。".to_string(),
        "ISO_IR 149" => "환자의 체온이 정상입니다. 심박수도 안정적입니다.".to_string(),
        "GB18030" | "GBK" => "患者体温正常。心率稳定。血压正常范围内。".to_string(),
        other => panic!("no multibyte sample text defined for charset code {other}"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "usage: bench <mode> [args...]\nmodes: parse <file> <iters> | write-only <file> <iters> | roundtrip <file> <iters> | dump <file> | info <file>"
        );
        std::process::exit(1);
    }
    let mode = args[1].as_str();

    match mode {
        #[cfg(feature = "pdatawriter-inprocess")]
        "pdatawriter-inprocess" => {
            // pdatawriter-inprocess <payload-mb> <iters>
            //
            // Isolates the userspace memcpy candidate 020 removes from real socket/kernel I/O
            // noise (see "pdatawriter-throughput"'s RESULT.md follow-up: the loopback-TCP
            // end-to-end measurement was too noisy to show a consistent delta, since socket
            // I/O dominates over one extra in-memory copy at these payload sizes). Writes
            // directly into a `Vec<u8>` sink via `PDataWriter::new_for_bench` - requires
            // `--features pdatawriter-inprocess` (forwards to dicom-ul's `bench-internals`
            // feature, itself benchmark-only instrumentation never part of patch.diff, see
            // candidate 020's RESULT.md).
            use dicom_ul::association::PDataWriter;
            let payload_mb: usize = args[2].parse().unwrap();
            let iters: usize = args[3].parse().unwrap();
            let payload = vec![0xABu8; payload_mb * 1024 * 1024];
            const LARGE_PDU_LEN: u32 = 1024 * 1024;
            let mut samples = Vec::with_capacity(iters);
            let mut out_len = 0usize;
            for _ in 0..iters {
                let mut sink: Vec<u8> = Vec::with_capacity(payload.len() + 4096);
                let t0 = Instant::now();
                {
                    use std::io::Write as _;
                    let mut writer = PDataWriter::new_for_bench(&mut sink, 1, LARGE_PDU_LEN);
                    writer.write_all(&payload).unwrap();
                    writer.finish().unwrap();
                }
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                out_len = sink.len();
                samples.push(dt);
            }
            println!(
                "pdatawriter-inprocess {}MB ({}/iter) out_len={}",
                payload_mb, stats(samples, "us"), out_len
            );
        }
        #[cfg(feature = "async-pdatawriter-inprocess")]
        "async-pdatawriter-inprocess" => {
            // async-pdatawriter-inprocess <payload-mb> <iters>
            //
            // Async counterpart to "pdatawriter-inprocess", isolating candidate 029's
            // AsyncPDataWriter direct-dispatch fix from real socket/kernel I/O noise the same
            // way. Writes directly into a `Vec<u8>` sink via `AsyncPDataWriter::new_for_bench` -
            // requires `--features async-pdatawriter-inprocess,dicom-ul/async,dicom-ul/bench-internals`
            // (the latter two forwarded explicitly on the command line, not baked into this
            // harness's own default features, for the same "stale optional-feature forward"
            // reason `pdatawriter-inprocess` itself documents above - `dicom-ul/bench-internals`
            // only exists once candidate 029's own `bench-shim.diff` is applied). A separate
            // feature from `pdatawriter-inprocess` (not shared) so this mode's own compilation
            // doesn't require candidate 020's sync-side `PDataWriter::new_for_bench` shim to
            // also be applied at the same time.
            use dicom_ul::association::AsyncPDataWriter;
            use tokio::io::AsyncWriteExt as _;
            let payload_mb: usize = args[2].parse().unwrap();
            let iters: usize = args[3].parse().unwrap();
            let payload = vec![0xABu8; payload_mb * 1024 * 1024];
            const LARGE_PDU_LEN: u32 = 1024 * 1024;
            let rt = tokio::runtime::Runtime::new().unwrap();
            let mut samples = Vec::with_capacity(iters);
            let mut out_len = 0usize;
            for _ in 0..iters {
                let mut sink: Vec<u8> = Vec::with_capacity(payload.len() + 4096);
                let t0 = Instant::now();
                rt.block_on(async {
                    let mut writer = AsyncPDataWriter::new_for_bench(&mut sink, 1, LARGE_PDU_LEN);
                    writer.write_all(&payload).await.unwrap();
                    writer.finish().await.unwrap();
                });
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                out_len = sink.len();
                samples.push(dt);
            }
            println!(
                "async-pdatawriter-inprocess {}MB ({}/iter) out_len={}",
                payload_mb, stats(samples, "us"), out_len
            );
        }
        #[cfg(feature = "async-pdatawriter-inprocess")]
        "async-pdatawriter-peak-mem" => {
            // async-pdatawriter-peak-mem <payload-mb>
            //
            // One-shot peak-RSS probe (VmHWM from /proc/self/status) for candidate 029's
            // AsyncPDataWriter direct-dispatch fix - same technique as write-dataset-peak-mem,
            // never taken for this fix (candidate 029 measured wall-clock only, see IDEAS.md).
            // Same feature/shim requirements as async-pdatawriter-inprocess above.
            use dicom_ul::association::AsyncPDataWriter;
            use tokio::io::AsyncWriteExt as _;
            let payload_mb: usize = args[2].parse().unwrap();
            let payload = vec![0xABu8; payload_mb * 1024 * 1024];
            const LARGE_PDU_LEN: u32 = 1024 * 1024;
            let rt = tokio::runtime::Runtime::new().unwrap();
            let mut sink: Vec<u8> = Vec::with_capacity(payload.len() + 4096);
            rt.block_on(async {
                let mut writer = AsyncPDataWriter::new_for_bench(&mut sink, 1, LARGE_PDU_LEN);
                writer.write_all(&payload).await.unwrap();
                writer.finish().await.unwrap();
            });
            let checksum = fnv1a(&sink);
            let status = std::fs::read_to_string("/proc/self/status").unwrap();
            let vmhwm = status
                .lines()
                .find(|l| l.starts_with("VmHWM:"))
                .unwrap_or("VmHWM: unavailable")
                .to_string();
            println!(
                "async-pdatawriter-peak-mem {}MB checksum={:#x} {}",
                payload_mb, checksum, vmhwm.trim()
            );
        }
        "pdatawriter-throughput" => {
            // pdatawriter-throughput <payload-mb> <iters>
            //
            // Real loopback TCP association (client+server over 127.0.0.1) - not a synthetic
            // in-memory sink, since PDataWriter::new is pub(crate) and can't be constructed
            // directly from outside dicom-ul. Times only the client-side send_pdata/write_all/
            // finish sequence (association setup happens once, outside the timed loop). The
            // server thread reads and discards, then reports a checksum for the correctness
            // check below - if candidate 020's dispatch_pdu_borrowed path ever produced wrong
            // bytes on the wire, this would show up as a checksum mismatch or a hang.
            use dicom_ul::association::{ClientAssociationOptions, ServerAssociationOptions};
            use std::io::Read as _;
            use std::net::TcpListener;

            let payload_mb: usize = args[2].parse().unwrap();
            let iters: usize = args[3].parse().unwrap();
            let payload = vec![0xABu8; payload_mb * 1024 * 1024];
            let payload_checksum = fnv1a(&payload);
            // 1 MiB max PDU length - large enough that candidate 020's fresh-PDU-and-big-write
            // branch (dispatch_pdu_borrowed) fires for almost the entire payload, instead of
            // being dominated by many tiny PDU dispatches at DICOM's more common ~16-32KB
            // defaults (which would mostly measure PDU-dispatch/syscall overhead instead of the
            // copy-avoidance this candidate targets).
            const LARGE_PDU_LEN: u32 = 1024 * 1024;

            const ABSTRACT_SYNTAX: &str = "1.2.840.10008.5.1.4.1.1.7"; // Secondary Capture

            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let server_addr = listener.local_addr().unwrap();
            let server = std::thread::spawn(move || {
                let server_options = ServerAssociationOptions::new()
                    .accept_called_ae_title()
                    .ae_title("BENCH_SCP")
                    .max_pdu_length(LARGE_PDU_LEN)
                    .with_abstract_syntax(ABSTRACT_SYNTAX);
                let (stream, _) = listener.accept().unwrap();
                let mut assoc = server_options.establish(stream).unwrap();
                let mut received_checksums = Vec::with_capacity(iters);
                for _ in 0..iters {
                    let mut buf = Vec::new();
                    assoc.receive_pdata().read_to_end(&mut buf).unwrap();
                    received_checksums.push(fnv1a(&buf));
                }
                received_checksums
            });

            let mut client = ClientAssociationOptions::new()
                .calling_ae_title("BENCH_SCU")
                .called_ae_title("BENCH_SCP")
                .max_pdu_length(LARGE_PDU_LEN)
                .with_presentation_context(ABSTRACT_SYNTAX, vec!["1.2.840.10008.1.2.1"])
                .establish(server_addr)
                .unwrap();
            let pc_id = client.presentation_contexts()[0].id;

            let mut samples = Vec::with_capacity(iters);
            for _ in 0..iters {
                let t0 = Instant::now();
                let mut pdata = client.send_pdata(pc_id);
                use std::io::Write as _;
                pdata.write_all(&payload).unwrap();
                pdata.finish().unwrap();
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                samples.push(dt);
            }

            let received_checksums = server.join().unwrap();
            let all_match = received_checksums.iter().all(|&c| c == payload_checksum);

            println!(
                "pdatawriter-throughput {}MB ({}/iter) checksum={:#x} received_match={}",
                payload_mb, stats(samples, "us"), payload_checksum, all_match
            );
        }
        "parse" => {
            let file = resolve_path(&args[2]);
            let iters: usize = args[3].parse().unwrap();
            let bytes = std::fs::read(&file).unwrap();
            let mut samples = Vec::with_capacity(iters);
            let mut checksum: u64 = 0;
            for _ in 0..iters {
                let t0 = Instant::now();
                let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                checksum = obj.length().0 as u64;
                samples.push(dt);
            }
            println!("parse {} ({}/iter) checksum={:#x}", file.display(), stats(samples, "us"), checksum);
        }
        "write-only-peak-mem" => {
            // single obj.write_all() call, then reports this process's peak RSS (VmHWM) - same
            // technique as write-dataset-peak-mem/transcode-to-native-peak-mem, this time for
            // candidates 001/008 (InMemFragment/DataToken::ItemValue: Vec<u8> -> Arc<[u8]>),
            // whose peak-memory impact was measured wall-clock-only originally, never for RSS.
            let file = resolve_path(&args[2]);
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            let mut out = Vec::with_capacity(bytes.len() + 4096);
            obj.write_all(&mut out).expect("write failed");
            let checksum = fnv1a(&out);
            let status = std::fs::read_to_string("/proc/self/status").unwrap();
            let vmhwm = status
                .lines()
                .find(|l| l.starts_with("VmHWM:"))
                .unwrap_or("VmHWM: unavailable")
                .to_string();
            println!("write-only-peak-mem {} checksum={:#x} {}", file.display(), checksum, vmhwm.trim());
        }
        "write-only" => {
            let file = resolve_path(&args[2]);
            let iters: usize = args[3].parse().unwrap();
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            let mut samples = Vec::with_capacity(iters);
            let mut checksum: u64 = 0;
            for _ in 0..iters {
                let mut out = Vec::with_capacity(bytes.len() + 4096);
                let t0 = Instant::now();
                obj.write_all(&mut out).expect("write failed");
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                checksum = fnv1a(&out);
                samples.push(dt);
            }
            println!("write-only {} ({}/iter) checksum={:#x}", file.display(), stats(samples, "us"), checksum);
        }
        "write-dataset-peak-mem" => {
            // single write_dataset_with_ts call, then reports this process's peak RSS
            // (VmHWM from /proc/self/status) - a one-shot memory probe for candidate 019,
            // since /usr/bin/time -v isn't available in this environment.
            use dicom_encoding::TransferSyntaxIndex;
            let file = resolve_path(&args[2]);
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            let ts_uid = obj.meta().transfer_syntax();
            let ts = dicom_transfer_syntax_registry::TransferSyntaxRegistry
                .get(ts_uid)
                .expect("transfer syntax should be registered");
            let mut out = Vec::with_capacity(bytes.len() + 4096);
            obj.write_dataset_with_ts(&mut out, ts).expect("write failed");
            let checksum = fnv1a(&out);
            let status = std::fs::read_to_string("/proc/self/status").unwrap();
            let vmhwm = status
                .lines()
                .find(|l| l.starts_with("VmHWM:"))
                .unwrap_or("VmHWM: unavailable")
                .to_string();
            println!("write-dataset-peak-mem {} checksum={:#x} {}", file.display(), checksum, vmhwm.trim());
        }
        "write-dataset-only" => {
            // like "write-only" but calls InMemDicomObject::write_dataset_with_ts directly
            // (dataset only, no preamble/meta group via write_all) - the actual entry point
            // candidate 019's borrowed write path changed. See RESULT.md for why "write-only"
            // (write_all) does NOT exercise the fix.
            use dicom_encoding::TransferSyntaxIndex;
            let file = resolve_path(&args[2]);
            let iters: usize = args[3].parse().unwrap();
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            let ts_uid = obj.meta().transfer_syntax();
            let ts = dicom_transfer_syntax_registry::TransferSyntaxRegistry
                .get(ts_uid)
                .expect("transfer syntax should be registered");
            let mut samples = Vec::with_capacity(iters);
            let mut checksum: u64 = 0;
            for _ in 0..iters {
                let mut out = Vec::with_capacity(bytes.len() + 4096);
                let t0 = Instant::now();
                obj.write_dataset_with_ts(&mut out, ts).expect("write failed");
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                checksum = fnv1a(&out);
                samples.push(dt);
            }
            println!("write-dataset-only {} ({}/iter) checksum={:#x}", file.display(), stats(samples, "us"), checksum);
        }
        "write-dataset-alloc-count" => {
            // like "write-dataset-only" but counts total alloc+dealloc calls across all
            // iterations instead of timing them - requires `--features diag-alloccount`.
            // Isolates allocation-*count* effects (e.g. candidate 016's VecDeque reuse) that a
            // peak-RSS/byte-volume metric can't see, per IDEAS.md's own open item on this.
            use dicom_encoding::TransferSyntaxIndex;
            let file = resolve_path(&args[2]);
            let iters: usize = args[3].parse().unwrap();
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            let ts_uid = obj.meta().transfer_syntax();
            let ts = dicom_transfer_syntax_registry::TransferSyntaxRegistry
                .get(ts_uid)
                .expect("transfer syntax should be registered");
            #[cfg(feature = "diag-alloccount")]
            {
                alloccount::reset();
                let mut checksum: u64 = 0;
                for _ in 0..iters {
                    let mut out = Vec::with_capacity(bytes.len() + 4096);
                    obj.write_dataset_with_ts(&mut out, ts).expect("write failed");
                    checksum = fnv1a(&out);
                }
                let (allocs, deallocs) = alloccount::counts();
                println!(
                    "write-dataset-alloc-count {} checksum={:#x} allocs={} deallocs={} allocs_per_iter={:.1}",
                    file.display(), checksum, allocs, deallocs, allocs as f64 / iters as f64
                );
            }
            #[cfg(not(feature = "diag-alloccount"))]
            {
                let _ = (ts, iters);
                eprintln!("write-dataset-alloc-count requires --features diag-alloccount");
            }
        }
        "write-only-alloc-count" => {
            // like "write-only" but counts total alloc+dealloc calls across all iterations
            // instead of timing them - requires --features diag-alloccount. Added for candidate
            // 040's follow-up on candidate 001 (InMemFragment Vec<u8> -> Arc<[u8]>): that
            // candidate's own callgrind Ir-count comparison showed a much smaller reduction
            // (~30%) than the measured wall-clock reduction (~86.5%), hypothesized but never
            // confirmed to be allocation-count/size churn Ir-counting alone doesn't capture.
            let file = resolve_path(&args[2]);
            let iters: usize = args[3].parse().unwrap();
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            #[cfg(feature = "diag-alloccount")]
            {
                alloccount::reset();
                let mut checksum: u64 = 0;
                for _ in 0..iters {
                    let mut out = Vec::with_capacity(bytes.len() + 4096);
                    obj.write_all(&mut out).expect("write failed");
                    checksum = fnv1a(&out);
                }
                let (allocs, deallocs) = alloccount::counts();
                println!(
                    "write-only-alloc-count {} checksum={:#x} allocs={} deallocs={} allocs_per_iter={:.1}",
                    file.display(), checksum, allocs, deallocs, allocs as f64 / iters as f64
                );
            }
            #[cfg(not(feature = "diag-alloccount"))]
            {
                let _ = iters;
                eprintln!("write-only-alloc-count requires --features diag-alloccount");
            }
        }
        // ---- comparison-only modes, exercise the throwaway *_ts_cs2 consuming methods added
        // to patch.diff purely to measure the "consuming self" alternative from issue #641's own
        // body against candidate 019's borrowed approach - see RESULT.md's comparison section ----
        #[cfg(feature = "comparison-ts2")]
        "write-dataset-peak-mem-consuming" => {
            // single write_dataset_with_ts2 call (consumes the object), same shape as
            // "write-dataset-peak-mem" so VmHWM is directly comparable.
            use dicom_encoding::TransferSyntaxIndex;
            let file = resolve_path(&args[2]);
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            let ts_uid = obj.meta().transfer_syntax().to_string();
            let ts = dicom_transfer_syntax_registry::TransferSyntaxRegistry
                .get(&ts_uid)
                .expect("transfer syntax should be registered");
            let mut out = Vec::with_capacity(bytes.len() + 4096);
            obj.into_inner()
                .write_dataset_with_ts2(&mut out, ts)
                .expect("write failed");
            let checksum = fnv1a(&out);
            let status = std::fs::read_to_string("/proc/self/status").unwrap();
            let vmhwm = status
                .lines()
                .find(|l| l.starts_with("VmHWM:"))
                .unwrap_or("VmHWM: unavailable")
                .to_string();
            println!("write-dataset-peak-mem-consuming {} checksum={:#x} {}", file.display(), checksum, vmhwm.trim());
        }
        #[cfg(feature = "comparison-ts2")]
        "write-dataset-only-consuming" => {
            // like "write-dataset-only" but calls the consuming write_dataset_with_ts2 - since
            // it consumes the object, a fresh object must be parsed every iteration; only the
            // write call itself (not the re-parse) is timed, so this stays a fair comparison of
            // just the write path against "write-dataset-only"'s reused-object loop.
            use dicom_encoding::TransferSyntaxIndex;
            let file = resolve_path(&args[2]);
            let iters: usize = args[3].parse().unwrap();
            let bytes = std::fs::read(&file).unwrap();
            let mut samples = Vec::with_capacity(iters);
            let mut checksum: u64 = 0;
            for _ in 0..iters {
                let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
                let ts_uid = obj.meta().transfer_syntax().to_string();
                let ts = dicom_transfer_syntax_registry::TransferSyntaxRegistry
                    .get(&ts_uid)
                    .expect("transfer syntax should be registered");
                let mut out = Vec::with_capacity(bytes.len() + 4096);
                let inner = obj.into_inner();
                let t0 = Instant::now();
                inner.write_dataset_with_ts2(&mut out, ts).expect("write failed");
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                checksum = fnv1a(&out);
                samples.push(dt);
            }
            println!("write-dataset-only-consuming {} ({}/iter) checksum={:#x}", file.display(), stats(samples, "us"), checksum);
        }
        #[cfg(feature = "comparison-ts2")]
        "dump-dataset-consuming" => {
            // like "dump-dataset" but via write_dataset_with_ts2, for the bit-exactness check
            let file = resolve_path(&args[2]);
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            let ts_uid = obj.meta().transfer_syntax().to_string();
            use dicom_encoding::TransferSyntaxIndex;
            let ts = dicom_transfer_syntax_registry::TransferSyntaxRegistry
                .get(&ts_uid)
                .expect("transfer syntax should be registered");
            let mut out = Vec::new();
            obj.into_inner()
                .write_dataset_with_ts2(&mut out, ts)
                .expect("write_dataset_with_ts2 failed");
            use std::io::Write as _;
            std::io::stdout().write_all(&out).unwrap();
        }
        "roundtrip" => {
            let file = resolve_path(&args[2]);
            let iters: usize = args[3].parse().unwrap();
            let bytes = std::fs::read(&file).unwrap();
            let mut samples = Vec::with_capacity(iters);
            let mut checksum: u64 = 0;
            for _ in 0..iters {
                let t0 = Instant::now();
                let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
                let mut out = Vec::with_capacity(bytes.len() + 4096);
                obj.write_all(&mut out).expect("write failed");
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                checksum = fnv1a(&out);
                samples.push(dt);
            }
            println!("roundtrip {} ({}/iter) checksum={:#x}", file.display(), stats(samples, "us"), checksum);
        }
        "dump" => {
            let file = resolve_path(&args[2]);
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            let mut out = Vec::new();
            obj.write_all(&mut out).expect("write failed");
            use std::io::Write as _;
            std::io::stdout().write_all(&out).unwrap();
        }
        "dump-dataset" => {
            // exercises InMemDicomObject::write_dataset_with_ts_cs directly (dataset only, no
            // preamble/meta group) - unlike "dump" (which goes through
            // FileDicomObject::write_all, deliberately NOT touched by candidate 019's borrowed
            // write path, see its RESULT.md) this is the actual entry point candidate 019 changed
            let file = resolve_path(&args[2]);
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            let ts_uid = obj.meta().transfer_syntax();
            use dicom_encoding::TransferSyntaxIndex;
            let ts = dicom_transfer_syntax_registry::TransferSyntaxRegistry
                .get(ts_uid)
                .expect("transfer syntax should be registered");
            let mut out = Vec::new();
            obj.write_dataset_with_ts(&mut out, ts)
                .expect("write_dataset_with_ts failed");
            use std::io::Write as _;
            std::io::stdout().write_all(&out).unwrap();
        }
        "verify-borrowed-edge-cases" => {
            // Synthetic object built in-process (not from a fixture) specifically to exercise
            // corners candidate 019's borrowed write path needs to match bit-for-bit against the
            // owned/cloning path: charset_changed=true (force_invalidate_sq_length), a two-level
            // nested SQ, and encapsulated pixel data with a non-empty offset table plus a mix of
            // non-empty AND empty fragments. Real pydicom fixtures don't reliably cover all of
            // this at once. Prints one checksum per write_dataset* method variant.
            use dicom_core::value::{C, PixelFragmentSequence, Value};
            use dicom_core::{DataElement, Tag, VR};
            use dicom_dictionary_std::tags;
            use dicom_encoding::text::SpecificCharacterSet;

            let mut inner2 = InMemDicomObject::new_empty();
            inner2.put(DataElement::new(Tag(0x0008, 0x0020), VR::DA, "20240101"));

            let mut inner1 = InMemDicomObject::new_empty();
            inner1.put(DataElement::new(Tag(0x0008, 0x0018), VR::UI, "1.2.3.4.5"));
            inner1.put(DataElement::new(
                Tag(0x0008, 0x1140),
                VR::SQ,
                Value::new_sequence(
                    C::from_vec(vec![inner2]),
                    dicom_core::Length::UNDEFINED,
                ),
            ));

            let mut obj = InMemDicomObject::new_empty();
            // triggers charset_changed = true via invalidate_if_charset_changed
            obj.put(DataElement::new(
                tags::SPECIFIC_CHARACTER_SET,
                VR::CS,
                "ISO_IR 100",
            ));
            obj.put(DataElement::new(
                Tag(0x0008, 0x0016),
                VR::UI,
                "1.2.840.10008.5.1.4.1.1.7",
            ));
            obj.put(DataElement::new(
                Tag(0x0008, 0x1115),
                VR::SQ,
                Value::new_sequence(C::from_vec(vec![inner1]), dicom_core::Length::UNDEFINED),
            ));
            // encapsulated pixel data: offset table + 3 fragments, middle one empty
            let offset_table: C<u32> = C::from_vec(vec![0, 8]);
            // .into() so this keeps compiling whether InMemFragment is Vec<u8> (baseline) or
            // Arc<[u8]> (candidates 001/008, being re-measured for peak RSS in this session)
            let fragments: C<dicom_core::value::InMemFragment> = C::from_vec(vec![
                vec![1u8, 2, 3, 4, 5, 6, 7, 8].into(),
                Vec::<u8>::new().into(),
                vec![9u8, 10, 11, 12].into(),
            ]);
            let pixel_seq = PixelFragmentSequence::new(offset_table, fragments);
            obj.put(DataElement::new(
                Tag(0x7fe0, 0x0010),
                VR::OB,
                Value::from(pixel_seq),
            ));

            use dicom_encoding::TransferSyntaxIndex;
            let ts = dicom_transfer_syntax_registry::TransferSyntaxRegistry
                .get("1.2.840.10008.1.2.1")
                .unwrap();

            let mut out_ts = Vec::new();
            obj.write_dataset_with_ts(&mut out_ts, ts).unwrap();

            let mut out_ts_cs = Vec::new();
            obj.write_dataset_with_ts_cs(&mut out_ts_cs, ts, SpecificCharacterSet::default())
                .unwrap();

            let mut out_ts_cs_opts = Vec::new();
            obj.write_dataset_with_ts_cs_options(
                &mut out_ts_cs_opts,
                ts,
                SpecificCharacterSet::default(),
                Default::default(),
            )
            .unwrap();

            let encoder = ts.encoder_for().unwrap();
            let mut out_plain = Vec::new();
            obj.write_dataset(&mut out_plain, encoder).unwrap();

            println!(
                "verify-borrowed-edge-cases write_dataset_with_ts checksum={:#x} len={}",
                fnv1a(&out_ts),
                out_ts.len()
            );
            println!(
                "verify-borrowed-edge-cases write_dataset_with_ts_cs checksum={:#x} len={}",
                fnv1a(&out_ts_cs),
                out_ts_cs.len()
            );
            println!(
                "verify-borrowed-edge-cases write_dataset_with_ts_cs_options checksum={:#x} len={}",
                fnv1a(&out_ts_cs_opts),
                out_ts_cs_opts.len()
            );
            println!(
                "verify-borrowed-edge-cases write_dataset checksum={:#x} len={}",
                fnv1a(&out_plain),
                out_plain.len()
            );
        }
        "transcode-to-native-peak-mem" => {
            // single transcode call, then reports this process's peak RSS (VmHWM) - same
            // technique as candidate 019's write-dataset-peak-mem, for candidate 021.
            use dicom_encoding::TransferSyntaxIndex;
            use dicom_pixeldata::Transcode;
            let file = resolve_path(&args[2]);
            let bytes = std::fs::read(&file).unwrap();
            let mut obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            let target_ts = dicom_transfer_syntax_registry::TransferSyntaxRegistry
                .get("1.2.840.10008.1.2.1")
                .expect("transfer syntax should be registered");
            obj.transcode(target_ts).expect("transcode failed");
            let mut out = Vec::new();
            obj.write_all(&mut out).expect("write failed");
            let checksum = fnv1a(&out);
            let status = std::fs::read_to_string("/proc/self/status").unwrap();
            let vmhwm = status
                .lines()
                .find(|l| l.starts_with("VmHWM:"))
                .unwrap_or("VmHWM: unavailable")
                .to_string();
            println!("transcode-to-native-peak-mem {} checksum={:#x} {}", file.display(), checksum, vmhwm.trim());
        }
        "transcode-to-native" => {
            // transcode-to-native <file> <iters>
            //
            // Transcode::transcode from an encapsulated (compressed) transfer syntax to a
            // native/uncompressed one - exercises transcode.rs's decode_inline, specifically
            // its 8-bit arm (candidate 021's target: `decoded_pixeldata.data().to_vec()` ->
            // `decoded_pixeldata.into_data()`). Re-parses fresh each iteration since transcode
            // mutates the object in place (a second transcode call to the same TS is a no-op).
            use dicom_encoding::TransferSyntaxIndex;
            use dicom_pixeldata::Transcode;
            let file = resolve_path(&args[2]);
            let iters: usize = args[3].parse().unwrap();
            let bytes = std::fs::read(&file).unwrap();
            let target_ts = dicom_transfer_syntax_registry::TransferSyntaxRegistry
                .get("1.2.840.10008.1.2.1")
                .expect("transfer syntax should be registered");
            let mut samples = Vec::with_capacity(iters);
            let mut checksum: u64 = 0;
            for _ in 0..iters {
                let mut obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
                let t0 = Instant::now();
                obj.transcode(target_ts).expect("transcode failed");
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                let mut out = Vec::new();
                obj.write_all(&mut out).expect("write failed");
                checksum = fnv1a(&out);
                samples.push(dt);
            }
            println!("transcode-to-native {} ({}/iter) checksum={:#x}", file.display(), stats(samples, "us"), checksum);
        }
        "overlay-decode" => {
            // overlay-decode <rows> <cols> <iters>
            // builds a synthetic in-memory object with one overlay plane (rows x cols, standard
            // packed Overlay Data, not the legacy embedded form) and times PixelDecoder::
            // decode_overlays() - isolates unpack_overlay_bits (candidate 022's target).
            use dicom_core::{DataElement, PrimitiveValue, Tag, VR};
            use dicom_dictionary_std::uids;
            use dicom_object::{DefaultDicomObject, FileDicomObject, FileMetaTableBuilder};
            use dicom_pixeldata::PixelDecoder;
            let rows: u16 = args[2].parse().unwrap();
            let cols: u16 = args[3].parse().unwrap();
            let iters: usize = args[4].parse().unwrap();

            let mut obj: DefaultDicomObject = FileDicomObject::new_empty_with_meta(
                FileMetaTableBuilder::new()
                    .transfer_syntax(uids::EXPLICIT_VR_LITTLE_ENDIAN)
                    .media_storage_sop_class_uid(uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
                    .media_storage_sop_instance_uid("2.25.221545913343461877066277885826230616209")
                    .build()
                    .unwrap(),
            );
            let group = 0x6000u16;
            obj.put_element(DataElement::new(Tag(group, 0x0010), VR::US, PrimitiveValue::from(rows)));
            obj.put_element(DataElement::new(Tag(group, 0x0011), VR::US, PrimitiveValue::from(cols)));
            obj.put_element(DataElement::new(Tag(group, 0x0040), VR::CS, PrimitiveValue::from("R")));
            obj.put_element(DataElement::new(Tag(group, 0x0050), VR::SS, PrimitiveValue::I16([1i16, 1].as_ref().into())));
            obj.put_element(DataElement::new(Tag(group, 0x0100), VR::US, PrimitiveValue::from(1u16)));
            obj.put_element(DataElement::new(Tag(group, 0x0102), VR::US, PrimitiveValue::from(0u16)));
            let num_bytes = (rows as usize * cols as usize).div_ceil(8).div_ceil(2) * 2;
            obj.put_element(DataElement::new(
                Tag(group, 0x3000),
                VR::OB,
                PrimitiveValue::from(vec![0xA5u8; num_bytes]),
            ));

            let mut samples = Vec::with_capacity(iters);
            let mut checksum: u64 = 0;
            for _ in 0..iters {
                let t0 = Instant::now();
                let overlays = obj.decode_overlays().expect("decode_overlays failed");
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                checksum = fnv1a(overlays[0].data());
                samples.push(dt);
            }
            println!("overlay-decode {}x{} ({}/iter) checksum={:#x}", rows, cols, stats(samples, "us"), checksum);
        }
        "colorspace" => {
            let file = resolve_path(&args[2]);
            let iters: usize = args[3].parse().unwrap();
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            let mut samples = Vec::with_capacity(iters);
            let mut checksum: u64 = 0;
            for _ in 0..iters {
                let t0 = Instant::now();
                let decoded = obj.decode_pixel_data().expect("pixel decode failed");
                let img = decoded.to_dynamic_image(0).expect("to_dynamic_image failed");
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                checksum = fnv1a(img.as_bytes());
                samples.push(dt);
            }
            println!("colorspace {} ({}/iter) checksum={:#x}", file.display(), stats(samples, "us"), checksum);
        }
        "colorspace-normalize" => {
            // forces VoiLutOption::Normalize (min/max scan over all samples in
            // Lut::new_rescale_and_normalize), regardless of whether the fixture has its own
            // window center/width - isolates candidate 018's min/max-fold fix
            use dicom_pixeldata::{ConvertOptions, VoiLutOption};
            let file = resolve_path(&args[2]);
            let iters: usize = args[3].parse().unwrap();
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            let options = ConvertOptions::new().with_voi_lut(VoiLutOption::Normalize);
            let mut samples = Vec::with_capacity(iters);
            let mut checksum: u64 = 0;
            for _ in 0..iters {
                let t0 = Instant::now();
                let decoded = obj.decode_pixel_data().expect("pixel decode failed");
                let img = decoded
                    .to_dynamic_image_with_options(0, &options)
                    .expect("to_dynamic_image failed");
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                checksum = fnv1a(img.as_bytes());
                samples.push(dt);
            }
            println!("colorspace-normalize {} ({}/iter) checksum={:#x}", file.display(), stats(samples, "us"), checksum);
        }
        "colorspace-fast" => {
            let file = resolve_path(&args[2]);
            let iters: usize = args[3].parse().unwrap();
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            let mut samples = Vec::with_capacity(iters);
            let mut checksum: u64 = 0;
            for _ in 0..iters {
                let t0 = Instant::now();
                let decoded = obj.decode_pixel_data().expect("pixel decode failed");
                let img = decoded.to_dynamic_image(0).expect("to_dynamic_image failed");
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                checksum = img.as_bytes().len() as u64;
                samples.push(dt);
            }
            println!("colorspace-fast {} ({}/iter) checksum={:#x}", file.display(), stats(samples, "us"), checksum);
        }
        "dump-image" => {
            let file = resolve_path(&args[2]);
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            let decoded = obj.decode_pixel_data().expect("pixel decode failed");
            let img = decoded.to_dynamic_image(0).expect("to_dynamic_image failed");
            use std::io::Write as _;
            std::io::stdout().write_all(img.as_bytes()).unwrap();
        }
        "gen-ybr" => {
            // gen-ybr <src.dcm> <out.dcm> <row-multiplier>
            // Tiles a native (uncompressed, PlanarConfiguration=Standard/pixel-interleaved)
            // SamplesPerPixel=3 source's pixel data vertically by row-multiplier, to reach an
            // arbitrary pixel count while keeping PhotometricInterpretation/BitsAllocated as-is.
            let src = resolve_path(&args[2]);
            let out = PathBuf::from(&args[3]);
            let mult: u32 = args[4].parse().unwrap();
            let bytes = std::fs::read(&src).unwrap();
            let mut obj = from_reader(Cursor::new(&bytes)).expect("parse failed");

            let rows: u16 = obj.element_by_name("Rows").unwrap().to_int().unwrap();
            let cols: u16 = obj.element_by_name("Columns").unwrap().to_int().unwrap();
            let pixel_bytes = obj
                .element_by_name("PixelData")
                .unwrap()
                .value()
                .to_bytes()
                .unwrap()
                .into_owned();
            let new_rows = rows as u32 * mult;
            let mut new_pixel_bytes = Vec::with_capacity(pixel_bytes.len() * mult as usize);
            for _ in 0..mult {
                new_pixel_bytes.extend_from_slice(&pixel_bytes);
            }

            obj.put(DataElement::new(
                tags::ROWS,
                VR::US,
                PrimitiveValue::from(new_rows as u16),
            ));
            obj.put(DataElement::new(
                tags::PIXEL_DATA,
                VR::OB,
                PrimitiveValue::from(new_pixel_bytes),
            ));
            let _ = cols;

            obj.write_to_file(&out).expect("write failed");
            println!(
                "gen-ybr: wrote {} ({} rows x {} cols, {} bytes pixel data)",
                out.display(),
                new_rows,
                cols,
                pixel_bytes.len() * mult as usize
            );
        }
        "gen-ybr-random" => {
            // gen-ybr-random <src.dcm> <out.dcm> <row-multiplier>
            // Same metadata scaffold as gen-ybr, but PixelData is filled with pseudo-random
            // bytes (deterministic xorshift, fixed seed) instead of tiled real content - needed
            // to exercise per-value rounding-boundary differences that a tiled real photo may
            // never happen to hit.
            let src = resolve_path(&args[2]);
            let out = PathBuf::from(&args[3]);
            let mult: u32 = args[4].parse().unwrap();
            let bytes = std::fs::read(&src).unwrap();
            let mut obj = from_reader(Cursor::new(&bytes)).expect("parse failed");

            let rows: u16 = obj.element_by_name("Rows").unwrap().to_int().unwrap();
            let cols: u16 = obj.element_by_name("Columns").unwrap().to_int().unwrap();
            let orig_len = obj
                .element_by_name("PixelData")
                .unwrap()
                .value()
                .to_bytes()
                .unwrap()
                .len();
            let new_rows = rows as u32 * mult;
            let total_len = orig_len * mult as usize;

            let mut state: u64 = 0x243F6A8885A308D3;
            let mut new_pixel_bytes = Vec::with_capacity(total_len);
            for _ in 0..total_len {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                new_pixel_bytes.push((state & 0xff) as u8);
            }

            obj.put(DataElement::new(
                tags::ROWS,
                VR::US,
                PrimitiveValue::from(new_rows as u16),
            ));
            obj.put(DataElement::new(
                tags::PIXEL_DATA,
                VR::OB,
                PrimitiveValue::from(new_pixel_bytes),
            ));
            let _ = cols;

            obj.write_to_file(&out).expect("write failed");
            println!(
                "gen-ybr-random: wrote {} ({} rows x {} cols, {} bytes pixel data)",
                out.display(),
                new_rows,
                cols,
                total_len
            );
        }
        "gen-planar-rgb" => {
            // gen-planar-rgb <src.dcm> <out.dcm> <target-rows> [bits-allocated]
            // Builds a synthetic PlanarConfiguration=1 (PixelFirst), PhotometricInterpretation=RGB
            // image from a same-shape source's metadata scaffold, with random plane content -
            // isolates interleave() alone (no colorspace conversion, since it's not YBR).
            //
            // optional bits-allocated (candidate 034, default 8 if omitted): when 16, builds a
            // BitsAllocated=16 fixture instead (OW pixel data, u16 words) - the only way to
            // exercise interleave::<u16>() (candidate 004's fix shares its generic body with the
            // u8 instantiation, but was only ever measured for u8; no real PlanarConfiguration=1
            // 16-bit fixture exists in dicom-test-files, see IDEAS.md).
            let src = resolve_path(&args[2]);
            let out = PathBuf::from(&args[3]);
            let target_rows: u32 = args[4].parse().unwrap();
            let bits_allocated: u16 = args.get(5).map(|s| s.parse().unwrap()).unwrap_or(8);
            let bytes = std::fs::read(&src).unwrap();
            let mut obj = from_reader(Cursor::new(&bytes)).expect("parse failed");

            let cols: u16 = obj.element_by_name("Columns").unwrap().to_int().unwrap();
            let total_pixels = target_rows as usize * cols as usize;

            obj.put(DataElement::new(
                tags::ROWS,
                VR::US,
                PrimitiveValue::from(target_rows as u16),
            ));
            obj.put(DataElement::new(
                tags::PHOTOMETRIC_INTERPRETATION,
                VR::CS,
                PrimitiveValue::from("RGB"),
            ));
            obj.put(DataElement::new(
                tags::PLANAR_CONFIGURATION,
                VR::US,
                PrimitiveValue::from(1u16),
            ));

            if bits_allocated == 16 {
                let mut state: u64 = 0x9E3779B97F4A7C15;
                let mut new_pixel_words = Vec::with_capacity(total_pixels * 3);
                for _ in 0..(total_pixels * 3) {
                    state ^= state << 13;
                    state ^= state >> 7;
                    state ^= state << 17;
                    new_pixel_words.push((state & 0xffff) as u16);
                }
                obj.put(DataElement::new(
                    tags::BITS_ALLOCATED,
                    VR::US,
                    PrimitiveValue::from(16u16),
                ));
                obj.put(DataElement::new(
                    tags::BITS_STORED,
                    VR::US,
                    PrimitiveValue::from(16u16),
                ));
                obj.put(DataElement::new(
                    tags::HIGH_BIT,
                    VR::US,
                    PrimitiveValue::from(15u16),
                ));
                obj.put(DataElement::new(
                    tags::PIXEL_DATA,
                    VR::OW,
                    PrimitiveValue::U16(new_pixel_words.into()),
                ));
            } else {
                let mut state: u64 = 0x9E3779B97F4A7C15;
                let mut new_pixel_bytes = Vec::with_capacity(total_pixels * 3);
                for _ in 0..(total_pixels * 3) {
                    state ^= state << 13;
                    state ^= state >> 7;
                    state ^= state << 17;
                    new_pixel_bytes.push((state & 0xff) as u8);
                }
                obj.put(DataElement::new(
                    tags::PIXEL_DATA,
                    VR::OB,
                    PrimitiveValue::from(new_pixel_bytes),
                ));
            }

            obj.write_to_file(&out).expect("write failed");
            println!(
                "gen-planar-rgb: wrote {} ({} rows x {} cols, {} total pixels, {}-bit)",
                out.display(),
                target_rows,
                cols,
                total_pixels,
                bits_allocated
            );
        }
        "text-decode" => {
            // text-decode <iters> - ISO_IR 100 (Latin-1) decode microbenchmark, high-byte-heavy
            // repeated content (accented characters need index_forward, not ASCII passthrough).
            let iters: usize = args[2].parse().unwrap();
            let codec = dicom_encoding::text::SpecificCharacterSet::from_code("ISO_IR 100").unwrap();
            let text = "Résultats du patient: température élevée, fréquence cardiaque normale, état général satisfaisant.\\".repeat(200);
            let sample = codec.encode(&text).expect("encode failed");
            let mut samples = Vec::with_capacity(iters);
            let mut checksum: u64 = 0;
            for _ in 0..iters {
                let t0 = Instant::now();
                let s = codec.decode(&sample).expect("decode failed");
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                checksum = fnv1a(s.as_bytes());
                samples.push(dt);
            }
            println!("text-decode ({}/iter) checksum={:#x}", stats(samples, "us"), checksum);
        }
        "text-encode" => {
            // text-encode <iters> - ISO_IR 100 (Latin-1) encode microbenchmark, same content.
            let iters: usize = args[2].parse().unwrap();
            let codec = dicom_encoding::text::SpecificCharacterSet::from_code("ISO_IR 100").unwrap();
            let text = "Résultats du patient: température élevée, fréquence cardiaque normale, état général satisfaisant.\\".repeat(200);
            let mut samples = Vec::with_capacity(iters);
            let mut checksum: u64 = 0;
            for _ in 0..iters {
                let t0 = Instant::now();
                let bytes = codec.encode(&text).expect("encode failed");
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                checksum = fnv1a(&bytes);
                samples.push(dt);
            }
            println!("text-encode ({}/iter) checksum={:#x}", stats(samples, "us"), checksum);
        }
        "text-decode-cs" => {
            // text-decode-cs <code> <iters> - candidate 049: same shape as text-decode above,
            // but for one of the multi-byte decl_character_set! codecs still on the generic
            // encoding-crate path (ISO_IR 13/WINDOWS_31J, ISO_IR 87/ISO_2022_JP,
            // ISO_IR 149/WINDOWS_949, GB18030, GBK). Sample text is real multi-byte content in
            // the matching script (Japanese/Korean/Chinese), not ASCII, so index_forward/
            // multi-byte lead-byte paths are actually exercised.
            let code = args[2].as_str();
            let iters: usize = args[3].parse().unwrap();
            let codec = dicom_encoding::text::SpecificCharacterSet::from_code(code)
                .unwrap_or_else(|| panic!("unknown charset code {code}"));
            let text = multibyte_sample_text(code).repeat(50);
            let sample = codec.encode(&text).expect("encode failed");
            let mut samples = Vec::with_capacity(iters);
            let mut checksum: u64 = 0;
            for _ in 0..iters {
                let t0 = Instant::now();
                let s = codec.decode(&sample).expect("decode failed");
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                checksum = fnv1a(s.as_bytes());
                samples.push(dt);
            }
            println!(
                "text-decode-cs {} ({}/iter) checksum={:#x} bytes={}",
                code,
                stats(samples, "us"),
                checksum,
                sample.len()
            );
        }
        "text-encode-cs" => {
            // text-encode-cs <code> <iters> - encode-side counterpart of text-decode-cs above.
            let code = args[2].as_str();
            let iters: usize = args[3].parse().unwrap();
            let codec = dicom_encoding::text::SpecificCharacterSet::from_code(code)
                .unwrap_or_else(|| panic!("unknown charset code {code}"));
            let text = multibyte_sample_text(code).repeat(50);
            let mut samples = Vec::with_capacity(iters);
            let mut checksum: u64 = 0;
            for _ in 0..iters {
                let t0 = Instant::now();
                let bytes = codec.encode(&text).expect("encode failed");
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                checksum = fnv1a(&bytes);
                samples.push(dt);
            }
            println!(
                "text-encode-cs {} ({}/iter) checksum={:#x}",
                code,
                stats(samples, "us"),
                checksum
            );
        }
        "gen-fragmented" => {
            // gen-fragmented <src.dcm> <out.dcm> <n-fragments> <fragment-size-mb>
            // Replaces the source's PixelData with a large synthetic encapsulated
            // PixelFragmentSequence (n-fragments, each fragment-size-mb MB of raw bytes - not
            // real JPEG, doesn't matter since this is for structural parse/write timing and
            // peak-RSS measurement, not decoding). Needed because color3d_jpeg_baseline.dcm
            // (6.14MB total) is too small for candidates 001/008's clone-avoidance to show above
            // this process's own fixed memory footprint in a peak-RSS measurement - see
            // ___OPTIMIZATIONS___/001.INMEMFRAGMENT_ARC's peak-RSS follow-up.
            use dicom_core::value::{C, InMemFragment, PixelFragmentSequence, Value};
            let src = resolve_path(&args[2]);
            let out = PathBuf::from(&args[3]);
            let n_fragments: usize = args[4].parse().unwrap();
            let fragment_size_mb: usize = args[5].parse().unwrap();
            let bytes = std::fs::read(&src).unwrap();
            let mut obj = from_reader(Cursor::new(&bytes)).expect("parse failed");

            let fragment_size = fragment_size_mb * 1024 * 1024;
            let fragments: C<InMemFragment> = (0..n_fragments)
                .map(|i| vec![(i % 256) as u8; fragment_size].into())
                .collect();
            let offset_table: C<u32> = C::new();
            let total_bytes = n_fragments * fragment_size;
            let pixel_seq = PixelFragmentSequence::new(offset_table, fragments);
            obj.put(DataElement::new(
                tags::PIXEL_DATA,
                VR::OB,
                Value::from(pixel_seq),
            ));
            obj.put(DataElement::new(
                tags::NUMBER_OF_FRAMES,
                VR::IS,
                n_fragments.to_string(),
            ));

            obj.write_to_file(&out).expect("write failed");
            println!(
                "gen-fragmented: wrote {} ({} fragments, {} bytes total pixel data)",
                out.display(), n_fragments, total_bytes
            );
        }
        "gen-mono" => {
            // gen-mono <src.dcm> <out.dcm> <row-multiplier>
            // Tiles a samples_per_pixel=1 (monochrome) source's pixel data vertically to reach
            // an arbitrary pixel count, keeping BitsAllocated/PhotometricInterpretation as-is.
            let src = resolve_path(&args[2]);
            let out = PathBuf::from(&args[3]);
            let mult: u32 = args[4].parse().unwrap();
            let bytes = std::fs::read(&src).unwrap();
            let mut obj = from_reader(Cursor::new(&bytes)).expect("parse failed");

            let rows: u16 = obj.element_by_name("Rows").unwrap().to_int().unwrap();
            let bits_allocated: u16 = obj.element_by_name("BitsAllocated").unwrap().to_int().unwrap();
            let pixel_bytes = obj
                .element_by_name("PixelData")
                .unwrap()
                .value()
                .to_bytes()
                .unwrap()
                .into_owned();
            let new_rows = rows as u32 * mult;
            let mut new_pixel_bytes = Vec::with_capacity(pixel_bytes.len() * mult as usize);
            for _ in 0..mult {
                new_pixel_bytes.extend_from_slice(&pixel_bytes);
            }

            obj.put(DataElement::new(
                tags::ROWS,
                VR::US,
                PrimitiveValue::from(new_rows as u16),
            ));
            let vr = if bits_allocated > 8 { VR::OW } else { VR::OB };
            obj.put(DataElement::new(
                tags::PIXEL_DATA,
                vr,
                PrimitiveValue::from(new_pixel_bytes),
            ));

            obj.write_to_file(&out).expect("write failed");
            println!(
                "gen-mono: wrote {} ({} rows, {} bytes pixel data)",
                out.display(),
                new_rows,
                pixel_bytes.len() * mult as usize
            );
        }
        "tovec" => {
            let file = resolve_path(&args[2]);
            let iters: usize = args[3].parse().unwrap();
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            let decoded = obj.decode_pixel_data().expect("pixel decode failed");
            let mut samples = Vec::with_capacity(iters);
            let mut checksum: u64 = 0;
            for _ in 0..iters {
                let t0 = Instant::now();
                let v: Vec<f32> = decoded.to_vec_frame(0).expect("to_vec_frame failed");
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                checksum = v.len() as u64;
                let _ = std::hint::black_box(&v);
                samples.push(dt);
            }
            println!("tovec {} ({}/iter) checksum={:#x}", file.display(), stats(samples, "us"), checksum);
        }
        "dictlookup" => {
            // dictlookup <iters> - measures StandardDataDictionary::by_tag throughput over a
            // realistic mix of common tags, to check whether the lookup is fast (HashMap-based,
            // as read from source) or a slower linear/binary-search structure.
            use dicom_core::Tag;
            use dicom_core::dictionary::DataDictionary;
            use dicom_dictionary_std::StandardDataDictionary;
            let iters: usize = args[2].parse().unwrap();
            let tags = [
                Tag(0x0008, 0x0018), // SOPInstanceUID
                Tag(0x0010, 0x0010), // PatientName
                Tag(0x0010, 0x0020), // PatientID
                Tag(0x0020, 0x000d), // StudyInstanceUID
                Tag(0x0028, 0x0010), // Rows
                Tag(0x0028, 0x0011), // Columns
                Tag(0x7fe0, 0x0010), // PixelData
                Tag(0x0008, 0x0060), // Modality
                Tag(0x0018, 0x0050), // SliceThickness
                Tag(0x9999, 0x9999), // not present - exercises the miss path
            ];
            let dict = StandardDataDictionary;
            let mut samples = Vec::with_capacity(iters);
            let mut checksum: u64 = 0;
            for _ in 0..iters {
                let t0 = Instant::now();
                let mut hits = 0u64;
                for _ in 0..1000 {
                    for &tag in &tags {
                        if dict.by_tag(tag).is_some() {
                            hits += 1;
                        }
                    }
                }
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                checksum = hits;
                samples.push(dt);
            }
            println!("dictlookup ({}/iter, 10000 lookups/iter) checksum={:#x}", stats(samples, "us"), checksum);
        }
        "json-encode" => {
            let file = resolve_path(&args[2]);
            let iters: usize = args[3].parse().unwrap();
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            let mut samples = Vec::with_capacity(iters);
            let mut checksum: u64 = 0;
            for _ in 0..iters {
                let t0 = Instant::now();
                let json = dicom_json::to_string(&obj).expect("json encode failed");
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                checksum = fnv1a(json.as_bytes());
                if samples.is_empty() {
                    eprintln!("json length: {} bytes", json.len());
                }
                samples.push(dt);
            }
            println!("json-encode {} ({}/iter) checksum={:#x}", file.display(), stats(samples, "us"), checksum);
        }
        "json-decode" => {
            // isolates dicom_json's deserialize path (JsonDataElement's Value/InlineBinary
            // handling in json/src/de/mod.rs) - candidate 026's target (double-deserialization
            // via an eager serde_json::Value DOM before the final typed Vec<T> conversion).
            let file = resolve_path(&args[2]);
            let iters: usize = args[3].parse().unwrap();
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            let json = dicom_json::to_string(&obj).expect("json encode failed");
            let mut samples = Vec::with_capacity(iters);
            let mut last_obj: Option<dicom_object::InMemDicomObject> = None;
            for _ in 0..iters {
                let t0 = Instant::now();
                let obj2: dicom_object::InMemDicomObject =
                    dicom_json::from_str(&json).expect("json decode failed");
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                let _ = std::hint::black_box(&obj2);
                last_obj = Some(obj2);
                samples.push(dt);
            }
            // checksum: hash of the full re-serialized JSON from the last decoded object -
            // a much stronger correctness check than a top-level element count, taken outside
            // the timed loop so it doesn't dilute the decode-only measurement above.
            let reencoded = dicom_json::to_string(&last_obj.unwrap()).expect("json re-encode failed");
            let checksum = fnv1a(reencoded.as_bytes());
            println!("json-decode {} ({}/iter) checksum={:#x}", file.display(), stats(samples, "us"), checksum);
        }
        "json-decode-fromvalue" => {
            // isolates dicom_json::from_value (deserializing directly from an already-built
            // serde_json::Value, e.g. via the json! macro - never has raw JSON text at all) - for
            // candidate 026's own follow-up: that fix's RawValue+from_str mechanism only helps
            // the from_str/from_reader entry points; from_value's own post-fix characteristics
            // were never benchmarked (see IDEAS.md). The pre-built Value is cloned inside the
            // loop (from_value consumes its argument) but the clone itself sits outside the timer,
            // same isolation discipline as json-decode above.
            let file = resolve_path(&args[2]);
            let iters: usize = args[3].parse().unwrap();
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            let json = dicom_json::to_string(&obj).expect("json encode failed");
            let value: serde_json::Value = serde_json::from_str(&json).expect("value parse failed");
            let mut samples = Vec::with_capacity(iters);
            let mut last_obj: Option<dicom_object::InMemDicomObject> = None;
            for _ in 0..iters {
                let value_clone = value.clone();
                let t0 = Instant::now();
                let obj2: dicom_object::InMemDicomObject =
                    dicom_json::from_value(value_clone).expect("json decode failed");
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                let _ = std::hint::black_box(&obj2);
                last_obj = Some(obj2);
                samples.push(dt);
            }
            let reencoded = dicom_json::to_string(&last_obj.unwrap()).expect("json re-encode failed");
            let checksum = fnv1a(reencoded.as_bytes());
            println!(
                "json-decode-fromvalue {} ({}/iter) checksum={:#x}",
                file.display(), stats(samples, "us"), checksum
            );
        }
        "json-decode-peak-mem" => {
            // single dicom_json::from_str call, then reports this process's peak RSS (VmHWM) -
            // same technique as write-only-peak-mem, this time for candidate 026 (RawValue +
            // from_str instead of an eager serde_json::Value DOM), whose peak-memory impact was
            // measured wall-clock-only originally, never for RSS.
            let file = resolve_path(&args[2]);
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            let json = dicom_json::to_string(&obj).expect("json encode failed");
            let obj2: dicom_object::InMemDicomObject =
                dicom_json::from_str(&json).expect("json decode failed");
            let reencoded = dicom_json::to_string(&obj2).expect("json re-encode failed");
            let checksum = fnv1a(reencoded.as_bytes());
            let status = std::fs::read_to_string("/proc/self/status").unwrap();
            let vmhwm = status
                .lines()
                .find(|l| l.starts_with("VmHWM:"))
                .unwrap_or("VmHWM: unavailable")
                .to_string();
            println!("json-decode-peak-mem {} checksum={:#x} {}", file.display(), checksum, vmhwm.trim());
        }
        "decode-raw" => {
            // decode-raw <file> <iters> - isolates PixelDecoder::decode_pixel_data alone (the
            // codec adapter's own decode, e.g. RLE/JPEG), with no LUT/windowing/colorspace work
            // on top (unlike `colorspace`/`tovec`, which both go through build_monochrome_image
            // or convert_pixel_slice's LUT construction as well).
            let file = resolve_path(&args[2]);
            let iters: usize = args[3].parse().unwrap();
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            let mut samples = Vec::with_capacity(iters);
            let mut checksum: u64 = 0;
            for _ in 0..iters {
                let t0 = Instant::now();
                let decoded = obj.decode_pixel_data().expect("pixel decode failed");
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                checksum = fnv1a(decoded.data());
                samples.push(dt);
            }
            println!("decode-raw {} ({}/iter) checksum={:#x}", file.display(), stats(samples, "us"), checksum);
        }
        "decode-frame-raw" => {
            // decode-frame-raw <file> <iters> - isolates PixelDecoder::decode_pixel_data_frame(0)
            // alone (the single-frame codec adapter decode path, e.g. RleLosslessAdapter::decode_frame).
            let file = resolve_path(&args[2]);
            let iters: usize = args[3].parse().unwrap();
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            let mut samples = Vec::with_capacity(iters);
            let mut checksum: u64 = 0;
            for _ in 0..iters {
                let t0 = Instant::now();
                let decoded = obj.decode_pixel_data_frame(0).expect("pixel decode failed");
                let dt = t0.elapsed().as_secs_f64() * 1e6;
                checksum = fnv1a(decoded.data());
                samples.push(dt);
            }
            println!("decode-frame-raw {} ({}/iter) checksum={:#x}", file.display(), stats(samples, "us"), checksum);
        }
        "gen-seq" => {
            // gen-seq <out.dcm> <n-items> [str-len] - builds a synthetic object with one large SQ
            // element (tags::REFERENCED_IMAGE_SEQUENCE) containing n-items nested InMemDicomObjects,
            // each with a couple of simple string attributes - stresses sequence/nested-object
            // handling, which no real dicom-test-files fixture is large enough to exercise
            // (the real ones tried, e.g. rtplan.dcm, are only a few KB).
            //
            // optional str-len (candidate 032): when given, each item also gets a single-valued
            // LO element (InstitutionName) of exactly that many bytes - the short UI-length values
            // above are only 20-40 bytes, too small to separate candidate 024's single-value
            // fast-path effect (a memcpy proportional to value length) from noise; see IDEAS.md.
            let out = PathBuf::from(&args[2]);
            let n: usize = args[3].parse().unwrap();
            let str_len: Option<usize> = args.get(4).map(|s| s.parse().unwrap());
            use dicom_core::{DataElement, VR, dicom_value};
            use dicom_core::value::DataSetSequence;
            let items: Vec<InMemDicomObject> = (0..n)
                .map(|i| {
                    let mut elems = vec![
                        DataElement::new(
                            tags::REFERENCED_SOP_CLASS_UID,
                            VR::UI,
                            dicom_value!(Str, "1.2.840.10008.5.1.4.1.1.7"),
                        ),
                        DataElement::new(
                            tags::REFERENCED_SOP_INSTANCE_UID,
                            VR::UI,
                            dicom_value!(Str, format!("1.2.3.4.5.6.7.8.9.{i}")),
                        ),
                    ];
                    if let Some(len) = str_len {
                        let long_value: String = (0..len)
                            .map(|j| (b'A' + ((i + j) % 26) as u8) as char)
                            .collect();
                        elems.push(DataElement::new(
                            tags::INSTITUTION_NAME,
                            VR::LO,
                            dicom_value!(Str, long_value),
                        ));
                    }
                    InMemDicomObject::from_element_iter(elems)
                })
                .collect();
            let mut obj = InMemDicomObject::new_empty();
            obj.put(DataElement::new(
                tags::REFERENCED_IMAGE_SEQUENCE,
                VR::SQ,
                DataSetSequence::from(items),
            ));
            let obj = obj
                .with_meta(
                    dicom_object::meta::FileMetaTableBuilder::new()
                        .transfer_syntax("1.2.840.10008.1.2.1"),
                )
                .expect("failed to build file meta");
            obj.write_to_file(&out).expect("write failed");
            println!("gen-seq: wrote {} ({} sequence items)", out.display(), n);
        }
        "gen-flat" => {
            // gen-flat <out.dcm> <n-elements> - builds an object with n-elements top-level UI-VR
            // string elements (no SQ nesting), each with the same short-string content gen-seq's
            // own per-item elements use - for candidate 026's own JSON deserialize fix (isolating
            // how much of its win is the SQ-recursion path vs. the flat-multi-element path; see
            // IDEAS.md - both gen-seq and gen-flat give the same total element count for a given
            // n so the two can be compared directly).
            let out = PathBuf::from(&args[2]);
            let n: usize = args[3].parse().unwrap();
            use dicom_core::{DataElement, Tag, VR, dicom_value};
            let mut obj = InMemDicomObject::new_empty();
            for i in 0..n {
                // private-group tags, cycling group as element numbers wrap past u16 - purely
                // structural, VR is supplied explicitly below so no dictionary lookup is needed.
                let tag = Tag((0x0009 + (i / 65536) as u16 * 2) as u16, (i % 65536) as u16);
                obj.put(DataElement::new(
                    tag,
                    VR::UI,
                    dicom_value!(Str, format!("1.2.3.4.5.6.7.8.9.{i}")),
                ));
            }
            let obj = obj
                .with_meta(
                    dicom_object::meta::FileMetaTableBuilder::new()
                        .transfer_syntax("1.2.840.10008.1.2.1"),
                )
                .expect("failed to build file meta");
            obj.write_to_file(&out).expect("write failed");
            println!("gen-flat: wrote {} ({} top-level elements)", out.display(), n);
        }
        "gen-at" => {
            // gen-at <out.dcm> <n-tags> - builds a synthetic object with one AT-VR element
            // (tags::ATTRIBUTE_IDENTIFIER_LIST) containing n-tags distinct Tag values - for
            // measuring candidate 027's AsAttributeTags hex-table fix (json/src/ser/value.rs),
            // which had no real dicom-test-files fixture exercising the AT VR at any scale.
            let out = PathBuf::from(&args[2]);
            let n: usize = args[3].parse().unwrap();
            use dicom_core::{DataElement, PrimitiveValue, Tag, VR};
            let tags_vec: Vec<Tag> = (0..n)
                .map(|i| Tag((i / 65536) as u16, (i % 65536) as u16))
                .collect();
            let mut obj = InMemDicomObject::new_empty();
            obj.put(DataElement::new(
                tags::ATTRIBUTE_IDENTIFIER_LIST,
                VR::AT,
                PrimitiveValue::Tags(tags_vec.into()),
            ));
            let obj = obj
                .with_meta(
                    // implicit VR LE (4-byte length field) - AT is a "short" VR in explicit
                    // encoding (2-byte length, 65535-byte max), too small to hold more than
                    // ~16,000 tags in one element.
                    dicom_object::meta::FileMetaTableBuilder::new()
                        .transfer_syntax("1.2.840.10008.1.2"),
                )
                .expect("failed to build file meta");
            obj.write_to_file(&out).expect("write failed");
            println!("gen-at: wrote {} ({} AT tags)", out.display(), n);
        }
        "gen-datetime-vrs" => {
            // gen-datetime-vrs <out.dcm> <n-items> - builds a flat object with n-items groups of
            // 5 single-valued elements (DA, DS, DT, IS, TM), one of each VR per group, at
            // distinct private-group tags - for measuring candidate 043's single-value fast path
            // extension to read_value_da/ds/dt/is/tm (parser/src/stateful/decode.rs), since no
            // real dicom-test-files fixture has enough of these VRs to time meaningfully.
            let out = PathBuf::from(&args[2]);
            let n: usize = args[3].parse().unwrap();
            use dicom_core::{DataElement, Tag, VR, dicom_value};
            let mut obj = InMemDicomObject::new_empty();
            for i in 0..n {
                let group = 0x0009 + (i / 13107) as u16 * 2; // stay within one group's element range
                let base = ((i % 13107) * 5) as u16;
                obj.put(DataElement::new(
                    Tag(group, base.wrapping_add(0)),
                    VR::DA,
                    dicom_value!(Str, "20260101"),
                ));
                obj.put(DataElement::new(
                    Tag(group, base.wrapping_add(1)),
                    VR::DS,
                    dicom_value!(Str, "3.14159"),
                ));
                obj.put(DataElement::new(
                    Tag(group, base.wrapping_add(2)),
                    VR::DT,
                    dicom_value!(Str, "20260101120000"),
                ));
                obj.put(DataElement::new(
                    Tag(group, base.wrapping_add(3)),
                    VR::IS,
                    dicom_value!(Str, format!("{}", i % 1000)),
                ));
                obj.put(DataElement::new(
                    Tag(group, base.wrapping_add(4)),
                    VR::TM,
                    dicom_value!(Str, "120000"),
                ));
            }
            let obj = obj
                .with_meta(
                    dicom_object::meta::FileMetaTableBuilder::new()
                        .transfer_syntax("1.2.840.10008.1.2.1"),
                )
                .expect("failed to build file meta");
            obj.write_to_file(&out).expect("write failed");
            println!("gen-datetime-vrs: wrote {} ({} groups x 5 elements)", out.display(), n);
        }
        "info" => {
            let file = resolve_path(&args[2]);
            let bytes = std::fs::read(&file).unwrap();
            let obj = from_reader(Cursor::new(&bytes)).expect("parse failed");
            println!("length: {}", obj.length().0);
            print_elem(&obj, "BitsAllocated");
            print_elem(&obj, "SamplesPerPixel");
            print_elem(&obj, "PhotometricInterpretation");
            print_elem(&obj, "NumberOfFrames");
            print_elem(&obj, "PlanarConfiguration");
            print_elem(&obj, "Rows");
            print_elem(&obj, "Columns");
        }
        _ => {
            eprintln!("unknown mode {mode}");
            std::process::exit(1);
        }
    }
}

fn print_elem(obj: &InMemDicomObject, name: &str) {
    match obj.element_by_name(name) {
        Ok(e) => println!("{name} = {:?}", e.value()),
        Err(_) => println!("{name} = <absent>"),
    }
}
