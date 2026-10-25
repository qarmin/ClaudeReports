//! Allocation counter for egui scenes: allocations and bytes per iteration, single-threaded.
use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

struct Counting;
static LOG: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static BIG: [AtomicUsize; 512] = [const { AtomicUsize::new(0) }; 512];
static NBIG: AtomicUsize = AtomicUsize::new(0);
fn log_big(n: usize) {
    if LOG.load(Relaxed) && n >= 2048 {
        let i = NBIG.fetch_add(1, Relaxed);
        if i < 512 {
            BIG[i].store(n, Relaxed);
        }
    }
}
static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);
static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        log_big(l.size());
        BYTES.fetch_add(l.size(), Relaxed);
        let live = LIVE.fetch_add(l.size(), Relaxed) + l.size();
        PEAK.fetch_max(live, Relaxed);
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        LIVE.fetch_sub(l.size(), Relaxed);
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        log_big(n);
        BYTES.fetch_add(n, Relaxed);
        if n >= l.size() {
            let live = LIVE.fetch_add(n - l.size(), Relaxed) + n - l.size();
            PEAK.fetch_max(live, Relaxed);
        } else {
            LIVE.fetch_sub(l.size() - n, Relaxed);
        }
        unsafe { System.realloc(p, l, n) }
    }
}
#[global_allocator]
static A: Counting = Counting;

fn measure(name: &str, iters: usize, mut f: impl FnMut()) {
    for _ in 0..20 {
        f();
    }
    let (a0, b0) = (ALLOCS.load(Relaxed), BYTES.load(Relaxed));
    PEAK.store(LIVE.load(Relaxed), Relaxed);
    let live0 = LIVE.load(Relaxed);
    for _ in 0..iters {
        f();
    }
    let a = (ALLOCS.load(Relaxed) - a0) as f64 / iters as f64;
    let b = (BYTES.load(Relaxed) - b0) as f64 / iters as f64;
    println!(
        "{name}: allocs/iter {a:.1} bytes/iter {b:.0} peak_live_delta {}",
        PEAK.load(Relaxed).saturating_sub(live0)
    );
}

