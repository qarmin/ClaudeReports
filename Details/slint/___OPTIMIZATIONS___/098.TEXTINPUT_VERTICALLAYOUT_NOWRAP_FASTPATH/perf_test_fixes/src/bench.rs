use std::rc::Rc;
use std::time::{Duration, Instant};

use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Platform, PlatformError, WindowAdapter};
use slint::{ComponentHandle, Model, PhysicalSize, Rgb8Pixel};

use crate::alloc_stats;
use crate::logic::wire_up;
use crate::{Logic, MainWindow, PerfMode, RowStyle, Settings};

pub struct BenchOptions {
    pub mode: PerfMode,
    pub count: i32,
    pub frames: usize,
    pub scroll_step: f32,
    pub width: u32,
    pub height: u32,
    pub flat_rows: bool,
    pub row_style: RowStyle,
    /// `rects` scene gradient fill: 0 = off, 1 = linear, 2 = radial, 3 = conic.
    pub gradient_kind: u8,
    /// Lay the `text` scene out with `wrap: word-wrap` instead of the default `no-wrap`.
    pub word_wrap: bool,
    /// Change the window width by this many pixels before each frame, alternating in sign, instead
    /// of scrolling. What a resize costs is a different question from what scrolling costs: it
    /// re-runs the layout at a new width rather than reusing it.
    pub resize_step: Option<u32>,
    /// Mutate this many model rows before each frame. Live data is what real list UIs do and no
    /// scene measured it: scrolling a static model exercises the renderer, not the model plumbing.
    /// `rects` scene: give every rectangle a binding on the animation clock.
    pub animate: bool,
    /// Animate the fill rather than the position: same bindings, unchanged geometry.
    pub animate_color: bool,
    /// Which buffer the software renderer assumes it is drawing into. `NewBuffer` is Slint's
    /// *default* and repaints everything; every measurement in this workspace used `ReusedBuffer`.
    pub repaint_buffer: RepaintBufferType,
    /// Render through `render_by_line` instead of `render`. These are two different implementations
    /// of the software renderer - the by-line one builds and sorts a `Scene` of spans, the other
    /// draws straight into the buffer - and only the second has ever been measured here. Embedded
    /// targets without a full framebuffer use the first.
    pub render_by_line: bool,
    /// Explicit fill alpha for `rects`; negative keeps the palette colours.
    pub item_alpha: f32,
    /// `rects`: draw a `Path` per item instead of a `Rectangle`.
    pub use_path: bool,
    /// `rects`: give every item a drop shadow, exercising femtovg's `draw_box_shadow`.
    pub item_shadow: bool,
    /// How many rectangles animate, so the per-animated-item cost separates from the per-item one.
    pub animate_count: i32,
    /// Dispatch a pointer-move event before each frame. Hit-testing walks the item tree, and a
    /// mouse moving over a list is something a real UI does on every single frame - no scene here
    /// dispatched any input at all.
    /// `images` scene: give each item its own source instead of sharing one.
    pub image_distinct: bool,
    /// `images` scene: size the item away from the source's natural size.
    pub image_rescale: bool,
    /// `images` scene: use `Repeat` horizontal/vertical tiling with a source crop smaller than
    /// the full image - the femtovg `draw_image_impl` branch that creates, blits into and
    /// schedules the deletion of a temporary GPU texture (`candidate FEMTOVG_TILED_9SLICE_TEXTURE_PER_FRAME`).
    pub image_tiling: bool,
    /// `images` scene: use a nine-slice source (fixed 24px borders on the built-in
    /// `nine_slice.png` asset), the femtovg `draw_image_impl` path that used to `collect::<Vec<_>>()`
    /// the `fit9slice` iterator on every frame regardless of whether anything changed
    /// (candidate FEMTOVG_9SLICE_FITS_VEC). The software renderer's equivalent site streams the
    /// same iterator with no allocation, so this exercises the gap directly.
    pub image_nine_slice: bool,
    /// `images` scene: pixel size of each generated source.
    pub image_size: u32,
    /// Load the shared source from `perf_test/assets/<name>` instead of generating it. A real
    /// PNG or JPEG is the only way to reach the decoder and the RGB vs RGBA renderer paths.
    pub image_file: Option<String>,
    /// Logical px each image item is drawn at.
    pub image_cell: f32,
    /// Dispatch cursor-movement keys before each frame. `text` mode: editing a large document is
    /// what a `TextEdit` is for, and candidate 03 made the glyphs of far-away paragraphs lazy - so
    /// moving the cursor through one is exactly the interaction that could pay for it.
    /// Flip a global property every frame. Every row's background reads it, so one write dirties
    /// one binding per row - a theme switch. The other scenes only write properties the renderer
    /// reads, which is a different path entirely.
    /// `nest` scene: how many levels each leaf is wrapped in.
    pub nest_depth: i32,
    /// `nest` scene: wrap in layouts (which query children) instead of plain rectangles.
    pub nest_plain: bool,
    /// `layouts` scene: children per container, layout kind (0=h,1=v,2=grid,3=none), and whether
    /// one child stretches.
    pub layout_children: i32,
    pub layout_kind: i32,
    pub layout_stretch: bool,
    pub toggle_global: bool,
    /// Select the whole document once, before the frames. A selection makes the draw path resolve
    /// selection geometry every frame, which walks paragraphs - so a huge selection is the case
    /// that could turn out O(document).
    /// Type one character per frame into the focused input. A text change invalidates the whole
    /// shaped-text cache entry, so this is the interaction with the most to lose in a large
    /// document - and the one a `TextEdit` exists for.
    /// Scroll with the wheel instead of writing the scroll property. Real scrolling goes through
    /// `Flickable`, which the property-driven scroll of every other scene bypasses entirely.
    /// Characters per generated line in `text` mode. A paragraph is the unit of lazy shaping, so a
    /// few very long lines and many short ones are entirely different workloads even at the same
    /// total byte count - minified JSON and CSV rows are the former.
    /// Distinct font sizes the rows cycle through. Every (font, size) pair is its own glyph-cache
    /// entry and its own shaping key.
    /// `rects`: per-item opacity (<1 may force an offscreen layer per item).
    /// Wrap the records model in an adapter. Filter/sort/map adapters are what real apps put
    /// between a model and a view, and no scene had one - the mutation paths through them are
    /// completely different from a bare `VecModel`.
    /// `rects`: toggle a subtree of this many items in and out with an `if`, every frame.
    /// Window scale factor. Everything in this workspace was measured at 1.0, which is not what a
    /// modern display does - glyphs are rasterized per (font, size, scale) and the shaped-text
    /// cache is keyed on it, so a HiDPI window is a different workload, not the same one bigger.
    pub scale_factor: f32,
    /// Alternate the scale factor every frame. Dragging a window between a HiDPI and a normal
    /// monitor does this, and the shaped-text cache is cleared wholesale when it changes.
    pub scale_toggle: bool,
    pub toggle_visibility: i32,
    pub model_adapter: ModelAdapter,
    pub item_opacity: f32,
    /// `rects`: corner radius and border width in logical px.
    pub item_radius: f32,
    pub item_border: f32,
    pub font_size_variants: i32,
    /// Base font size in logical px, so size and variety can be varied independently.
    pub font_size_base: f32,
    pub line_length: usize,
    /// See `backend::Options::long_line_chars`.
    pub long_line_chars: Option<usize>,
    pub wheel: bool,
    pub type_text: bool,
    pub select_all: bool,
    pub key_input: bool,
    pub pointer_move: bool,
    /// Move the pointer within one row, so the hit test runs but the item stack does not change.
    pub pointer_jitter: bool,
    pub mutate: usize,
    /// Which mutation. They reach different paths through `ModelNotify`.
    pub mutate_mode: MutateMode,
    /// Sample one in N allocations and print where they come from.
    #[cfg_attr(not(feature = "alloc-profile"), allow(dead_code))]
    pub alloc_profile: Option<u64>,
    /// Which phase `alloc_profile` samples.
    #[cfg_attr(not(feature = "alloc-profile"), allow(dead_code))]
    pub alloc_profile_phase: ProfilePhase,
    /// Where to write a PPM of the last frame. Unlike the windowed `--frames` snapshot this needs
    /// no display and no GL, so it is the pixel check that always works here.
    pub snapshot: Option<String>,
    /// See `MainWindow.probe-opacity`/`probe-animate`/`probe-count` in `ui/main.slint`.
    pub probe_opacity: f32,
    pub probe_animate: bool,
    pub probe_count: i32,
    /// See `MainWindow.probe-layer-opacity`/`probe-layer-animate`/`probe-layer-count` in
    /// `ui/main.slint`.
    pub probe_layer_opacity: f32,
    pub probe_layer_animate: bool,
    pub probe_layer_count: i32,
}

