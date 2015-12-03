#[macro_use]
extern crate capnp;
extern crate capnp_rpc;
extern crate ramp;
extern crate log;
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
pub mod client;

use ramp_capnp::{ramp_interface};

type DB = RwLock<Database>;

struct RampServer {
    db:DB
}


impl RampServer {
    fn new() -> RampServer {
        // create a new Database, wrap in RwLock and Arc
        let db = RwLock::new(Database::new());
        RampServer{db:db}
    }
}

impl ramp_interface::Server for RampServer {
    fn prepare(&mut self, mut context: ramp_interface::PrepareContext) {
        println!("Preparing");
        {
            let (params, mut results) = context.get();
            let key = params.get_key().unwrap();
            let value = params.get_value().unwrap();

            let deps = {
                let target = params.get_dependencies().unwrap();
                let size = target.len();
                let mut entries = Vec::with_capacity(size as usize);
                for i in 0..size {
                    entries.push(target.get(i).unwrap().to_string());
                }
                entries
            };

            // let deps = params.get_dependencies().unwrap();
            let timestamp = params.get_timestamp();

            println!("Acquiring write lock");
            let mut db = self.db.write().unwrap(); // hold lock till prepare is done
            db.prepare(key.to_string(), value.to_string(), deps, timestamp);
        }

        context.done();
        println!("Prepared");
    }

    fn commit(&mut self, mut context: ramp_interface::CommitContext) {
        println!("Committing");
        {
            let (params, mut results) = context.get();
            let ts = params.get_timestamp();

            println!("Acquiring write lock");
            let mut db = self.db.write().unwrap();
            println!("Write lock acquired");
            db.commit(ts);

        }

        context.done();
    }

    fn get(&mut self, mut context: ramp_interface::GetContext) {
        {
            let (params, mut results) = context.get();
            let key = params.get_key().unwrap();

            let mut db = self.db.write().unwrap();
            let version = db.get(key.to_string());

            match version {
                Some(v) => {
                    println!("Found version {} {}", v.timestamp, &v.value);
                    let mut r = results.init_result();
                    let mut ver = r.init_version();
                    ver.set_value(&v.value);
                    ver.set_timestamp(v.timestamp);

                    let len = v.dependencies.len() as u32;
                    let mut deps = ver.init_dependencies(len);

                    println!("{} dependencies: {:?}", len, v.dependencies);
                    for i in 0..len {
                        println!("Setting dependency {}",i);
                        deps.set(i as u32, &v.dependencies.get(i as usize).unwrap());
                    }
                    println!("Deps set for get return");
                },
                None => {
                    let mut r = results.init_result();
                    r.set_none(());
                }
            };

        }
        context.done();
    }

    fn get_version(&mut self, mut context: ramp_interface::GetVersionContext) {
        {
            println!("Getting specific version");
            let (params, mut results) = context.get();
            let key = params.get_key().unwrap();
            let timestamp = params.get_timestamp();

            let mut db = self.db.write().unwrap();
            let version = db.get_version(key.to_string(), timestamp);

            match version {
                Some(v) => {
                    let mut r = results.init_result();
                    let mut ver = r.init_version();
                    ver.set_value(&v.value);
                    ver.set_timestamp(v.timestamp);

                    let len = v.dependencies.len() as u32;
                    let mut deps = ver.init_dependencies(len);
                    for i in 0..len {
                        deps.set(i as u32, &v.dependencies.get(i as usize).unwrap());
                    }
                },
                None => {
                    // let mut r = results.init_result();
                    // r.set_none(());
                }
            };
        }

        context.done();
    }
}

fn main() {
    println!("Hello, world!");

    let rpc_server = EzRpcServer::new("127.0.0.1:6000").unwrap();

    let ramp = Box::new(RampServer::new());
    let ramp_server = Box::new(ramp_interface::ServerDispatch { server : ramp });
    rpc_server.serve(ramp_server);

    println!("Goodbye forever.");
}
