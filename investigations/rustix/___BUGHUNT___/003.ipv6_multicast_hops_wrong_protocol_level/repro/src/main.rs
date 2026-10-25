use std::os::fd::AsRawFd;

use rustix::net::{sockopt, AddressFamily, SocketType};

fn independent_getsockopt_ipv6_multicast_hops(fd: i32) -> Result<i32, i32> {
    // Bypass rustix entirely: raw libc getsockopt at the CORRECT level
    // (IPPROTO_IPV6) to see what the kernel actually holds.
    unsafe {
        let mut val: libc::c_int = -1;
        let mut len: libc::socklen_t = std::mem::size_of::<libc::c_int>() as libc::socklen_t;
        let rc = libc::getsockopt(
            fd,
            libc::IPPROTO_IPV6,
            libc::IPV6_MULTICAST_HOPS,
            &mut val as *mut _ as *mut libc::c_void,
            &mut len,
        );
        if rc == 0 {
            Ok(val)
        } else {
            Err(*libc::__errno_location())
        }
    }
}

fn independent_getsockopt_ip_passsec(fd: i32) -> Result<i32, i32> {
    // IPPROTO_IP option 18 == IP_PASSSEC on Linux. This is the option that
    // actually gets hit if level IPPROTO_IP is (incorrectly) used with
    // optname IPV6_MULTICAST_HOPS (==18), since option numbers are only
    // unique within a protocol level.
    const IP_PASSSEC: libc::c_int = 18;
    unsafe {
        let mut val: libc::c_int = -1;
        let mut len: libc::socklen_t = std::mem::size_of::<libc::c_int>() as libc::socklen_t;
        let rc = libc::getsockopt(
            fd,
            libc::IPPROTO_IP,
            IP_PASSSEC,
            &mut val as *mut _ as *mut libc::c_void,
            &mut len,
        );
        if rc == 0 {
            Ok(val)
        } else {
            Err(*libc::__errno_location())
        }
    }
}

fn main() {
    let sock = rustix::net::socket(AddressFamily::INET6, SocketType::DGRAM, None)
        .expect("failed to create AF_INET6/SOCK_DGRAM socket");
    let fd = sock.as_raw_fd();

    let before_hops = independent_getsockopt_ipv6_multicast_hops(fd);
    let before_passsec = independent_getsockopt_ip_passsec(fd);
    println!("BEFORE: real IPV6_MULTICAST_HOPS (IPPROTO_IPV6) = {:?}", before_hops);
    println!("BEFORE: IP_PASSSEC (IPPROTO_IP, optname 18)     = {:?}", before_passsec);

    let target: u32 = 7;
    let set_result = sockopt::set_ipv6_multicast_hops(&sock, target);
    println!("rustix set_ipv6_multicast_hops({}) -> {:?}", target, set_result);

    let rustix_get = sockopt::ipv6_multicast_hops(&sock);
    println!("rustix ipv6_multicast_hops() -> {:?}", rustix_get);

    let after_hops = independent_getsockopt_ipv6_multicast_hops(fd);
    let after_passsec = independent_getsockopt_ip_passsec(fd);
    println!("AFTER:  real IPV6_MULTICAST_HOPS (IPPROTO_IPV6) = {:?}", after_hops);
    println!("AFTER:  IP_PASSSEC (IPPROTO_IP, optname 18)     = {:?}", after_passsec);

    let bug_confirmed = after_hops.map(|v| v as u32) != Ok(target);
    let side_effect = before_passsec != after_passsec;
    println!();
    println!(
        "==> BUG CONFIRMED (kernel's real IPV6_MULTICAST_HOPS did NOT change to {}): {}",
        target, bug_confirmed
    );
    println!(
        "==> UNINTENDED SIDE EFFECT on unrelated IP_PASSSEC option: {}",
        side_effect
    );
}
