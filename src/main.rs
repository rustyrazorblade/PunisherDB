#![feature(slice_concat_ext)]

extern crate ramp;
extern crate bufstream;
extern crate regex;

use std::sync::mpsc::channel;
use std::thread;
use std::sync::{RwLock, Arc};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, BufRead, Write};
use regex::Regex;
use std::slice::SliceConcatExt;
#[macro_use]
extern crate log;
extern crate env_logger;

use bufstream::BufStream;

use ramp::Database;

type DB = Arc<RwLock<Database>>;


fn main() {
    /*
    Logger notes:
    RUST_LOG=error ./main
    RUST_LOG=info
    http://rust-lang.github.io/log/env_logger/
    */

    info!("Starting up RAMP socket server!");
    let db = Arc::new(RwLock::new(Database::new()));
    let listener = TcpListener::bind("127.0.0.1:6000").unwrap();

    info!("Socket bound");

    for stream in listener.incoming() {
        // info!("connection established, spawning new thread");
        let db2 = db.clone();
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream, db2)
                    });
            },
            Err(e) => {}
        }
    }

    info!("Goodbye forever.");
}

fn handle_client(mut stream: TcpStream, mut db: DB ) {
    info!("Starting new client thread, creating regexes");

    let prepare = Regex::new(r"prepare\s+([:alpha:]+)\s+([:alpha:]+)\s+(\d+)\s?([a-z,]*)").unwrap();
    let commit = Regex::new(r"commit (\d+)").unwrap();
    let get_version = Regex::new(r"get\s+([:alpha:]+)\s+(\d+)").unwrap();
    let get_current = Regex::new(r"get\s+([:alpha:]+)").unwrap();

    let mut buf = BufStream::new(stream.try_clone().unwrap());

    let mut buffer = String::new();
    for line in buf.lines() {
        let l = line.unwrap();
        println!("Line: {}", l);

        if prepare.is_match(&l) {
            println!("prepare statement");
            let cap = prepare.captures(&l).unwrap();
            let key = cap.at(1).unwrap();
            let value = cap.at(2).unwrap();
            let timestamp = cap.at(3).unwrap().parse::<i64>().unwrap();

            println!("Key, value, timestamp, deps: {} : {} : {} : {}",
                     key, value, timestamp, cap.at(4).unwrap());

            let deps : Vec<String> = cap.at(4).unwrap()
                                    .split(",").map(|x| x.to_string())
                                    .collect();

            println!("depencencies: {:?}", deps );

            {
                let mut writer = (*db).write().unwrap();
                writer.prepare(key.to_string(),
                           value.to_string(),
                           deps,
                           timestamp);
            }
            stream.write("PREPARED\n".as_bytes());
            continue;
        } else if commit.is_match(&l) {
            let cap = commit.captures(&l).unwrap();
            let timestamp = cap.at(1).unwrap().parse::<i64>().unwrap();
            {
                let mut writer = (*db).write().unwrap();
                writer.commit(timestamp);
            }
            stream.write("COMMITTED\n".as_bytes());
            continue;
        } else if get_version.is_match(&l) {
            let cap = get_version.captures(&l).unwrap();
            let key = cap.at(1).unwrap().to_string();
            let timestamp = cap.at(2).unwrap().parse::<i64>().unwrap();
            println!("Get version");
            {
                let mut reader = (*db).read().unwrap();
                match reader.get_version(key, timestamp) {
                    Some(version) => {

                        let d = version.dependencies.join(",");
                        let response = format!("{} {} {}\n",
                                                version.value,
                                                version.timestamp,
                                                d);
                        stream.write(response.as_bytes());
                    },
                    None => {
                        stream.write("NOT FOUND\n".as_bytes());
                    }
                };

            }
            continue;
        } else if get_current.is_match(&l) {
            println!("Get current");
            let cap = get_current.captures(&l).unwrap();
            let key = cap.at(1).unwrap().to_string();
            // let key =
            {
                let mut reader = (*db).read().unwrap();
                match reader.get(key) {
                    Some(version) => {

                        let d = version.dependencies.join(",");
                        let response = format!("{} {} {}\n",
                                                version.value,
                                                version.timestamp,
                                                d);
                        stream.write(response.as_bytes());
                    },
                    None => {
                        stream.write("NOT FOUND\n".as_bytes());
                    }
                };

            }

            continue
        }
    }
}
