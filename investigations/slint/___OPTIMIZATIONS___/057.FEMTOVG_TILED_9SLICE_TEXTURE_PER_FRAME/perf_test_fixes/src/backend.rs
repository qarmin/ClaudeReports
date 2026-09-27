use slint::PlatformError;

use crate::bench::{BenchOptions, ModelAdapter, MutateMode, ProfilePhase};
use crate::frames::FrameRunOptions;
use crate::{PerfMode, RowStyle};

/// Renderers compiled into this binary, in the order Slint would prefer them.
pub fn compiled_renderers() -> Vec<&'static str> {
    let mut renderers = Vec::new();
    if cfg!(feature = "femtovg") {
        renderers.push("femtovg");
    }
    if cfg!(any(feature = "skia", feature = "skia-opengl", feature = "skia-vulkan")) {
        renderers.push("skia");
    }
    if cfg!(feature = "software") {
        renderers.push("software");
    }
    renderers
}

pub fn compiled_backends() -> Vec<&'static str> {
    let mut backends = vec!["winit"];
    if cfg!(feature = "qt") {
        backends.push("qt");
    }
    backends
}

pub struct Options {
    backend: Option<String>,
    renderer: Option<String>,
    /// Content to create at startup, also used by the benchmark.
    pub mode: Option<PerfMode>,
    pub count: i32,
    /// `Some` for the headless benchmark, `None` for the interactive window.
    pub bench: Option<BenchOptions>,
    /// `Some` to render that many frames in the window and then quit.
    pub frame_run: Option<FrameRunOptions>,
    pub flat_rows: bool,
    pub row_style: RowStyle,
    pub gradient: bool,
    pub word_wrap: bool,
    pub resize_step: Option<u32>,
    pub image_distinct: bool,
    pub image_rescale: bool,
    pub image_tiling: bool,
    pub image_size: u32,
    pub image_file: Option<String>,
    pub image_cell: f32,
    pub nest_depth: i32,
    pub nest_plain: bool,
    pub layout_children: i32,
    pub layout_kind: i32,
    pub layout_stretch: bool,
    pub toggle_global: bool,
    pub scale_factor: f32,
    pub scale_toggle: bool,
    pub toggle_visibility: i32,
    pub model_adapter: ModelAdapter,
    pub item_opacity: f32,
    pub item_radius: f32,
    pub item_border: f32,
    pub font_size_variants: i32,
    pub line_length: usize,
    pub wheel: bool,
    pub type_text: bool,
    pub select_all: bool,
    pub key_input: bool,
    pub pointer_move: bool,
    pub pointer_jitter: bool,
    pub animate: bool,
    pub animate_color: bool,
    pub repaint_buffer: slint::platform::software_renderer::RepaintBufferType,
    pub render_by_line: bool,
    pub font_size_base: f32,
    pub item_alpha: f32,
    pub use_path: bool,
    pub animate_count: i32,
    pub mutate: usize,
    pub mutate_mode: MutateMode,
}