fn main() {
    if std::env::var("SCENE").as_deref() == Ok("layouthash") {
        use std::hash::{Hash, Hasher};
        let mut fonts = epaint::text::Fonts::new(Default::default(), epaint::text::FontDefinitions::default());
        let texts = ["Hello, World! fi ffl AV To", "مرحبا بالعالم 123 abc", "你好,世界 こんにちは 한국어", "emoji 😀👩‍👩‍👧 flag 🇵🇱 e\u{301}", "tab\there\r\nline2\nline3 wrapped text that is long enough to wrap around the width", "Ünïcödé ÅÄÖ ß ½ ™"];
        let mut h = std::collections::hash_map::DefaultHasher::new();
        for ppp in [1.0_f32, 1.5, 2.0] {
            for (i, t) in texts.iter().enumerate() {
                for size in [10.0_f32, 14.0, 33.3] {
                    let g = fonts.with_pixels_per_point(ppp).layout((*t).to_owned(), epaint::FontId::proportional(size), epaint::Color32::WHITE, 120.0 + i as f32 * 10.0);
                    g.rect.min.x.to_bits().hash(&mut h);
                    g.rect.max.x.to_bits().hash(&mut h);
                    g.rect.max.y.to_bits().hash(&mut h);
                    for row in &g.rows {
                        for gl in &row.glyphs {
                            gl.pos.x.to_bits().hash(&mut h);
                            gl.pos.y.to_bits().hash(&mut h);
                            gl.advance_width.to_bits().hash(&mut h);
                            gl.chr.hash(&mut h);
                            gl.uv_rect.min.hash(&mut h);
                        }
                    }
                }
            }
        }
        println!("layout hash {:x}", h.finish());
        return;
    }
    if std::env::var("SCENE").as_deref() == Ok("startup") {
        let (a0, b0, l0) = (ALLOCS.load(Relaxed), BYTES.load(Relaxed), LIVE.load(Relaxed));
        let mut fonts = epaint::text::Fonts::new(Default::default(), epaint::text::FontDefinitions::default());
        let (a1, b1, l1) = (ALLOCS.load(Relaxed), BYTES.load(Relaxed), LIVE.load(Relaxed));
        black_box(fonts.with_pixels_per_point(1.0).layout_no_wrap("Hello world 123".to_owned(), epaint::FontId::default(), epaint::Color32::WHITE));
        let (a2, b2, l2) = (ALLOCS.load(Relaxed), BYTES.load(Relaxed), LIVE.load(Relaxed));
        println!("Fonts::new: allocs {} bytes {} live {}", a1 - a0, b1 - b0, l1 - l0);
        println!("first layout: allocs {} bytes {} live {}", a2 - a1, b2 - b1, l2 - l1);
        let ctx = egui::Context::default();
        let (a3, b3, l3) = (ALLOCS.load(Relaxed), BYTES.load(Relaxed), LIVE.load(Relaxed));
        let out = ctx.run_ui(egui::RawInput::default(), |ui| { ui.label("hi"); });
        out.drop_without_applying_deltas();
        let (a4, b4, l4) = (ALLOCS.load(Relaxed), BYTES.load(Relaxed), LIVE.load(Relaxed));
        println!("Context::default: allocs {} bytes {} live {}", a3 - a2, b3 - b2, l3 - l2);
        println!("first frame: allocs {} bytes {} live {}", a4 - a3, b4 - b3, l4 - l3);
        return;
    }
    use egui::RawInput;
    let ctx = egui::Context::default();
    let mut demo = egui_demo_lib::DemoWindows::default();
    if std::env::var("SCENE").as_deref() == Ok("ui") {
        for _ in 0..300 {
            let out = ctx.run_ui(RawInput::default(), |ui| demo.ui(ui));
            out.drop_without_applying_deltas();
        }
        return;
    }
    measure("demo_run_ui", 200, || {
        let out = ctx.run_ui(RawInput::default(), |ui| demo.ui(ui));
        out.drop_without_applying_deltas();
    });
    measure("demo_run_ui+tessellate", 200, || {
        let mut out = ctx.run_ui(RawInput::default(), |ui| demo.ui(ui));
        black_box(ctx.tessellate(out.shapes, out.pixels_per_point));
        out.textures_delta.clear();
    });
    let out = ctx.run_ui(RawInput::default(), |ui| demo.ui(ui));
    measure("demo_only_tessellate", 200, || {
        black_box(ctx.tessellate(out.shapes.clone(), out.pixels_per_point));
    });
    if std::env::var("BIGTESS").is_ok() {
        let shapes = out.shapes.clone();
        LOG.store(true, Relaxed);
        black_box(ctx.tessellate(shapes, out.pixels_per_point));
        LOG.store(false, Relaxed);
        let n = NBIG.load(Relaxed).min(512);
        let v: Vec<usize> = (0..n).map(|i| BIG[i].load(Relaxed)).collect();
        println!("big allocs in one tessellate ({n}): {v:?}");
    }
    out.drop_without_applying_deltas();

    let mut fonts = epaint::text::Fonts::new(Default::default(), epaint::text::FontDefinitions::default());
    let font_id = epaint::FontId::default();
    measure("text_layout_uncached", 200, || {
        let job = epaint::text::LayoutJob::simple(
            egui_demo_lib::LOREM_IPSUM_LONG.to_owned(),
            font_id.clone(),
            epaint::Color32::WHITE,
            512.0,
        );
        black_box(fonts.layout_uncached(1.0, job.into()));
    });
    if std::env::var("BIG").is_ok() {
        LOG.store(true, Relaxed);
        let job = epaint::text::LayoutJob::simple(egui_demo_lib::LOREM_IPSUM_LONG.to_owned(), font_id.clone(), epaint::Color32::WHITE, 512.0);
        black_box(fonts.layout_uncached(1.0, job.into()));
        LOG.store(false, Relaxed);
        let n = NBIG.load(Relaxed).min(512);
        let v: Vec<usize> = (0..n).map(|i| BIG[i].load(Relaxed)).collect();
        println!("big allocs in one uncached layout: {v:?}");
    }
    measure("text_layout_cached", 2000, || {
        black_box(fonts.with_pixels_per_point(1.0).layout(
            egui_demo_lib::LOREM_IPSUM_LONG.to_owned(),
            font_id.clone(),
            epaint::Color32::WHITE,
            512.0,
        ));
    });
    measure("short_label_cached", 20000, || {
        black_box(fonts.with_pixels_per_point(1.0).layout_no_wrap(
            "Hello world".to_owned(),
            font_id.clone(),
            epaint::Color32::WHITE,
        ));
    });
}
