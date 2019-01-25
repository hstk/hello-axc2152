extern crate warp;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use warp::Filter;

fn main() {
    println!("Serving Warp app...");

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    // let hello = path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let base = warp::any().map(|| "Hello from the AXC-2152!");

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 10, 2)), 8080);

    warp::serve(base).run(socket);
    // warp::serve(hello)
    //     .run(([127, 0, 0, 1], 3030));
}