impl Default for BenchOptions {
    fn default() -> Self {
        Self {
            mode: PerfMode::Scroll,
            count: 10_000,
            frames: 5,
            scroll_step: 200.0,
            width: 1280,
            height: 860,
            flat_rows: false,
            row_style: RowStyle::Layout,
            gradient_kind: 0,
            word_wrap: false,
            resize_step: None,
            image_distinct: false,
            image_rescale: false,
            image_tiling: false,
            image_nine_slice: false,
            image_size: 64,
            image_file: None,
            image_cell: 68.0,
            nest_depth: 1,
            nest_plain: false,
            layout_children: 4,
            layout_kind: 0,
            layout_stretch: false,
            toggle_global: false,
            scale_factor: 1.0,
            scale_toggle: false,
            toggle_visibility: 0,
            model_adapter: ModelAdapter::None,
            item_opacity: 1.0,
            item_radius: 3.0,
            item_border: 1.0,
            font_size_variants: 1,
            font_size_base: 13.0,
            line_length: crate::generator::LINE_LEN,
            long_line_chars: None,
            wheel: false,
            type_text: false,
            select_all: false,
            key_input: false,
            pointer_move: false,
            pointer_jitter: false,
            animate: false,
            animate_color: false,
            repaint_buffer: RepaintBufferType::ReusedBuffer,
            render_by_line: false,
            item_alpha: -1.0,
            use_path: false,
            item_shadow: false,
            animate_count: i32::MAX,
            mutate: 0,
            mutate_mode: MutateMode::Changed,
            snapshot: None,
            alloc_profile: None,
            alloc_profile_phase: ProfilePhase::LastFrame,
            probe_opacity: 1.0,
            probe_animate: false,
            probe_count: 0,
            probe_layer_opacity: 1.0,
            probe_layer_animate: false,
            probe_layer_count: 0,
        }
    }
}

