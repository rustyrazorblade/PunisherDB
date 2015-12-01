#[macro_use]
extern crate capnp;
extern crate capnp_rpc;

use capnp::capability::{Server};
use capnp_rpc::ez_rpc::EzRpcServer;

// this is generated automatically as part of the build process
// see build.rs
pub mod ramp_capnp {
  include!(concat!(env!("OUT_DIR"), "/ramp_capnp.rs"));
}
use ramp_capnp::ramp;

struct RampServer {
    db:i64
}

impl RampServer {
    fn new() -> RampServer {
        RampServer{db:1}
    }

    fn listen(&mut self) {

    }
}

impl ramp::Server for RampServer {
    fn prepare(&mut self, mut context: ramp::PrepareContext) {
        let (params, mut results) = context.get();
        let key = params.get_key().unwrap();
        let value = params.get_value().unwrap();
        let timestamp = params.get_value().unwrap();

    }

    fn commit(&mut self, mut context: ramp::CommitContext) {

    }
}

fn main() {
    println!("Hello, world!");

    let ramp = RampServer::new();
    ramp.listen();

    println!("Goodbye forever.");
}
