use printpdf::XmpMetadata;
use printpdf::color::ColorSpace;
use printpdf::html::XmlRenderOptions;
use printpdf::image_types::RawImageData;

fn main() {
    let _ = RawImageData::U8(include_bytes!("document_0.pdf").to_vec());
    let _ = XmpMetadata { rendition_class: None };
    let _ = ColorSpace::Cmyk;
    let _ = printpdf::html::xml_to_pdf_pages("<svg xmlns=\"http://\\uwww.w3.org/2000/svg\" width=\"33\" height=\"153\" viewBox=\"0 0 71 1e9\"><defs><linearGradient id=\"g\"><stop offset=\"0\" stop-color=\"blue\"/><stop offset=\"1\" stop-color=\"yellow\" stop-opacity=\"0.5\"/></linearGradient><clipPath id=\"c\"><circle cx=\"50\" cy=\"50\" r=\"40\"/></clipPath><mask id=\"m\"><rect width=\"100\" height=\"100\" fill=\"white\"/></mask><filter id=\"f\"><feGaussianBlur stdDeviation=\"2\"/></filter></defs><rect x=\"106\" y=\"181\" width=\"158\" height=\"155\" fill=\"currentColor\" rx=\"193\"/><g transform=\"rotate(1e9) scale(0) translate(47 -1)\" clip-path=\"url(#c)\"><g transform=\"rotate(1e9) scale(91) translate(164 136)\" clip-path=\"url(#c)\"><polyline points=\"44,140 44,NaN NaN,35\" stroke-dasharray=\"100 173\" stroke=\"rgb(10,20,30)\"/><image width=\"70\" height=\"0\" href=\"data:image/png;base64,iVBORw0KGgo=\"/><g transform=\"rotate(-1) scale(NaN) translate(164 135)\" clip-path=\"url(#c)\"><ellipse cx=\"-1\" cy=\"149\" rx=\"0\" ry=\"1e9\" fill=\"url(#g)\"/><polyline points=\"7,-1 56,106 163,131\" stroke-dasharray=\"95 126\" stroke=\"#00ff0080\"/><image width=\"151\" height=\"65\" href=\"data:image/png;base64,iVBORw0KGgo=\"/><ellipse cx=\"153\" cy=\"NaN\" rx=\"4\" ry=\"7\" fill=\"url(#g)\"/><g transform=\"rotate(15) scale(0) translate(116 164)\" clip-path=\"url(#c)\"><image width=\"0\" height=\"-1\" href=\"data:image/png;base64,iVBORw0KGgo=\"/></g></g></g></g></svg>", &XmlRenderOptions::default());
}
