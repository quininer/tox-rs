#[macro_use] mod macros;

use std::fmt;
use std::io;
use std::net::{ ToSocketAddrs, SocketAddr };


pub fn addr_to_string<A: ToSocketAddrs>(addr: A) -> Result<(String, u16), io::Error> {
    let addr = try!(
        try!(addr.to_socket_addrs())
            .next()
            .ok_or(io::Error::new(io::ErrorKind::AddrNotAvailable, "Not Found Addr."))
    );
    let mut host = String::new();
    let port = addr.port();
    match addr {
        SocketAddr::V4(a) => fmt::write(&mut host, format_args!("{}", a.ip())),
        SocketAddr::V6(a) => fmt::write(&mut host, format_args!("{}", a.ip()))
    }.ok();
    Ok((host, port))
}

#[test]
fn test_addr_to_str() {
    assert_eq!(
        addr_to_string("127.0.0.1:80").ok(),
        Some((String::from("127.0.0.1"), 80))
    );
    assert_eq!(
        addr_to_string(("2a02:6b8:0:1::1", 80)).ok(),
        Some((String::from("2a02:6b8:0:1::1"), 80))
    );
}
