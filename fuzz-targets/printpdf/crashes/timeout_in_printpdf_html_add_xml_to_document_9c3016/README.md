# timeout in printpdf::html::add_xml_to_document

- crate: printpdf 0.12.8
- kind: timeout
- last API call: `printpdf::html::add_xml_to_document`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling printpdf::html::add_xml_to_document
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
printpdf = { version = "=0.12.8", features = ["bmp", "dds", "gif", "hdr", "html", "html_multithreaded", "ico", "images", "jpeg", "js-sys", "png", "pnm", "rayon", "svg", "text_layout", "text_layout_hyphenation", "tga", "tiff", "wasm-bindgen-futures", "web-sys", "webp"] }
```

`src/main.rs`:

```rust
use printpdf::ExtendedGraphicsStateMap;
use printpdf::PageAnnotMap;
use printpdf::PdfDocument;
use printpdf::PdfDocumentInfo;
use printpdf::PdfFontMap;
use printpdf::PdfLayerMap;
use printpdf::PdfMetadata;
use printpdf::PdfResources;
use printpdf::ShadingMap;
use printpdf::XObjectMap;
use printpdf::html::XmlRenderOptions;

fn main() {
    let mut pdf_document = PdfDocument { metadata: PdfMetadata { info: PdfDocumentInfo::default(), xmp: None }, resources: PdfResources { fonts: PdfFontMap::default(), xobjects: XObjectMap::default(), extgstates: ExtendedGraphicsStateMap::default(), shadings: ShadingMap::default(), layers: PdfLayerMap::default() }, bookmarks: PageAnnotMap::default(), pages: vec![] };
    let _ = printpdf::html::add_xml_to_document(&mut pdf_document, "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"23\" height=\"191\" viewBox=\"0 0 105 0\"><defs><linearGradient id=\"g\"><stop offset=\"0\" stop-color=\"blue\"/><stop offset=\"1\" stop-color=\"yellow\" stop-opacity=\"0.5\"/></linearGradient><clipPath id=\"c\"><circle cx=\"50\" cy=\"50\" r=\"40\"/></clipPath></defs><rect x=\"-1\" y=\"NaN\" width=\"176\" height=\"24\" fill=\"url(#g)\" rx=\"37\"/></svg>", &XmlRenderOptions::default());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_printpdf_html_add_xml_to_document_9c3016`).

## Output

```
[azul][displaylist] dropping a TextLayout with FontHash::invalid() — its font never resolved, so no renderer would have drawn it
```
