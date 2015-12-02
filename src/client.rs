extern crate capnp;
extern crate capnp_rpc;

use capnp_rpc::ez_rpc::EzRpcClient;
use capnp_rpc::capability::{InitRequest, LocalClient, WaitForContent};


pub mod ramp_capnp {
  include!(concat!(env!("OUT_DIR"), "/ramp_capnp.rs"));
}

use ramp_capnp::ramp_interface;

fn main() {
    println!("Starting up test");

    let mut rpc_client = EzRpcClient::new("127.0.0.1:6000").unwrap();
    let mut client : ramp_interface::Client = rpc_client.get_main();

    { // this scoping is kind of ridiculous
        println!("Preparing");
        let mut request = client.prepare_request();
        {
            let mut builder = request.init();
            builder.set_key("test");
            builder.set_value("haddad");
            builder.set_timestamp(1);
            // add dependencies
        }
        let promise = request.send();
        // println!("Response: {:?}",
    }
    // get request, check value

    {
        let mut request = client.commit_request();
        {
            let mut builder = request.init();
            builder.set_timestamp(1);
        }
        let promise = request.send();
    }


    println!("Punisher client test ended");
}
