# timeout in printpdf::html::xml_to_pdf_pages_debug

- crate: printpdf 0.12.8
- kind: timeout
- last API call: `printpdf::html::xml_to_pdf_pages_debug`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling printpdf::html::xml_to_pdf_pages_debug
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
printpdf = { version = "=0.12.8", features = ["bmp", "dds", "gif", "hdr", "html", "html_multithreaded", "ico", "images", "jpeg", "js-sys", "png", "pnm", "rayon", "svg", "text_layout", "text_layout_hyphenation", "tga", "tiff", "wasm-bindgen-futures", "web-sys", "webp"] }
```

`src/main.rs`:

```rust
use printpdf::html::XmlRenderOptions;

fn main() {
    let _ = printpdf::html::xml_to_pdf_pages_debug("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"NaN\" height=\"0\" viewBox=\"0 0 -1 NaN\"><defs><linearGradient id=\"g\"><stop offset=\"0\" stop-color=\"blue\"/><stop offset=\"1\" stop-color=\"yellow\" stop-opacity=\"0.5\"/></linearGradient><clipPath id=\"c\"><circle cx=\"50\" cy=\"50\" r=\"40\"/></clipPath><mask id=\"m\"><rect width=\"100\" height=\"100\" fill=\"white\"/></mask><filter id=\"f\"><feGaussianBlur stdDeviation=\"2\"/></filter></defs><image width=\"NaN\" height=\"1e9\" href=\"data:image/png;base64,iVBORw0KGgo=\"/><ellipse cx=\"-1\" cy=\"0\" rx=\"NaN\" ry=\"NaN\" fill=\"url(#g)\"/><g mask=\"url(#m)\" style=\"mix-blend-mode:multiply\" filter=\"url(#f)\"><g mask=\"url(#m)\" style=\"mix-blend-mode:multiply\" filter=\"url(#f)\"><g mask=\"url(#m)\" style=\"mix-blend-mode:multiply\" filter=\"url(#f)\"><image width=\"1e9\" height=\"78\" href=\"data:image/png;base64,iVBORw0KGgo=\"/><rect x=\"NaN\" y=\"42\" width=\"-1\" height=\"67\" fill=\"url(#g)\" rx=\"-1\"/><image width=\"0\" height=\"140\" href=\"data:image/png;base64,iVBORw0KGgo=\"/></g><rect x=\"0\" y=\"34\" width=\"82\" height=\"133\" fill=\"rgb(10,20,30)\" rx=\"6\"/><polyline points=\"40,NaN 1e9,96 187,1e9\" stroke-dasharray=\"33 167\" stroke=\"rgb(10,20,30)\"/><text x=\"95\" y=\"NaN\" font-size=\"60\" fill=\"red\">Hello żółw 😀</text></g><image width=\"179\" height=\"173\" href=\"data:image/png;base64,iVBORw0KGgo=\"/></svg>", &XmlRenderOptions::default());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_printpdf_html_xml_to_pdf_pages_debug_ce754a`).

## Output

```
[DEBUG xml_to_pdf_pages_debug] Starting, xml length=1286
[DEBUG xml_to_pdf_pages_debug] Preprocessing HTML...
[DEBUG xml_to_pdf_pages_debug] Preprocessed, length=1286
[DEBUG xml_to_pdf_pages_debug] Parsing XML...
[DEBUG xml_to_pdf_pages_debug] XML parsed, got 1 root nodes
[DEBUG xml_to_pdf_pages_debug] Converting to StyledDom...
[DEBUG xml_to_pdf_pages_debug] StyledDom created with 26 nodes
[DEBUG xml_to_pdf_pages_debug] Building font pool from scratch...
[DEBUG xml_to_pdf_pages_debug] Creating font manager...
[DEBUG xml_to_pdf_pages_debug] Font manager created
[DEBUG xml_to_pdf_pages_debug] Starting paged layout...
[azul][displaylist] dropping a TextLayout with FontHash::invalid() — its font never resolved, so no renderer would have drawn it
memory allocation of 536870912 bytes failed
stack backtrace:
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
