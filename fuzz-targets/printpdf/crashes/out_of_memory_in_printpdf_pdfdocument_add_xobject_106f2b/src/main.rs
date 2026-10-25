use printpdf::PdfDocument;
use printpdf::svg::Svg;

fn main() {
    let mut pdf_document = PdfDocument::new("");
    let _ = pdf_document.add_xobject(&Svg::parse("<svg xmlns=\"http://www.w3.org/2000/svg\" x=\"0\"><defs>><filter id=\"f\"><feGaussianBlur /></filter></defs><g mask=\"ly\" filter=\"url(#f)\"><g mask=\"urlf)\"><path d=\"M82 1e9 L131 2 d\"/><text>He😀</text></g></g></svg>", &mut vec![]).unwrap());
}