/// What the records model is wrapped in before the view sees it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ModelAdapter {
    None,
    /// Keeps ~90% of the rows, so the mapping is nearly as long as the source.
    Filter,
    /// Sorts by the third field, which the generator makes unordered.
    Sort,
    /// A cheap per-row transformation.
    Map,
}

/// How `--mutate` changes the model each frame.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MutateMode {
    /// `set_row_data` on rows that are currently visible.
    Changed,
    /// `set_row_data` on rows far past the viewport - the same notification, no visible effect.
    ChangedOffscreen,
    /// `push` at the end, which is what an append-only log does.
    Push,
    /// `insert(0, ..)`, the classic O(n) shape.
    InsertFront,
    /// `remove(0)` paired with a `push`, so the row count stays put.
    Rotate,
}

impl MutateMode {
    fn label(self) -> &'static str {
        match self {
            Self::Changed => "set_row_data on visible rows",
            Self::ChangedOffscreen => "set_row_data on off-screen rows",
            Self::Push => "push at the end",
            Self::InsertFront => "insert at the front",
            Self::Rotate => "remove(0) + push",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ProfilePhase {
    Fill,
    Show,
    LastFrame,
}

struct BenchPlatform {
    window: Rc<MinimalSoftwareWindow>,
}

impl Platform for BenchPlatform {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(self.window.clone())
    }
}

struct Phase {
    duration: Duration,
    allocations: u64,
}

/// Applies one frame's worth of `--mutate` to the model.
///
/// The rows touched move with the frame so the same row isn't rewritten every time, which would
/// let a change-detector suppress the work and measure nothing.
fn mutate_model(records: &Rc<slint::VecModel<crate::RecordItem>>, options: &BenchOptions, frame: usize) {
    let count = records.row_count();
    match options.mutate_mode {
        MutateMode::Changed | MutateMode::ChangedOffscreen => {
            // Visible rows are the first screenful once the scroll offset is applied; off-screen
            // ones sit deliberately far past it.
            let base = match options.mutate_mode {
                MutateMode::ChangedOffscreen => count / 2,
                _ => 0,
            };
            for index in 0..options.mutate {
                let row = (base + index + frame) % count;
                records.set_row_data(row, crate::generator::make_record(row + frame * 7919));
            }
        }
        MutateMode::Push => {
            for index in 0..options.mutate {
                records.push(crate::generator::make_record(count + index));
            }
        }
        MutateMode::InsertFront => {
            for index in 0..options.mutate {
                records.insert(0, crate::generator::make_record(count + index));
            }
        }
        MutateMode::Rotate => {
            for index in 0..options.mutate {
                records.remove(0);
                records.push(crate::generator::make_record(count + index));
            }
        }
    }
}

/// Running totals of the shaping counters. A frame is charged the difference between two reads
/// rather than a reset around `render()`, so layout work triggered before the render - by a
/// resize, say - lands on the frame that triggered it.
#[derive(Clone, Copy, Default)]
struct ShapingTotals {
    text_shapes: u64,
    paragraph_shapes: u64,
    line_breaks: u64,
}

impl ShapingTotals {
    fn since(&self, earlier: &Self) -> Self {
        Self {
            text_shapes: self.text_shapes - earlier.text_shapes,
            paragraph_shapes: self.paragraph_shapes - earlier.paragraph_shapes,
            line_breaks: self.line_breaks - earlier.line_breaks,
        }
    }
}

struct FrameStats {
    duration: Duration,
    allocations: u64,
    /// Items whose text had to be shaped again during this frame.
    text_shapes: u64,
    /// Individual paragraphs shaped during this frame. A large document shapes its paragraphs on
    /// demand, so one item can account for many (or zero) of these.
    paragraph_shapes: u64,
    /// Cached items whose lines had to be broken again.
    line_breaks: u64,
}

/// Hands `render_by_line` one line of the same buffer `render` would have written into, so the two
/// paths are compared on identical output rather than on different targets.
struct LineWriter<'a> {
    buffer: &'a mut [Rgb8Pixel],
    stride: usize,
}

