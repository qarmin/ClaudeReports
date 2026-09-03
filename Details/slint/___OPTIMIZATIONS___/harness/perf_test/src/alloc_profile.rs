//! Sampling allocation profiler: every Nth allocation stores an unresolved backtrace in a
//! fixed size table, symbols are resolved once when the report is printed. This exists
//! because `perf`, `valgrind` and `heaptrack` are not always available, and the question
//! "which code allocates 700 000 times per frame" needs an answer either way.
//!
//! Nothing here allocates while sampling (fixed static table, unresolved frames), and the
//! sampler is off unless [`start`] is called.

use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

const MAX_FRAMES: usize = 14;
const BUCKETS: usize = 4096;
/// Frames at the top of every capture that only describe the allocator itself.
const SKIPPED_TOP_FRAMES: usize = 3;

struct Table {
    keys: [u64; BUCKETS],
    frames: [[usize; MAX_FRAMES]; BUCKETS],
    counts: [u64; BUCKETS],
    used: usize,
    dropped: u64,
}

static TABLE: Mutex<Table> = Mutex::new(Table {
    keys: [0; BUCKETS],
    frames: [[0; MAX_FRAMES]; BUCKETS],
    counts: [0; BUCKETS],
    used: 0,
    dropped: 0,
});

/// 0 means "not sampling".
static RATE: AtomicU64 = AtomicU64::new(0);
/// Kept across [`stop`] so the report can scale the counts.
static REPORT_RATE: AtomicU64 = AtomicU64::new(0);
static SEEN: AtomicU64 = AtomicU64::new(0);

