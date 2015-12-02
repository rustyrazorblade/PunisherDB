extern crate capnp;
extern crate capnp_rpc;

use capnp_rpc::ez_rpc::EzRpcClient;
use capnp_rpc::capability::{InitRequest, LocalClient, WaitForContent};


pub mod ramp_capnp {
  include!(concat!(env!("OUT_DIR"), "/ramp_capnp.rs"));
}

fn main() {
    println!("Starting up test");
}
