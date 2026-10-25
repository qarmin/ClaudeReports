use pdf::file::UpdateOptions;
use pdf::object::LabParams;
use pdf::object::NoUpdate;

fn main() {
    let mut no_update = NoUpdate;
    let _ = <pdf::object::NoUpdate as pdf::object::Updater>::update_with::<pdf::object::LabParams>(&mut no_update, <pdf::object::Ref<pdf::object::Stream<()>>>::from_id(0u64).get_inner(), LabParams { white_point: vec![], black_point: None, range: None }, UpdateOptions::default());
}