pub fn parse_args() -> Result<Options, String> {
    let defaults = BenchOptions::default();
    let mut options = Options {
        backend: None,
        renderer: None,
        mode: None,
        count: 0,
        bench: None,
        frame_run: None,
        flat_rows: false,
        row_style: RowStyle::Layout,
        gradient: false,
        word_wrap: false,
        resize_step: None,
        image_distinct: false,
        image_rescale: false,
        image_tiling: false,
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
        line_length: 0,
        wheel: false,
        type_text: false,
        select_all: false,
        key_input: false,
        pointer_move: false,
        pointer_jitter: false,
        animate: false,
        animate_color: false,
        repaint_buffer: slint::platform::software_renderer::RepaintBufferType::ReusedBuffer,
        render_by_line: false,
        font_size_base: 13.0,
        item_alpha: -1.0,
        use_path: false,
        animate_count: i32::MAX,
        mutate: 0,
        mutate_mode: MutateMode::Changed,
    };
    let mut bench = false;
    let mut frames = None;
    let mut scroll_step = defaults.scroll_step;
    let mut size = (defaults.width, defaults.height);
    let mut alloc_profile = None;
    let mut alloc_profile_phase = ProfilePhase::LastFrame;
    let mut snapshot = None;
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        let mut value = || args.next().ok_or(format!("`{arg}` needs a value"));
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            "--renderer" | "-r" => options.renderer = Some(value()?),
            "--backend" | "-b" => options.backend = Some(value()?),
            "--mode" | "-m" => options.mode = Some(parse_mode(&value()?)?),
            "--count" | "-c" => options.count = parse_number(&value()?)?,
            "--bench" => bench = true,
            "--flat-rows" => {
                options.flat_rows = true;
                options.row_style = RowStyle::Flat;
            }
            "--row-style" => {
                options.row_style = match value()?.as_str() {
                    "layout" => RowStyle::Layout,
                    "flat" => RowStyle::Flat,
                    "clipped" => RowStyle::Clipped,
                    "touch" => RowStyle::Touch,
                    "bare" => RowStyle::Bare,
                    "layout-fixed" => RowStyle::LayoutFixed,
                    other => {
                        return Err(format!(
                            "unknown row style `{other}`, expected layout, layout-fixed, flat, clipped, touch or bare"
                        ));
                    }
                };
                options.flat_rows = matches!(options.row_style, RowStyle::Flat);
            }
            "--gradient" => options.gradient = true,
            "--word-wrap" => options.word_wrap = true,
            "--resize-step" => options.resize_step = Some(value()?.parse().map_err(|_| "--resize-step wants a number")?),
            "--image-distinct" => options.image_distinct = true,
            "--image-rescale" => options.image_rescale = true,
            "--image-tiling" => options.image_tiling = true,
            "--image-size" => options.image_size = value()?.parse().map_err(|_| "--image-size wants a number")?,
            "--nest-depth" => options.nest_depth = value()?.parse().map_err(|_| "--nest-depth wants a number")?,
            "--nest-plain" => options.nest_plain = true,
            "--layout-children" => options.layout_children = value()?.parse().map_err(|_| "--layout-children wants a number")?,
            "--layout-stretch" => options.layout_stretch = true,
            "--path" => options.use_path = true,
            "--layout-kind" => {
                options.layout_kind = match value()?.as_str() {
                    "h" | "horizontal" => 0,
                    "v" | "vertical" => 1,
                    "grid" => 2,
                    "none" => 3,
                    other => return Err(format!("--layout-kind: expected h, v, grid or none, got `{other}`")),
                };
            }
            "--toggle-global" => options.toggle_global = true,
            "--radius" => options.item_radius = value()?.parse().map_err(|_| "--radius wants a number")?,
            "--border" => options.item_border = value()?.parse().map_err(|_| "--border wants a number")?,
            "--scale-toggle" => options.scale_toggle = true,
            "--scale-factor" => options.scale_factor = value()?.parse().map_err(|_| "--scale-factor wants a number")?,
            "--toggle-visibility" => options.toggle_visibility = value()?.parse().map_err(|_| "--toggle-visibility wants a number")?,
            "--model-adapter" => {
                options.model_adapter = match value()?.as_str() {
                    "none" => ModelAdapter::None,
                    "filter" => ModelAdapter::Filter,
                    "sort" => ModelAdapter::Sort,
                    "map" => ModelAdapter::Map,
                    other => return Err(format!("unknown --model-adapter `{other}`")),
                }
            }
            "--opacity" => options.item_opacity = value()?.parse().map_err(|_| "--opacity wants a number")?,
            "--font-sizes" => options.font_size_variants = value()?.parse().map_err(|_| "--font-sizes wants a number")?,
            "--line-length" => options.line_length = value()?.parse().map_err(|_| "--line-length wants a number")?,
            "--wheel" => options.wheel = true,
            "--type-text" => options.type_text = true,
            "--select-all" => options.select_all = true,
            "--key-input" => options.key_input = true,
            "--pointer-move" => options.pointer_move = true,
            "--pointer-jitter" => options.pointer_jitter = true,
            "--animate" => options.animate = true,
            "--render-by-line" => options.render_by_line = true,
            "--image-file" => options.image_file = Some(value()?),
            "--image-cell" => options.image_cell = value()?.parse().map_err(|_| "--image-cell wants a number")?,
            "--font-size" => options.font_size_base = value()?.parse().map_err(|_| "--font-size wants a number")?,
            "--repaint-buffer" => {
                use slint::platform::software_renderer::RepaintBufferType::*;
                options.repaint_buffer = match value()?.as_str() {
                    "new" => NewBuffer,
                    "reused" => ReusedBuffer,
                    "swapped" => SwappedBuffers,
                    other => return Err(format!("--repaint-buffer: unknown mode `{other}`")),
                };
            }
            "--item-alpha" => options.item_alpha = value()?.parse().map_err(|_| "--item-alpha wants a number")?,
            "--animate-color" => {
                options.animate = true;
                options.animate_color = true;
            }
            "--animate-count" => options.animate_count = value()?.parse().map_err(|_| "--animate-count wants a number")?,
            "--mutate" => options.mutate = value()?.parse().map_err(|_| "--mutate wants a number")?,
            "--mutate-mode" => {
                options.mutate_mode = match value()?.as_str() {
                    "changed" => MutateMode::Changed,
                    "changed-offscreen" => MutateMode::ChangedOffscreen,
                    "push" => MutateMode::Push,
                    "insert-front" => MutateMode::InsertFront,
                    "rotate" => MutateMode::Rotate,
                    other => return Err(format!("unknown --mutate-mode `{other}`")),
                }
            }
            "--frames" | "-f" => frames = Some(parse_number(&value()?)? as usize),
            "--scroll-step" => {
                let raw = value()?;
                scroll_step = raw.parse().map_err(|_| format!("`{raw}` is not a number"))?;
            }
            "--size" => size = parse_size(&value()?)?,
            "--snapshot" => snapshot = Some(value()?),
            "--alloc-profile" => alloc_profile = Some(parse_number(&value()?)?.max(1) as u64),
            "--profile-phase" => {
                alloc_profile_phase = match value()?.as_str() {
                    "fill" => ProfilePhase::Fill,
                    "show" => ProfilePhase::Show,
                    "frame" | "frames" => ProfilePhase::LastFrame,
                    other => return Err(format!("unknown phase `{other}`")),
                }
            }
            other => return Err(format!("unknown argument `{other}`, try --help")),
        }
    }

    if bench {
        options.bench = Some(BenchOptions {
            mode: options.mode.unwrap_or(defaults.mode),
            count: if options.count > 0 { options.count } else { defaults.count },
            frames: frames.unwrap_or(defaults.frames),
            scroll_step,
            width: size.0,
            height: size.1,
            flat_rows: options.flat_rows,
            row_style: options.row_style,
            gradient: options.gradient,
            alloc_profile,
            alloc_profile_phase,
            word_wrap: options.word_wrap,
            resize_step: options.resize_step,
            image_distinct: options.image_distinct,
            image_rescale: options.image_rescale,
            image_tiling: options.image_tiling,
            image_size: options.image_size,
            image_file: options.image_file.clone(),
            image_cell: options.image_cell,
            nest_depth: options.nest_depth,
            nest_plain: options.nest_plain,
            layout_children: options.layout_children,
            layout_kind: options.layout_kind,
            layout_stretch: options.layout_stretch,
            toggle_global: options.toggle_global,
            scale_factor: options.scale_factor,
            scale_toggle: options.scale_toggle,
            toggle_visibility: options.toggle_visibility,
            model_adapter: options.model_adapter,
            item_opacity: options.item_opacity,
            item_radius: options.item_radius,
            item_border: options.item_border,
            font_size_variants: options.font_size_variants,
            line_length: if options.line_length > 0 { options.line_length } else { defaults.line_length },
            wheel: options.wheel,
            type_text: options.type_text,
            select_all: options.select_all,
            key_input: options.key_input,
            pointer_move: options.pointer_move,
            pointer_jitter: options.pointer_jitter,
            animate: options.animate,
            animate_color: options.animate_color,
            repaint_buffer: options.repaint_buffer,
            render_by_line: options.render_by_line,
            font_size_base: options.font_size_base,
            item_alpha: options.item_alpha,
            use_path: options.use_path,
            animate_count: options.animate_count,
            mutate: options.mutate,
            mutate_mode: options.mutate_mode,
            snapshot,
        });
    } else if let Some(frames) = frames {
        options.frame_run =
            Some(FrameRunOptions { frames, size, scroll_step, alloc_profile, snapshot });
    }

    Ok(options)
}

