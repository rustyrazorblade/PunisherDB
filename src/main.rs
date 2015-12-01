#[macro_use]
extern crate capnp;
extern crate capnp_rpc;
extern crate ramp;

use std::sync::mpsc::channel;
use std::thread;
use std::sync::{RwLock, Arc};

use ramp::Database;

use capnp::capability::{Server};
use capnp_rpc::ez_rpc::EzRpcServer;

// this is generated automatically as part of the build process
// see build.rs
pub mod ramp_capnp {
  include!(concat!(env!("OUT_DIR"), "/ramp_capnp.rs"));
}
use ramp_capnp::ramp_interface;

type DB = Arc<RwLock<Database>>;

struct RampServer {
    db:DB
}


impl RampServer {
    fn new() -> RampServer {
        // create a new Database, wrap in RwLock and Arc
        let db = Arc::new(RwLock::new(Database::new()));
        RampServer{db:db}
    }
}

impl ramp_interface::Server for RampServer {
    fn prepare(&mut self, mut context: ramp_interface::PrepareContext) {
        let (params, mut results) = context.get();
        let key = params.get_key().unwrap();
        let value = params.get_value().unwrap();
        let timestamp = params.get_value().unwrap();
    }

    fn commit(&mut self, mut context: ramp_interface::CommitContext) {

    }
}

fn main() {
    println!("Hello, world!");

    let ramp = RampServer::new();

    println!("Goodbye forever.");
}
