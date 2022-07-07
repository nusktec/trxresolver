#[warn(dead_code)]
extern crate serde_json;

use std::fs;
use std::convert;
use std::char;
use serde_json::{json, Value, Result};
use rand::prelude::*;
use std::alloc::System;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::borrow::Borrow;
use std::thread::sleep;

//seeder link
pub static SERVER_URL: &str = "https://trustteller.com/dev/seeder-1.json";
//block length
static BLOCK_LENGTH: usize = 544;

//check live server
pub fn junk_server_health() -> bool {
    let mock_server = fs::read_to_string("junks.json").expect("File not found or damaged !");
    //cast string to json
    let jx = mock_server.len();
    //constant signature 544 and check block length
    let expected = (jx % BLOCK_LENGTH);
    print!("\nBlock length re-checked confirmation: {}", jx);
    //do detail comparison and assert
    if expected != 0 {
        return false;
    }
    return true;
}

//encryption
pub fn trx_enc() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    //get current time
    let xtime = since_the_epoch.as_secs();
    let signature_digits = xtime.to_string();
    //temp.chars().map(|d| d.to_digit(10));
    //block variable
    let mut _block_number: String = String::new();
    //numbers generators
    let _alphabet = (10..36).map(|i| char::from_digit(i, 36).unwrap()).collect::<Vec<_>>();
    //code_holder
    let mut xcode: String = "".to_owned();
    for n in 1..5 {
        //run mixed-up
        for mut x in 1..3 {
            x = x + xtime;
            _block_number += &((x * n) * (xtime / 6000)).to_string();
        }
        //build string
        for _num in signature_digits.chars() {
            xcode.push(_alphabet[(_num as usize) / 4]);
            xcode.push_str(&_block_number);
        }
        xcode.push_str("-");
    }
    return xcode;
}