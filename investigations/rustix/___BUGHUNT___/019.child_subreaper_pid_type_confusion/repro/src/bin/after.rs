// Same scenario against the fixed bool-based API: a plain boolean round-trips correctly because
// it now matches what the kernel actually tracks (kernel/sys.c: is_child_subreaper = !!arg2).
use rustix::process::{child_subreaper, set_child_subreaper};

fn main() {
    let before = child_subreaper().unwrap();
    assert_eq!(before, false, "expected no subreaper set initially, got {:?}", before);

    set_child_subreaper(true).unwrap();
    let after = child_subreaper().unwrap();
    println!("set_child_subreaper(true) -> child_subreaper() returned {:?}", after);

    set_child_subreaper(false).unwrap();

    if after == true {
        println!("did not reproduce: bool round-trips correctly");
        std::process::exit(0);
    } else {
        println!("REPRODUCED (regression): expected true, got {:?}", after);
        std::process::exit(1);
    }
}
