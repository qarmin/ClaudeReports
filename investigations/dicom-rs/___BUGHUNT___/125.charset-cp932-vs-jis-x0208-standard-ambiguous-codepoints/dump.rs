use encoding::{DecoderTrap, Encoding};
use std::fs::File;
use std::io::{BufWriter, Write};

fn dump_double_byte(enc: &'static dyn Encoding, name: &str, out: &mut BufWriter<File>) {
    for lead in 0u16..256 {
        for trail in 0u16..256 {
            let bytes = [lead as u8, trail as u8];
            if let Ok(s) = enc.decode(&bytes, DecoderTrap::Strict) {
                let cps: String = s.chars().map(|c| format!("{:04X}", c as u32)).collect::<Vec<_>>().join(",");
                writeln!(out, "{}\t{:02X}{:02X}\t{}", name, lead, trail, cps).unwrap();
            }
        }
    }
}

fn dump_jis0208(enc: &'static dyn Encoding, out: &mut BufWriter<File>) {
    for lead in 0x21u8..=0x7E {
        for trail in 0x21u8..=0x7E {
            let mut bytes = vec![0x1B, b'$', b'B'];
            bytes.push(lead);
            bytes.push(trail);
            if let Ok(s) = enc.decode(&bytes, DecoderTrap::Strict) {
                let cps: String = s.chars().map(|c| format!("{:04X}", c as u32)).collect::<Vec<_>>().join(",");
                writeln!(out, "ISO_2022_JP_0208\t{:02X}{:02X}\t{}", lead, trail, cps).unwrap();
            }
        }
    }
}

fn main() {
    let f = File::create("charset_dump.tsv").unwrap();
    let mut out = BufWriter::new(f);
    dump_double_byte(encoding::all::WINDOWS_31J, "WINDOWS_31J", &mut out);
    dump_double_byte(encoding::all::WINDOWS_949, "WINDOWS_949", &mut out);
    dump_jis0208(encoding::all::ISO_2022_JP, &mut out);
    out.flush().unwrap();
    eprintln!("done");
}
