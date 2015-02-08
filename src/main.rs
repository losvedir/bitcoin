#![feature(io)]

extern crate crypto;
extern crate hyper;
extern crate "rustc-serialize" as rustc_serialize;
extern crate time;
extern crate websocket;

mod circle;
mod coinbase;

fn main() {
    println!("{:?}", coinbase::get_time());
    println!("{:?}", coinbase::get_book());
    println!("{:?}", coinbase::get_accounts());

    println!("{:?}", circle::get_info());

    println!("websocketing....");
    coinbase::get_and_listen_websocket();
}