impl slint::platform::software_renderer::LineBufferProvider for LineWriter<'_> {
    type TargetPixel = Rgb8Pixel;

    fn process_line(
        &mut self,
        line: usize,
        range: core::ops::Range<usize>,
        render_fn: impl FnOnce(&mut [Self::TargetPixel]),
    ) {
        let start = line * self.stride;
        render_fn(&mut self.buffer[start + range.start..start + range.end]);
    }
}

/// Headless benchmark: fills the selected mode with `count` items, renders `frames`
/// frames with the software renderer while scrolling, and reports per frame timings.
/// Needs no display, so it also runs over ssh and in CI.
pub fn run(options: &BenchOptions) -> Result<(), PlatformError> {
    let window = MinimalSoftwareWindow::new(options.repaint_buffer);
    slint::platform::set_platform(Box::new(BenchPlatform { window: window.clone() }))
        .map_err(|error| format!("cannot install the benchmark platform: {error}"))?;
    window.set_size(PhysicalSize::new(options.width, options.height));

    if options.scale_factor != 1.0 {
        window.dispatch_event(slint::platform::WindowEvent::ScaleFactorChanged {
            scale_factor: options.scale_factor,
        });
    }

    let app = MainWindow::new()?;
    let records = crate::logic::wire_up_with_line_length(&app, options.line_length);
    match options.model_adapter {
        ModelAdapter::None => {}
        ModelAdapter::Filter => app.set_records(slint::ModelRc::new(
            i_slint_core::model::FilterModel::new(records.clone(), |record: &crate::RecordItem| {
                !record.first.as_str().ends_with('7')
            }),
        )),
        ModelAdapter::Sort => app.set_records(slint::ModelRc::new(
            i_slint_core::model::SortModel::new(
                records.clone(),
                |a: &crate::RecordItem, b: &crate::RecordItem| a.third.cmp(&b.third),
            ),
        )),
        ModelAdapter::Map => app.set_records(slint::ModelRc::new(
            i_slint_core::model::MapModel::new(records.clone(), |record: crate::RecordItem| record),
        )),
    }
    app.set_mode(options.mode);
    app.set_word_wrap(options.word_wrap);
    app.set_probe_opacity(options.probe_opacity);
    app.set_probe_animate(options.probe_animate);
    app.set_probe_count(options.probe_count);
    app.set_probe_layer_opacity(options.probe_layer_opacity);
    app.set_probe_layer_animate(options.probe_layer_animate);
    app.set_probe_layer_count(options.probe_layer_count);
    app.global::<Settings>().set_font_size_variants(options.font_size_variants);
    app.global::<Settings>().set_font_size_base(options.font_size_base);
    app.global::<Settings>().set_flat_rows(options.flat_rows);
    app.global::<Settings>().set_row_style(options.row_style);
    app.set_backend_info("headless software".into());

    #[cfg(feature = "alloc-profile")]
    if let Some(rate) = options.alloc_profile
        && options.alloc_profile_phase == ProfilePhase::Fill
    {
        crate::alloc_profile::start(rate);
    }

    let fill_start = Instant::now();
    let fill_allocations = alloc_stats::allocations();
    match options.mode {
        PerfMode::Text => {
            app.global::<Logic>().invoke_add_lines(options.count);
            if let Some(chars) = options.long_line_chars {
                // Prepended, not appended: `--type-text`/`--key-input` click at a fixed row
                // (see `if options.key_input || ...` below), so inserting here keeps that row on
                // an ordinary generated line, never on this one. `usize::MAX` as the index keeps
                // this line's generated words distinct from every `push_line_of(.., index, ..)`
                // call `invoke_add_lines` already made.
                let mut long_line = String::new();
                crate::generator::push_line_of(&mut long_line, usize::MAX, chars);
                let mut text = String::from(app.get_text_content().as_str());
                text.insert_str(0, &long_line);
                app.set_text_content(text.into());
            }
        }
        PerfMode::List | PerfMode::Scroll => {
            app.global::<Logic>().invoke_add_records(options.count)
        }
        PerfMode::Nest => {
            app.set_rect_count(options.count);
            app.set_nest_depth(options.nest_depth);
            app.set_nest_use_layout(!options.nest_plain);
        }
        PerfMode::Layouts => {
            app.set_rect_count(options.count);
            app.set_layout_children(options.layout_children);
            app.set_layout_kind(options.layout_kind);
            app.set_layout_stretch(options.layout_stretch);
        }
        PerfMode::Images => {
            app.set_rect_count(options.count);
            app.set_image_distinct(options.image_distinct);
            app.set_image_cell(options.image_cell);
            app.set_image_rescale(options.image_rescale);
            app.set_image_tiling(options.image_tiling);
            app.set_image_source({
                let mut img = match &options.image_file {
                    Some(name) => crate::generator::load_asset(name),
                    None if options.image_nine_slice => crate::generator::load_asset("nine_slice.png"),
                    None => crate::generator::make_image(0, options.image_size),
                };
                if options.image_nine_slice {
                    img.set_nine_slice_edges(24, 24, 24, 24);
                }
                img
            });
            if options.image_distinct {
                // 64 distinct sources cycled over the grid: enough that a per-source cache can't
                // serve everything from one entry, few enough that generating them is not the
                // measurement.
                let sources: Vec<slint::Image> = (1..=64)
                    .map(|seed| crate::generator::make_image(seed, options.image_size))
                    .collect();
                app.set_image_sources(slint::ModelRc::new(slint::VecModel::from(sources)));
            }
        }
        PerfMode::Rects => {
            app.set_rect_count(options.count);
            app.set_rect_gradient_kind(options.gradient_kind as i32);
            app.set_rect_extra_count(options.toggle_visibility);
            app.set_rect_opacity(options.item_opacity);
            app.set_rect_radius(options.item_radius);
            app.set_rect_border(options.item_border);
            app.set_rect_animate(options.animate);
            app.set_rect_animate_color(options.animate_color);
            app.set_rect_item_alpha(options.item_alpha);
            app.set_rect_use_path(options.use_path);
            app.set_rect_shadow(options.item_shadow);
            app.set_rect_animate_count(options.animate_count);
        }
    }
    let fill = fill_start.elapsed();
    let fill_allocations = alloc_stats::allocations() - fill_allocations;
    // `wire_up`'s `on_add_lines`/`on_add_records` handlers set `status` to a message that embeds
    // real wall-clock timing (`"generated in {build:.2?}"`), which differs every process run -
    // harmless for interactive use, but it sits inside the rendered frame at a fixed row near the
    // bottom (not excluded by any fixed row cutoff for every window size), so any `--bench` pixel
    // comparison sampled with enough repeats converges on ~3700px of "noise" there regardless of
    // scene/flags/frame count, which can mask a real, smaller divergence elsewhere. Blanking it
    // right after fill makes every `--bench` snapshot byte-identical across runs again.
    app.set_status(Default::default());

    #[cfg(feature = "alloc-profile")]
    if let Some(rate) = options.alloc_profile {
        match options.alloc_profile_phase {
            ProfilePhase::Fill => crate::alloc_profile::stop(),
            ProfilePhase::Show => crate::alloc_profile::start(rate),
            ProfilePhase::LastFrame => {}
        }
    }

    let show_start = Instant::now();
    let show_allocations = alloc_stats::allocations();
    app.show()?;
    let show = show_start.elapsed();
    let show_allocations = alloc_stats::allocations() - show_allocations;

    #[cfg(feature = "alloc-profile")]
    if options.alloc_profile.is_some() && options.alloc_profile_phase == ProfilePhase::Show {
        crate::alloc_profile::stop();
    }

    if options.key_input || options.select_all || options.type_text {
        // Key events go to the focused item, and nothing is focused at startup: click into the
        // text area first, or every key is dropped and the benchmark measures an idle window.
        let position = slint::LogicalPosition::new(options.width as f32 / 2.0, 300.0);
        for event in [
            slint::platform::WindowEvent::PointerPressed {
                position,
                button: slint::platform::PointerEventButton::Left,
            },
            slint::platform::WindowEvent::PointerReleased {
                position,
                button: slint::platform::PointerEventButton::Left,
            },
        ] {
            app.window().dispatch_event(event);
        }
    }

    if options.select_all {
        // Ctrl+A. The selection then stays for every frame that follows.
        for event in [
            slint::platform::WindowEvent::KeyPressed { text: slint::platform::Key::Control.into() },
            slint::platform::WindowEvent::KeyPressed { text: "a".into() },
            slint::platform::WindowEvent::KeyReleased { text: "a".into() },
            slint::platform::WindowEvent::KeyReleased {
                text: slint::platform::Key::Control.into(),
            },
        ] {
            app.window().dispatch_event(event);
        }
    }

    // Sized for the largest physical window any of the flags can produce: `--scale-toggle` doubles
    // the physical size when it raises the scale factor, and rendering into a buffer sized for the
    // smaller one panics inside the renderer.
    let max_scale = if options.scale_toggle { 2.0f32.max(options.scale_factor) } else { 1.0 };
    let buffer_pixels =
        ((options.width as f32 * max_scale) * (options.height as f32 * max_scale)) as usize;
    let mut buffer = vec![Rgb8Pixel::default(); buffer_pixels.max((options.width * options.height) as usize)];
    let mut stats = Vec::with_capacity(options.frames);
    // Shaping done before the first frame, i.e. by the layout pass that `show()` triggers.
    let mut shapes_before_first_frame = 0;
    let mut paragraph_shapes_before_first_frame = 0;
    let mut totals = ShapingTotals::default();

    for frame in 0..options.frames {
        #[cfg(feature = "alloc-profile")]
        if let Some(rate) = options.alloc_profile
            && frame + 1 == options.frames
            && options.alloc_profile_phase == ProfilePhase::LastFrame
        {
            crate::alloc_profile::start(rate);
        }

        // A frame is charged from here, before the properties that drive it are written: writing
        // them can trigger a re-layout that the render then only reads the results of. The shaping
        // counters are read as running totals rather than reset around `render()` for the same
        // reason - a reset inside the closure would charge that layout to nobody.
        #[cfg(feature = "dep-stats")]
        i_slint_core::properties::dep_stats::reset();
        let allocations = alloc_stats::allocations();
        let start = Instant::now();

        // Model churn, before the scroll: a real list UI changes its data while being scrolled,
        // and the two costs are separable only if both can be turned on independently.
        if options.mutate > 0 && records.row_count() > 0 {
            mutate_model(&records, options, frame);
        }

        // Scrolling is what the interesting workload looks like: the item tree is
        // traversed again while nothing about the model itself changed.
        app.set_scroll_y(-(frame as f32) * options.scroll_step);
        // Alternate so the window keeps changing width without drifting away from `--size`.
        // Frame 0 keeps the original width: the counters below only start covering the pre-render
        // layout from frame 1 on, so resizing on frame 0 would report work it did not count.
        let render_width = match options.resize_step {
            Some(step) if frame % 2 == 1 => options.width - step,
            _ => options.width,
        };
        if options.resize_step.is_some() {
            window.set_size(PhysicalSize::new(render_width, options.height));
        }
        if options.scale_toggle {
            window.dispatch_event(slint::platform::WindowEvent::ScaleFactorChanged {
                scale_factor: if frame % 2 == 0 { 1.0 } else { 2.0 },
            });
        }
        if options.toggle_visibility > 0 {
            app.set_rect_show_extra(frame % 2 == 0);
        }
        if options.toggle_global {
            app.global::<Settings>().set_alt_theme(frame % 2 == 0);
        }
        if options.wheel {
            // A wheel notch over the middle of the viewport. Goes to the Flickable, not to the
            // scroll property, so it exercises the real scrolling path including its animation.
            app.window().dispatch_event(slint::platform::WindowEvent::PointerScrolled {
                position: slint::LogicalPosition::new(
                    options.width as f32 / 2.0,
                    options.height as f32 / 2.0,
                ),
                delta_x: 0.0,
                delta_y: -120.0,
            });
        }
        if options.type_text {
            // One character. The text property changes, which invalidates the item's shaped-text
            // cache entry - the whole document has to be laid out again.
            app.window().dispatch_event(slint::platform::WindowEvent::KeyPressed {
                text: "x".into(),
            });
        }
        if options.key_input {
            // PageDown moves the cursor a screenful at a time, so the visible band moves and the
            // paragraphs under it have to be shaped - the worst case for lazy shaping.
            app.window().dispatch_event(slint::platform::WindowEvent::KeyPressed {
                text: slint::platform::Key::PageDown.into(),
            });
        }
        if options.pointer_move || options.pointer_jitter {
            // A small circle inside the viewport, so it stays over content and the position
            // differs every frame (a repeated position could be short-circuited).
            //
            // `--pointer-jitter` keeps the movement to a fraction of a row, so the set of items
            // under the pointer does not change: the hit-test traversal still runs in full, but no
            // exit/enter events fire. The difference between the two is what the stack churn costs.
            let angle = frame as f32 * 0.7;
            let radius = if options.pointer_jitter { 0.4 } else { 200.0 };
            window.dispatch_event(slint::platform::WindowEvent::PointerMoved {
                position: slint::LogicalPosition::new(
                    options.width as f32 / 2.0 + radius * angle.cos(),
                    options.height as f32 / 2.0 + radius * angle.sin(),
                ),
            });
        }
        slint::platform::update_timers_and_animations();
        window.request_redraw();

        // The renderer derives the physical size from the *logical* one times the scale factor,
        // and `window.size()` keeps reporting the size the window was set to - so it cannot be
        // used to size the render here.
        let window_size_for_render = if options.scale_toggle {
            let scale = if frame % 2 == 0 { 1.0 } else { 2.0 };
            PhysicalSize::new(
                (options.width as f32 * scale) as u32,
                (options.height as f32 * scale) as u32,
            )
        } else {
            PhysicalSize::new(render_width, options.height)
        };
        let mut frame_shaping = ShapingTotals::default();
        window.draw_if_needed(|renderer| {
            let cache = renderer.text_layout_cache();
            let read = || ShapingTotals {
                text_shapes: cache.cache_miss_count(),
                // STUB 2026-08-23: `paragraph_shape_count()` only exists on TextLayoutCache when
                // candidate 003.TEXT_DOCUMENT_SHAPING's own patch is applied (it added the method) -
                // a pristine tree has no such API, per MEASUREMENT.md section 12. Stubbed to 0 so
                // the harness builds against pristine slint; real value needs 003 applied and
                // rebased first. See ___OPTIMIZATIONS___/README.md.
                paragraph_shapes: 0,
                line_breaks: cache.layout_miss_count(),
            };
            if frame == 0 {
                // Everything shaped before the first render: `show()` and the layout it triggers.
                totals = read();
                shapes_before_first_frame = totals.text_shapes;
                paragraph_shapes_before_first_frame = totals.paragraph_shapes;
            }
            // The stride has to be the width actually being rendered, not the buffer's: with
            // `--resize-step` they differ, and passing the wrong one renders a sheared image (and
            // measures the wrong amount of work).
            // The window's own size is authoritative: a scale-factor change resizes it.
            let physical = window_size_for_render;
            let pixels = (physical.width * physical.height) as usize;
            if options.render_by_line {
                renderer.render_by_line(LineWriter {
                    buffer: &mut buffer[..pixels],
                    stride: physical.width as usize,
                });
            } else {
                renderer.render(&mut buffer[..pixels], physical.width as usize);
            }
            let after = read();
            frame_shaping = after.since(&totals);
            totals = after;
        });
        #[cfg(feature = "dep-stats")]
        {
            let dep = i_slint_core::properties::dep_stats::report();
            if frame + 1 == options.frames {
                let share = |value: u64| 100.0 * value as f64 / dep.registrations.max(1) as f64;
                println!(
                    "dependency registrations, last frame: {} total, {} distinct, {} duplicates ({:.1}%) - of those a 1-entry cache would catch {} ({:.1}%), a 4-entry cache {} ({:.1}%)",
                    dep.registrations,
                    dep.distinct,
                    dep.duplicates,
                    share(dep.duplicates),
                    dep.consecutive,
                    share(dep.consecutive),
                    dep.in_ring,
                    share(dep.in_ring)
                );
                println!(
                    "  of which actually skipped by the front-of-list check: {} ({:.1}%)",
                    dep.skipped,
                    share(dep.skipped)
                );
            }
        }
        stats.push(FrameStats {
            duration: start.elapsed(),
            allocations: alloc_stats::allocations() - allocations,
            text_shapes: frame_shaping.text_shapes,
            paragraph_shapes: frame_shaping.paragraph_shapes,
            line_breaks: frame_shaping.line_breaks,
        });
    }

    #[cfg(feature = "alloc-profile")]
    if options.alloc_profile.is_some() {
        crate::alloc_profile::stop();
    }

    if let Some(path) = &options.snapshot {
        let mut ppm = format!("P6\n{} {}\n255\n", options.width, options.height).into_bytes();
        ppm.extend(buffer.iter().flat_map(|pixel| [pixel.r, pixel.g, pixel.b]));
        match std::fs::write(path, ppm) {
            Ok(()) => println!("snapshot written to {path}"),
            Err(error) => eprintln!("no snapshot: {error}"),
        }
    }

    report(
        options,
        Phase { duration: fill, allocations: fill_allocations },
        Phase { duration: show, allocations: show_allocations },
        shapes_before_first_frame,
        paragraph_shapes_before_first_frame,
        &stats,
    );

    #[cfg(feature = "alloc-profile")]
    if options.alloc_profile.is_some() {
        crate::alloc_profile::report_by_site(20);
        crate::alloc_profile::report(8, 8);
    }

    Ok(())
}

