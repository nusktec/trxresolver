extern crate serde_json;
extern crate rand;

use std::thread::sleep;
//import for thread sleep and interruption
use std::fs;
use std::time::{Duration, SystemTime, Instant, UNIX_EPOCH};
//global imports
use serde_json::{json, Value, Result};

#[warn(unused_imports)]
//custom modules
mod cores;
mod transcriber;

//circle timer in secs
static CIRCLE_TIMER_SECS_DELAY: u64 = 2;

//starter function and loader
fn main() {
    //cores::trx_enc();
    //instance counter
    let mut _clock: i64 = 0;
    //runner till error encountered
    loop {
        //junk file checker
        let jsc = cores::junk_server_health();
        _clock = _clock + 1;
        println!("\n:90 server running: circle-{}-ok: {}", _clock, jsc);
        //assert for block number and kill thread
        assert_eq!(jsc, true);
        sleep(Duration::from_secs(CIRCLE_TIMER_SECS_DELAY));
        //call resolver method
        trx_resolver(remote_loader());
    }
}

//seeder, protocol life check
fn remote_loader() -> Value {
    let base_url = cores::SERVER_URL;
    //check server health
    let resp = reqwest::blocking::get(base_url).unwrap();
    //check if server is okay
    assert_eq!(resp.status(), 200);
    //convert to json
    let data: Value = resp.json().unwrap();
    //return json value
    return data;
}

//transaction resolver and builder
fn trx_resolver(j: Value) {
    //get transaction stamp
    let _trx_stamp = cores::trx_enc();
    //transaction code
    print!("{}\n", _trx_stamp);
}