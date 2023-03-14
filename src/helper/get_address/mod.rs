use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use dotenv::var;

pub fn get_address() -> SocketAddr {
    let port: u16 = var("PORT").unwrap().parse().unwrap();

    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port))
}
