use indicatif::MultiProgress;
use indicatif::ProgressBar;
use indicatif::ProgressDrawTarget;

fn main() {
    let _ = MultiProgress::with_draw_target(ProgressDrawTarget::stdout_with_hz(0u8)).insert_before(&ProgressBar::hidden(), ProgressBar::no_length());
}
