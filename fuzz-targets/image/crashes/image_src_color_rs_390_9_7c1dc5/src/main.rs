fn main() {
    let vec = vec![];
    let _ = <image::Rgba<u8> as image::Pixel>::from_slice(&vec);
}
