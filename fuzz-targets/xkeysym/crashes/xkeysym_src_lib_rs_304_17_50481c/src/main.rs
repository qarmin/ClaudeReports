use xkeysym::KeyCode;

fn main() {
    let key_code = KeyCode::default();
    let _ = xkeysym::keysym(key_code, 0u8, KeyCode::from(1u32), 0u8, &[]);
}
