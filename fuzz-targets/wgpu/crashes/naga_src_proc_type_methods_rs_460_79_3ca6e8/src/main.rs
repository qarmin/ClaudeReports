use naga::ir::TypeInner;

fn main() {
    let _ = TypeInner::Struct { members: vec![], span: 1u32 }.component_type(0usize);
}
