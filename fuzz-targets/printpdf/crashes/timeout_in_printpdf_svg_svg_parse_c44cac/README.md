# timeout in printpdf::svg::Svg::parse

- crate: printpdf 0.12.8
- kind: timeout
- last API call: `printpdf::svg::Svg::parse`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling printpdf::svg::Svg::parse
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
printpdf = { version = "=0.12.8", features = ["bmp", "dds", "gif", "hdr", "html", "html_multithreaded", "ico", "images", "jpeg", "js-sys", "png", "pnm", "rayon", "svg", "text_layout", "text_layout_hyphenation", "tga", "tiff", "wasm-bindgen-futures", "web-sys", "webp"] }
```

`src/main.rs`:

```rust
use printpdf::deserialize::PdfParseErrorSeverity;
use printpdf::deserialize::PdfWarnMsg;
use printpdf::svg::Svg;

fn main() {
    let _ = Svg::parse("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"NaN\" height=\"1e9\" viewBox=\"0 0 84 1e9\"><defs><linearGradient id=\"g\"><stop offset=\"0\" stop-color=\"blue\"/><stop offset=\"1\" stop-color=\"yellow\" stop-opacity=\"0.5\"/></linearGradient><clipPath id=\"c\"><circle cx=\"50\" cy=\"50\" r=\"40\"/></clipPath><mask id=\"m\"><rect width=\"100\" height=\"100\" fill=\"white\"/></mask><filter id=\"f\"><feGaussianBlur stdDeviation=\"2\"/></filter></defs><polyline points=\"-1,0 109,0 1e9,119\" stroke-dasharray=\"-1 1e9\" stroke=\"url(#g)\"/><g transform=\"rotate(60) scale(1e9) translate(1e9 67)\" clip-path=\"url(#c)\"></g><g mask=\"url(#m)\" style=\"mix-blend-mode:multiply\" filter=\"url(#f)\"><text x=\"1e9\" y=\"7\" font-size=\"188\" fill=\"transparent\">Hello żółw 😀</text><g transform=\"rotate(114) scale(37) translate(156 0)\" clip-path=\"url(#c)\"><text x=\"NaN\" y=\"136\" font-size=\"-1\" fill=\"rgb(10,20,30)\">Hello żółw 😀</text><text x=\"0\" y=\"1e9\" font-size=\"1e9\" fill=\"red\">Hello żółw 😀</text><polyline points=\"-1,92 116,63 0,16\" stroke-dasharray=\"121 1e9\" stroke=\"red\"/></g><g mask=\"url(#m)\" style=\"mix-blend-mode:multiply\" filter=\"url(#f)\"></g><text x=\"71\" y=\"1e9\" font-size=\"79\" fill=\"none\">Hello żółw 😀</text><g mask=\"url(#m)\" style=\"mix-blend-mode:multiply\" filter=\"url(#f)\"><text x=\"0\" y=\"27\" font-size=\"40\" fill=\"rgb(10,20,30)\">Hello żółw 😀</text></g></g></svg>", &mut vec![PdfWarnMsg::info(0usize, 2usize, "M".to_string()), PdfWarnMsg::info(1usize, 9usize, "<svg xm&amplns=\"http://www.w3.org/2000/svg\"".to_string()), PdfWarnMsg { page: 1usize, op_id: 9usize, severity: PdfParseErrorSeverity::Warning, msg: "_[;t\n.=\0:\u{200b}aik8r媲q$%V#}\u{50d79}\u{97b6f}".to_string() }]);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_printpdf_svg_svg_parse_c44cac`).

