use serde_wasm_bindgen::Error;

fn main() {
    let _ = Error::new::<String>("".to_string());
}
