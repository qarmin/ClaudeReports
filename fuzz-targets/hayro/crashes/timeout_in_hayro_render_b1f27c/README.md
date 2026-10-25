# timeout in hayro::render

- crate: https://github.com/LaurenzV/hayro @ e0cd0f7c56de05bdf87b8252a6abbe31d870aa65
- kind: timeout
- last API call: `hayro::render`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling hayro::render
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
hayro = { git = "https://github.com/LaurenzV/hayro", rev = "e0cd0f7c56de05bdf87b8252a6abbe31d870aa65" }
hayro-interpret = { git = "https://github.com/LaurenzV/hayro", rev = "e0cd0f7c56de05bdf87b8252a6abbe31d870aa65" }
hayro-jbig2 = { git = "https://github.com/LaurenzV/hayro", rev = "e0cd0f7c56de05bdf87b8252a6abbe31d870aa65" }
hayro-syntax = { git = "https://github.com/LaurenzV/hayro", rev = "e0cd0f7c56de05bdf87b8252a6abbe31d870aa65" }
```

`src/main.rs`:

```rust
use hayro::PixmapSettings;
use hayro::RenderCache;
use hayro::RenderSettings;
use hayro_interpret::InterpreterSettings;
use hayro_jbig2::ParseError;
use hayro_syntax::Pdf;
use hayro_syntax::PdfData;

fn main() {
    let _ = ParseError::UnexpectedEof;
    let pdf = Pdf::new(PdfData::from(include_bytes!("document_0.pdf").to_vec())).unwrap();
    let pages = pdf.pages();
    let page = pages.get(0usize).unwrap();
    let render_cache = RenderCache::new();
    let _ = hayro::render(page, &render_cache, &InterpreterSettings::default(), &RenderSettings::default(), &PixmapSettings::default());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_hayro_render_b1f27c`).

