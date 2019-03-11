extern crate warp;

use warp::Filter;

fn main() {
    println!("Serving Warp app...");
    let base = warp::any().map(|| "Hello from the AXC-2152!\n");
    let addr = [192, 168, 10, 2];

    warp::serve(base).run((addr, 8080));
}
