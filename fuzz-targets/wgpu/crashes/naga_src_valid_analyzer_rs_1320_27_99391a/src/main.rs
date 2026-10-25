use naga::valid::ModuleInfo;

fn main() {
    let module_info = ModuleInfo::default();
    let _ = module_info.get_entry_point(0usize);
}
