use indicatif::MultiProgress;
use indicatif::ProgressBar;
use indicatif::ProgressDrawTarget;

fn main() {
    let progress_draw_target = ProgressDrawTarget::stdout();
    let multi_progress = MultiProgress::with_draw_target(ProgressDrawTarget::stdout_with_hz(0u8));
    multi_progress.remove(&MultiProgress::default().insert(1usize, ProgressBar::with_draw_target(None, progress_draw_target)));
}
