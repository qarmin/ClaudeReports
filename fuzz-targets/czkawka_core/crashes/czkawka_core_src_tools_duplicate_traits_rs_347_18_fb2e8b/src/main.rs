use czkawka_core::common::model::CheckingMethod;
use czkawka_core::common::model::HashType;
use czkawka_core::tools::duplicate::DuplicateFinder;
use czkawka_core::tools::duplicate::DuplicateFinderParameters;

fn main() {
    let _ = <czkawka_core::tools::duplicate::DuplicateFinder as czkawka_core::common::traits::PrintResults>::print_results_to_writer::<Vec<u8>>(&DuplicateFinder::new(DuplicateFinderParameters::new(CheckingMethod::VideoAudioContent, HashType::default(), false, 0u64, 0u64, false)), &mut vec![]);
}
