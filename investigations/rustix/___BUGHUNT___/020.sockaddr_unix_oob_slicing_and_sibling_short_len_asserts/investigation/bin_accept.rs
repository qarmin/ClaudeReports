use rustix::net::{
    acceptfrom, bind, connect, getpeername, listen, socket, AddressFamily, SocketAddrUnix,
    SocketType,
};
use std::convert::TryInto;
use std::thread;

fn main() {
    let server = socket(AddressFamily::UNIX, SocketType::STREAM, None).unwrap();
    let server_name = vec![b'S'; 107]; // max-length abstract name
    let server_addr = SocketAddrUnix::new_abstract_name(&server_name).unwrap();
    bind(&server, &server_addr).unwrap();
    listen(&server, 1).unwrap();

    let client_name = vec![b'C'; 107]; // max-length abstract name on the client side too
    let client_thread = thread::spawn(move || {
        let client = socket(AddressFamily::UNIX, SocketType::STREAM, None).unwrap();
        let client_addr = SocketAddrUnix::new_abstract_name(&client_name).unwrap();
        bind(&client, &client_addr).unwrap();
        connect(&client, &server_addr).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(200));
    });

    std::thread::sleep(std::time::Duration::from_millis(50));
    let (conn, peer) = acceptfrom(&server).unwrap();
    println!(
        "acceptfrom peer addr_len = {:?}",
        peer.as_ref().map(|p| p.addr_len())
    );
    if let Some(peer) = peer {
        let unix: SocketAddrUnix = peer.try_into().unwrap();
        println!(
            "acceptfrom abstract_name len = {:?}",
            unix.abstract_name().map(|b| b.len())
        );
    }
    let via_getpeername = getpeername(&conn).unwrap().unwrap();
    let unix: SocketAddrUnix = via_getpeername.try_into().unwrap();
    println!(
        "getpeername abstract_name len = {:?}",
        unix.abstract_name().map(|b| b.len())
    );
    client_thread.join().unwrap();
    println!("ACCEPT/GETPEERNAME TEST: no panic, OK");
}
