use indicatif::MultiProgress;
use indicatif::ProgressBar;
use indicatif::ProgressDrawTarget;

fn main() {
    let multi_progress = MultiProgress::default();
    let _ = multi_progress.insert_after(&ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(0u8)), ProgressBar::new_spinner());
}
