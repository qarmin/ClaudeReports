// Scratch example for bug-hunt candidate 062 (socket_addr_any_field_order_sweep).
// Exercises SocketAddrAny::new() and ::read() - the two constructors using
// `Self { storage, len }` field order inconsistent with the struct's declared
// `len` then `storage` order - to confirm no init-order hazard at runtime.
// Not part of the permanent patch; discarded via `git checkout` at session end.

fn main() {
    use core::mem::MaybeUninit;
    use rustix::net::addr::SocketAddrStorage;
    use rustix::net::{AddressFamily, SocketAddrAny};

    let storage_len = core::mem::size_of::<SocketAddrStorage>();
    // Linux/BSD sockaddr_in: u16 family + u16 port + u32 addr + 8 bytes padding = 16 bytes.
    let sockaddr_in_len: u32 = 16;

    unsafe {
        // SocketAddrAny::new(): minimal plausible len.
        let mut buf: MaybeUninit<SocketAddrStorage> = MaybeUninit::zeroed();
        *(buf.as_mut_ptr() as *mut u16) = 2u16; // AF_INET
        let any = SocketAddrAny::new(buf, sockaddr_in_len);
        assert_eq!(any.address_family(), AddressFamily::INET);
        println!("new(): family = {:?}", any.address_family());

        // SocketAddrAny::read(): via a real pointer + copy_nonoverlapping path.
        let mut src: MaybeUninit<SocketAddrStorage> = MaybeUninit::zeroed();
        *(src.as_mut_ptr() as *mut u16) = 2u16;
        let src_ptr = src.as_ptr();
        let any2 = SocketAddrAny::read(src_ptr, sockaddr_in_len);
        assert_eq!(any2.address_family(), AddressFamily::INET);
        println!("read(): family = {:?}", any2.address_family());

        // Boundary: len == full storage size (max allowed).
        let mut buf3: MaybeUninit<SocketAddrStorage> = MaybeUninit::zeroed();
        *(buf3.as_mut_ptr() as *mut u16) = 2u16;
        let any3 = SocketAddrAny::new(buf3, storage_len as u32);
        assert_eq!(any3.address_family(), AddressFamily::INET);
        println!("new() at max len {} : family = {:?}", storage_len, any3.address_family());

        // Boundary: len == header-sized minimum (sa_family_t only, 2 bytes on Linux,
        // but SocketAddrAny::new asserts len >= size_of::<sockaddr_header>()).
        // sockaddr_header is typically { sa_family_t, ... } - try the smallest
        // value that passed the internal assert above (sockaddr_in_len) already
        // covers a realistic minimal case; also try an explicit tiny attempt that
        // is expected to panic to confirm the assert boundary is independent of
        // field order too.
    }

    println!("OK - no init-order hazard observed");
}
