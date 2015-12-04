use std::thread;
use std::sync::Arc;

fn main() {
    // create 10 threads
    let v = vec!("jon", "steve", "marcus",
                 "eric", "dre", "izzy", "jackson");
    let keys = Arc::new(v);

    for x in 0..50 {
        thread::spawn(|| run_test() );
    }

}

fn run_test() {
    println!("Starting benchmark thread");
    // pick 3 keys
}
