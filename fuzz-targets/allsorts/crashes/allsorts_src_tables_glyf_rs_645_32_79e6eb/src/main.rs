use allsorts::binary::read::ReadScope;

fn main() {
    let vec = include_bytes!("data_0.bin").to_vec();
    let read_scope = ReadScope::new(&vec);
    let mut read_ctxt = read_scope.ctxt();
    let _ = <allsorts::woff::WoffHeader as allsorts::binary::read::ReadBinary>::read(&mut read_ctxt);
    let _ = <allsorts::tables::glyf::Glyph as allsorts::binary::read::ReadBinary>::read(&mut read_ctxt);
}
