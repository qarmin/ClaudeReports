use printpdf::PdfDocument;
use printpdf::deserialize::PdfParseOptions;
use printpdf::html::XmlRenderOptions;

fn main() {
    let mut pdf_document = PdfDocument::parse(&include_bytes!("data_0.bin").to_vec(), &PdfParseOptions::default(), &mut vec![]).unwrap();
    let _ = printpdf::html::add_xml_to_document(&mut pdf_document, "<svg xs=\"vg\" width=\"1e9\" height=\"55\" vx=\"0N\"><circle cx=\"1\" r=\"0.6\"/></svg>", &XmlRenderOptions::default());
}
