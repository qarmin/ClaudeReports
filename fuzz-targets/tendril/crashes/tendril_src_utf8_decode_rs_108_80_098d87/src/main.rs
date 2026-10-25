use tendril::IncompleteUtf8;

fn main() {
    let mut incomplete_utf8 = IncompleteUtf8 { buffer: [0u8, 0u8, 255u8, 0u8], buffer_len: 3u8 };
    let _ = incomplete_utf8.try_to_complete_codepoint(&vec![]);
}
