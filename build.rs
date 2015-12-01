extern crate capnpc;

fn main() {
    ::capnpc::compile(".", &["ramp.capnp"]).unwrap();
}
