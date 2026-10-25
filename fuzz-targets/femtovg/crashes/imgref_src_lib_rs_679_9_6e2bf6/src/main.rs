use femtovg::renderer::Void;

fn main() {
    let mut void = Void;
    let _ = <femtovg::renderer::Void as femtovg::Renderer>::screenshot(&mut void);
}
