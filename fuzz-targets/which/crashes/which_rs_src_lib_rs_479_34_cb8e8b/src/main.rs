use which::sys::RealSys;

fn main() {
    let which_config = <which::WhichConfig<which::sys::RealSys, which::Noop>>::new_with_sys(RealSys);
    let which_config2 = which_config.system_cwd(false);
    let which_config3 = which_config2.system_path_list();
    let _ = which_config3.all_results();
}
