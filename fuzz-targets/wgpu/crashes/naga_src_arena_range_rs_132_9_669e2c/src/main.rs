fn main() {
    let arena = <naga::Arena<naga::diagnostic_filter::DiagnosticFilterNode>>::new();
    let _ = arena.range_from(1usize);
}
