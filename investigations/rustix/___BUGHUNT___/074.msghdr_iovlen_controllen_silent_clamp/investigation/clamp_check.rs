fn msg_iov_len(len: usize) -> std::os::raw::c_int {
    len.try_into().unwrap_or(std::os::raw::c_int::MAX)
}
fn msg_control_len(len: usize) -> u32 {
    len.try_into().unwrap_or(u32::MAX)
}
fn main() {
    for len in [0usize, 1024, (i32::MAX as usize), (i32::MAX as usize)+1, (i32::MAX as usize)+1000] {
        println!("msg_iov_len({len}) = {}", msg_iov_len(len));
    }
    for len in [0usize, 131072, (u32::MAX as usize), (u32::MAX as usize)+1, (u32::MAX as usize)+1_000_000] {
        println!("msg_control_len({len}) = {}", msg_control_len(len));
    }
}
