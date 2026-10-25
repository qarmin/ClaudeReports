use realfft::FftError;

fn main() {
    let fft_error = FftError::InputValues(false, false);
    let _ = fft_error.to_string();
}
