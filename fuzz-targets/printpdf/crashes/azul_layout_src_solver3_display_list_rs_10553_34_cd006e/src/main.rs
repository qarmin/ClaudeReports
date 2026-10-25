use printpdf::html::XmlRenderOptions;

fn main() {
    let _ = printpdf::html::xml_to_pdf_pages("<svg xs=\"vg\" width=\"1e9\" height=\"1\" x=\"0 \"><path d=\"M1 7Z\" s=\"d\"/></svg>", &XmlRenderOptions::default());
}
