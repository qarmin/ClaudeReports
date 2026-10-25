use anstyle_parse::AsciiParser;

fn main() {
    let mut ascii_parser = AsciiParser;
    let _ = <anstyle_parse::AsciiParser as anstyle_parse::CharAccumulator>::add(&mut ascii_parser, 0u8);
}
