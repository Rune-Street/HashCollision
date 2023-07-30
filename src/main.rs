#![feature(slice_pattern)]
extern crate core;

use core::slice::SlicePattern;
use std::process::exit;
use md5::{Md5, Digest};
use hex;

const TARGET_HASH_BYTES: [u8; 16] = [94, 182, 59, 187, 224, 30, 238, 208, 147, 203, 34, 187, 143, 90, 205, 195];

fn main() {
    let mut line = String::new();
    println!("Enter the number of bytes you'd like to match (try 1 or 2, 3+ will be slow): ");
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    let mut matching = line.trim().parse::<usize>().expect("Please enter a valid number");

    // create a Md5 hasher instance

    for val in 0..u64::MAX {
        let mut hasher = Md5::new();
        hasher.update(val.to_ne_bytes());
        let result = hasher.finalize();
        let result_slice = result.as_slice();
        let mut found = true;
        for index in 0..matching {
            if result[index] != TARGET_HASH_BYTES[index] {
                found = false;
                break
            }
        }
        if found {
            println!("Found hash collision with target_hash: value = {}", val);
            println!("Original hash: {:?}", TARGET_HASH_BYTES);
            println!("Colliding hash: {:?}", result_slice);
            exit(0);
        }
    }
}
