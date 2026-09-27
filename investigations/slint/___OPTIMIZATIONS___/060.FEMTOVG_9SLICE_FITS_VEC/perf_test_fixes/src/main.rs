#[cfg(feature = "alloc-profile")]
mod alloc_profile;
mod alloc_stats;
mod backend;
mod bench;
mod frames;
mod generator;
mod logic;

use slint::ComponentHandle;

use crate::logic::wire_up;

slint::include_modules!();

/// The middle value of `values`, which must not be empty.
///
/// Frame statistics are reported as medians rather than means: on a real compositor a single frame
/// can render before the window is fully mapped, or stall on it, and one such frame out of ten is
/// enough to invent a regression that isn't there. The reports print the spread alongside, so an
/// outlier is visible instead of averaged in.
fn median<T: Ord + Copy>(values: impl IntoIterator<Item = T>) -> T {
    let mut values: Vec<T> = values.into_iter().collect();
    values.sort_unstable();
    values[values.len() / 2]
}

fn main() -> Result<(), slint::PlatformError> {
    let options = match backend::parse_args() {
        Ok(options) => options,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2);
        }
    };

    if let Some(bench_options) = &options.bench {
        return bench::run(bench_options);
    }

    let backend_description = backend::select(&options)?;

    let app = MainWindow::new()?;
    app.set_backend_info(backend_description.into());
    wire_up(&app);
    app.global::<Settings>().set_flat_rows(options.flat_rows);
    app.global::<Settings>().set_row_style(options.row_style);
    app.set_word_wrap(options.word_wrap);

    if let Some(mode) = options.mode {
        app.set_mode(mode);
        if options.count > 0 {
            match mode {
                PerfMode::Text => app.global::<Logic>().invoke_add_lines(options.count),
                PerfMode::List | PerfMode::Scroll => {
                    app.global::<Logic>().invoke_add_records(options.count)
                }
                PerfMode::Nest | PerfMode::Layouts => {
                    app.set_rect_count(options.count);
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
                            None if options.image_nine_slice => {
                                crate::generator::load_asset("nine_slice.png")
                            }
                            None => crate::generator::make_image(0, options.image_size),
                        };
                        if options.image_nine_slice {
                            img.set_nine_slice_edges(24, 24, 24, 24);
                        }
                        img
                    });
                    if options.image_distinct {
                        // Mirrors `bench::run`'s `PerfMode::Images` fill: 64 distinct sources
                        // cycled over the grid, so the windowed (`--frames`) path exercises the
                        // same "does the renderer cache per source" question the headless
                        // (`--bench`) path already did - this branch used to be missing here
                        // entirely, silently making `--image-distinct` a no-op under `--frames`.
                        let sources: Vec<slint::Image> = (1..=64)
                            .map(|seed| crate::generator::make_image(seed, options.image_size))
                            .collect();
                        app.set_image_sources(slint::ModelRc::new(slint::VecModel::from(sources)));
                    }
                }
                PerfMode::Rects => {
                    app.set_rect_count(options.count);
                    app.set_rect_gradient(options.gradient);
                }
            }
        }
    }

    if let Some(frame_run) = &options.frame_run {
        frames::install(&app, frame_run)?;
    }

    app.run()
}
