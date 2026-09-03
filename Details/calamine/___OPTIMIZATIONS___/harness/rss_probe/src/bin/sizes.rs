fn main() {
    println!("size_of::<calamine::Data> = {}", std::mem::size_of::<calamine::Data>());
    println!("size_of::<calamine::Cell<calamine::Data>> = {}", std::mem::size_of::<calamine::Cell<calamine::Data>>());
    println!("size_of::<calamine::DataRef> = {}", std::mem::size_of::<calamine::DataRef>());
    println!("align_of::<calamine::Data> = {}", std::mem::align_of::<calamine::Data>());
    println!("align_of::<calamine::DataRef> = {}", std::mem::align_of::<calamine::DataRef>());
}
