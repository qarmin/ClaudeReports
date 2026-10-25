use zlib_rs::inflate::Window;

fn main() {
    let window = Window::empty();
    let _ = window.buffer_size();
}