fn report(
    options: &BenchOptions,
    fill: Phase,
    show: Phase,
    shapes_before_first_frame: u64,
    paragraph_shapes_before_first_frame: u64,
    stats: &[FrameStats],
) {
    let mode = match options.mode {
        PerfMode::Text => "text",
        PerfMode::List => "list",
        PerfMode::Scroll => "scroll",
        PerfMode::Rects => "rects",
        PerfMode::Images => "images",
        PerfMode::Nest => "nest",
        PerfMode::Layouts => "layouts",
    };
    let unit = match options.mode {
        PerfMode::Text => "lines",
        PerfMode::Rects | PerfMode::Images | PerfMode::Nest => "rectangles",
        _ => "records",
    };

    println!("mode           {mode}");
    println!("content        {} {unit}", options.count);
    println!("window         {}x{} (headless software renderer)", options.width, options.height);
    println!("scroll step    {} px per frame", options.scroll_step);
    if options.scale_factor != 1.0 {
        println!("scale factor   {}", options.scale_factor);
    }
    println!(
        "rows           {}",
        match options.row_style {
            RowStyle::Layout => "layout inside the row",
            RowStyle::Flat => "flat (children positioned explicitly)",
            RowStyle::Clipped => "layout inside the row + clip: true",
            RowStyle::Touch => "layout inside the row + a TouchArea",
            RowStyle::Bare => "bare (no children at all)",
            RowStyle::LayoutFixed => "layout inside the row, fixed widths, no elide",
            RowStyle::VerticalFixed => "VerticalLayout of 3 fixed-width, no-wrap texts",
            RowStyle::VerticalFixedInput => "VerticalLayout of 3 fixed-width, no-wrap TextInputs",
        }
    );
    if options.mode == PerfMode::Text {
        println!("wrap           {}", if options.word_wrap { "word-wrap" } else { "no-wrap" });
    }
    if let Some(step) = options.resize_step {
        println!("resize         width alternating by {step} px per frame");
    }
    if options.mode == PerfMode::Images {
        println!(
            "images         {} sources, {}px each, {}{}",
            if options.image_distinct { "64 distinct" } else { "1 shared" },
            options.image_size,
            if options.image_rescale { "rescaled" } else { "natural size" },
            if options.image_tiling { ", repeat-tiled with a source crop" } else { "" },
        );
        if options.image_nine_slice {
            println!("               nine-slice source (24px borders)");
        }
    }
    if options.mode == PerfMode::Nest {
        println!(
            "nesting        {} levels of {}",
            options.nest_depth,
            if options.nest_plain { "plain rectangles" } else { "layouts" }
        );
    }
    if options.toggle_global {
        println!("global         one global property flipped per frame (every row reads it)");
    }
    if options.scale_toggle {
        println!("scale          alternating 1.0 / 2.0 every frame");
    }
    if options.toggle_visibility > 0 {
        println!("visibility     a subtree of {} items toggled per frame", options.toggle_visibility);
    }
    if options.model_adapter != ModelAdapter::None {
        println!("model          wrapped in a {:?} adapter", options.model_adapter);
    }
    if options.font_size_variants > 1 {
        println!("font sizes     {} distinct sizes cycled over the rows", options.font_size_variants);
    }
    if options.mode == PerfMode::Text {
        println!("line length    {} characters", options.line_length);
    }
    if let Some(chars) = options.long_line_chars {
        println!(
            "long line      one extra {chars}-char line prepended, never edited by --type-text/--key-input"
        );
    }
    if options.wheel {
        println!("wheel          one wheel notch per frame (through Flickable)");
    }
    if options.type_text {
        println!("typing         one character typed per frame");
    }
    if options.select_all {
        println!("selection      whole document selected before the first frame");
    }
    if options.key_input {
        println!("keys           one PageDown per frame");
    }
    if options.pointer_move {
        println!("pointer        one PointerMoved event per frame");
    }
    if options.animate {
        println!(
            "animate        every rectangle binds to animation-tick() - time-driven, so this \
scene is NOT pixel-comparable"
        );
    }
    if options.mutate > 0 {
        println!("mutate         {} rows/frame, {}", options.mutate, options.mutate_mode.label());
    }
    println!("fill           {:.2?} ({} allocations)", fill.duration, fill.allocations);
    println!(
        "show + layout  {:.2?} ({} allocations, {shapes_before_first_frame} text shapes, \
{paragraph_shapes_before_first_frame} paragraph shapes)",
        show.duration, show.allocations
    );
    println!();
    println!(
        "{:>5}  {:>12}  {:>14}  {:>12}  {:>17}  {:>12}",
        "frame", "time", "allocations", "text shapes", "paragraph shapes", "line breaks"
    );
    for (index, frame) in stats.iter().enumerate() {
        println!(
            "{index:>5}  {:>12}  {:>14}  {:>12}  {:>17}  {:>12}",
            format!("{:.2?}", frame.duration),
            frame.allocations,
            frame.text_shapes,
            frame.paragraph_shapes,
            frame.line_breaks
        );
    }

    if let Some(resident) = alloc_stats::resident_bytes() {
        println!();
        println!(
            "resident memory {:.1} MB after the last frame",
            resident as f64 / (1024.0 * 1024.0)
        );
    }

    if let Some(sched) = alloc_stats::scheduling_stats() {
        println!(
            "scheduling     ctxt switches {} voluntary / {} involuntary, page faults {} minor / {} major",
            sched.voluntary_ctxt_switches,
            sched.nonvoluntary_ctxt_switches,
            sched.minor_faults,
            sched.major_faults
        );
    }

    if let Some((first, rest)) = stats.split_first() {
        println!();
        println!("first frame    {:.2?} ({} allocations)", first.duration, first.allocations);
        if !rest.is_empty() {
            // Median, like the windowed report: one outlying frame should be visible in the spread
            // line, not averaged into the headline. Headless the two agree to a rounding error,
            // which is exactly why the windowed one needs the median.
            let time = crate::median(rest.iter().map(|frame| frame.duration));
            let allocations = crate::median(rest.iter().map(|frame| frame.allocations));
            let shapes = crate::median(rest.iter().map(|frame| frame.paragraph_shapes));
            // Same wording as the windowed report, so one parser reads either.
            println!(
                "later frames   {time:.2?} median, {:.1} fps ({allocations} allocations, \
{shapes} paragraph shapes per frame)",
                1.0 / time.as_secs_f64(),
            );
            let slowest = rest.iter().map(|frame| frame.duration).max().unwrap_or_default();
            let fastest = rest.iter().map(|frame| frame.duration).min().unwrap_or_default();
            let most = rest.iter().map(|frame| frame.allocations).max().unwrap_or_default();
            let least = rest.iter().map(|frame| frame.allocations).min().unwrap_or_default();
            println!(
                "               spread {fastest:.2?}..{slowest:.2?} over {} frames, \
allocations {least}..{most}",
                rest.len()
            );
        }
    }
}
