use krilla::document::Document;

fn main() {
    let mut document = Document::new();
    let mut page = document.start_page();
    let mut surface = page.surface();
    surface.push_isolated();
}