thread_local! {
    static IN_SAMPLER: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub fn start(rate: u64) {
    let mut table = TABLE.lock().unwrap();
    table.keys.fill(0);
    table.counts.fill(0);
    table.used = 0;
    table.dropped = 0;
    drop(table);
    SEEN.store(0, Ordering::Relaxed);
    RATE.store(rate.max(1), Ordering::Relaxed);
    REPORT_RATE.store(rate.max(1), Ordering::Relaxed);
}

pub fn stop() {
    RATE.store(0, Ordering::Relaxed);
}

/// Called from the global allocator on every allocation.
pub fn sample() {
    let rate = RATE.load(Ordering::Relaxed);
    if rate == 0 {
        return;
    }
    if !SEEN.fetch_add(1, Ordering::Relaxed).is_multiple_of(rate) {
        return;
    }
    // `backtrace::trace` itself may allocate on the first call per process; the guard keeps
    // that from recursing into the sampler.
    if IN_SAMPLER.with(|flag| flag.replace(true)) {
        return;
    }

    let mut frames = [0usize; MAX_FRAMES];
    let mut index = 0;
    let mut skipped = 0;
    backtrace::trace(|frame| {
        if skipped < SKIPPED_TOP_FRAMES {
            skipped += 1;
            return true;
        }
        frames[index] = frame.ip() as usize;
        index += 1;
        index < MAX_FRAMES
    });

    let mut hash = 0xcbf29ce484222325u64;
    for ip in &frames {
        hash = (hash ^ *ip as u64).wrapping_mul(0x100000001b3);
    }
    hash |= 1; // 0 marks an empty bucket

    let mut table = TABLE.lock().unwrap();
    let mut bucket = (hash as usize) % BUCKETS;
    loop {
        if table.keys[bucket] == hash {
            table.counts[bucket] += 1;
            break;
        }
        if table.keys[bucket] == 0 {
            if table.used * 4 >= BUCKETS * 3 {
                table.dropped += 1;
                break;
            }
            table.keys[bucket] = hash;
            table.frames[bucket] = frames;
            table.counts[bucket] = 1;
            table.used += 1;
            break;
        }
        bucket = (bucket + 1) % BUCKETS;
    }
    drop(table);

    IN_SAMPLER.with(|flag| flag.set(false));
}

/// Aggregates the samples by the innermost frame that belongs to Slint, femtovg or this
/// binary - i.e. by allocation *site*, ignoring the `Vec`/`HashMap` internals above it.
pub fn report_by_site(top: usize) {
    let rate = REPORT_RATE.load(Ordering::Relaxed).max(1);
    let table = TABLE.lock().unwrap();
    let stacks: Vec<(u64, [usize; MAX_FRAMES])> = (0..BUCKETS)
        .filter(|bucket| table.keys[*bucket] != 0)
        .map(|bucket| (table.counts[bucket], table.frames[bucket]))
        .collect();
    drop(table);

    let mut sites: std::collections::HashMap<String, u64> = std::collections::HashMap::new();
    let mut total = 0;
    for (count, frames) in &stacks {
        total += count;
        let mut site = None;
        for ip in frames.iter().take_while(|ip| **ip != 0) {
            let mut found = None;
            backtrace::resolve(*ip as *mut _, |symbol| {
                if found.is_some() {
                    return;
                }
                let name = symbol.name().map(|name| format!("{name:#}")).unwrap_or_default();
                let interesting = (name.contains("i_slint_")
                    || name.contains("femtovg::")
                    || name.contains("perf_test::"))
                    && !name.contains("alloc_profile");
                if interesting {
                    let line = symbol.lineno().map(|l| format!(":{l}")).unwrap_or_default();
                    let file = symbol
                        .filename()
                        .map(|f| {
                            let f = f.to_string_lossy().to_string();
                            f.rsplit_once('/').map(|(_, t)| t.to_string()).unwrap_or(f)
                        })
                        .unwrap_or_default();
                    found = Some(format!("{name}  [{file}{line}]"));
                }
            });
            if let Some(name) = found {
                site = Some(name);
                break;
            }
        }
        *sites.entry(site.unwrap_or_else(|| "<unattributed>".into())).or_default() += count;
    }

    let mut sites: Vec<(String, u64)> = sites.into_iter().collect();
    sites.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

    println!();
    println!("allocation sites (~{} allocations sampled 1 in {rate}):", total * rate);
    for (site, count) in sites.iter().take(top) {
        println!("  ~{:>8} ({:>5.1}%)  {site}", count * rate, 100.0 * *count as f64 / total as f64);
    }
}

/// Prints the `top` heaviest allocation stacks, scaled back up by the sampling rate.
pub fn report(top: usize, frames_per_stack: usize) {
    let rate = REPORT_RATE.load(Ordering::Relaxed).max(1);
    let table = TABLE.lock().unwrap();
    let mut stacks: Vec<(u64, [usize; MAX_FRAMES])> = (0..BUCKETS)
        .filter(|bucket| table.keys[*bucket] != 0)
        .map(|bucket| (table.counts[bucket], table.frames[bucket]))
        .collect();
    let dropped = table.dropped;
    drop(table);

    stacks.sort_by_key(|(count, _)| std::cmp::Reverse(*count));
    let total: u64 = stacks.iter().map(|(count, _)| count).sum();

    println!();
    println!(
        "allocation profile: {} samples (1 in {rate}) = ~{} allocations, {} distinct stacks{}",
        total,
        total * rate,
        stacks.len(),
        if dropped > 0 {
            format!(", {dropped} samples dropped (table full)")
        } else {
            String::new()
        }
    );

    for (count, frames) in stacks.iter().take(top) {
        println!();
        println!("~{} allocations ({:.1}%)", count * rate, 100.0 * *count as f64 / total as f64);
        for ip in frames.iter().take(frames_per_stack) {
            if *ip == 0 {
                break;
            }
            let mut printed = false;
            backtrace::resolve(*ip as *mut _, |symbol| {
                if printed {
                    return;
                }
                printed = true;
                let name = symbol.name().map(|name| format!("{name:#}")).unwrap_or_default();
                match (symbol.filename(), symbol.lineno()) {
                    (Some(file), Some(line)) => {
                        let file = file.to_string_lossy();
                        let short =
                            file.rsplit_once("/src/").map(|(_, tail)| tail).unwrap_or(&file);
                        println!("    {name}  [{short}:{line}]");
                    }
                    _ => println!("    {name}"),
                }
            });
            if !printed {
                println!("    {ip:#x}");
            }
        }
    }
}