fn parse_mode(value: &str) -> Result<PerfMode, String> {
    match value {
        "text" => Ok(PerfMode::Text),
        "list" => Ok(PerfMode::List),
        "scroll" => Ok(PerfMode::Scroll),
        "rects" => Ok(PerfMode::Rects),
        "images" => Ok(PerfMode::Images),
        "nest" => Ok(PerfMode::Nest),
        "layouts" => Ok(PerfMode::Layouts),
        other => Err(format!("unknown mode `{other}`, expected text, list, scroll, rects, images, nest or layouts")),
    }
}

fn parse_number(value: &str) -> Result<i32, String> {
    value.replace('_', "").parse().map_err(|_| format!("`{value}` is not a number"))
}

fn parse_size(value: &str) -> Result<(u32, u32), String> {
    let (width, height) = value
        .split_once(['x', 'X'])
        .ok_or_else(|| format!("`{value}` is not a size, expected e.g. 1280x860"))?;
    Ok((parse_number(width)? as u32, parse_number(height)? as u32))
}

fn print_help() {
    println!(
        "Slint perf test\n\n\
         Usage: perf_test [options]\n\n\
         Window and renderer:\n\
         \x20 -b, --backend <name>   winit (default) or qt\n\
         \x20 -r, --renderer <name>  femtovg (default), skia or software\n\n\
         Content created at startup, in both the window and the benchmark:\n\
         \x20 -m, --mode <name>      text, list, scroll or rects\n\
         \x20 -c, --count <n>        lines (text mode) or records to add\n\n\
         \x20     --flat-rows        rows without an inner layout (see FINDINGS.md)\n\
         \x20     --row-style <s>    scroll mode row: layout | flat | clipped | bare\n\
         \x20     --gradient         fill the rects scene with a gradient\n\n\
         Benchmarks, both print a per frame table and exit:\n\
         \x20     --bench            headless, software renderer, no window\n\
         \x20 -f, --frames <n>       frames to render (default 5 with --bench)\n\
         \x20     --scroll-step <px> how far to scroll between frames (default 200)\n\
         \x20     --size <WxH>       render target size (default 1280x860)\n\
         \x20     --alloc-profile <n> with --bench: sample 1 in n allocations of the last\n\
         \x20                        frame and print which code allocates\n\
         \x20     --profile-phase <p> which phase to profile: fill, show or frames\n\
         \x20     --snapshot <file>  write a PPM of the last frame (--bench or --frames)\n\
         \x20     --word-wrap        text mode: wrap: word-wrap instead of no-wrap\n\
         \x20     --resize-step <px> with --bench: resize the window each frame instead of scrolling\n\
         \x20     --pointer-move     with --bench: dispatch a PointerMoved event each frame\n\
         \x20     --animate          rects: bind every rectangle to animation-tick()\n\
         \x20     --animate-color    same, but animate the fill instead of the position\n\
         \x20     --item-alpha <a>   rects: explicit fill alpha 0..1 instead of the palette\n\
         \x20     --repaint-buffer <new|reused|swapped>  which buffer the renderer assumes (default reused)\n\
         \x20     --font-size <px>   base font size, independent of --font-sizes variety\n\
         \x20     --image-file <n>   images: load assets/<n> instead of generating the source\n\
         \x20     --image-cell <px>  images: drawn size of each item\n\
         \x20     --render-by-line   use render_by_line (the Scene path) instead of render\n\
         \x20     --animate-count <n> rects: only the first n rectangles animate\n\
         \x20     --mutate <n>       with --bench: change n model rows per frame\n\
         \x20     --mutate-mode <m>  changed|changed-offscreen|push|insert-front|rotate\n\n\
         Backends compiled in: {}\n\
         Renderers compiled in: {}\n\n\
         Renderers other than the default need their cargo feature (--features skia).\n\
         SLINT_BACKEND=winit-software and friends work as well.",
        compiled_backends().join(", "),
        compiled_renderers().join(", "),
    );
}

/// Applies the requested backend/renderer and returns a description of it for the UI.
/// Without any request, Slint picks the default itself (femtovg for the winit backend).
pub fn select(options: &Options) -> Result<String, PlatformError> {
    if options.backend.is_none() && options.renderer.is_none() {
        return Ok(match std::env::var("SLINT_BACKEND") {
            Ok(from_env) if !from_env.is_empty() => format!("{from_env} (SLINT_BACKEND)"),
            _ => format!("default ({})", compiled_renderers().first().unwrap_or(&"none")),
        });
    }

    let mut selector = slint::BackendSelector::new();
    let mut description = String::new();

    if let Some(backend) = &options.backend {
        selector = selector.backend_name(backend.clone());
        description.push_str(backend);
    }
    // The Qt backend brings its own renderer, so a renderer request would not apply.
    if let Some(renderer) = &options.renderer {
        selector = selector.renderer_name(renderer.clone());
        if !description.is_empty() {
            description.push('-');
        }
        description.push_str(renderer);
    }

    selector.select()?;
    Ok(description)
}
