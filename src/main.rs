#![feature(alloc_system)]
extern crate alloc_system;

mod conversion_paths_2;

use std::time::SystemTime;
use std::env::args;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use conversion_paths_2::ConversionPaths;

fn main() {
    println!("Putting file in memory");
    let file = args().nth(1).unwrap();
    println!("Arg: {}", file);
    let f = File::open(file).unwrap();
    let reader = BufReader::new(f);

    let mut lines: Vec<Vec<u32>> = reader.lines().map(|l| {
        let line = l.unwrap();
        let mut vec: Vec<u32> = line.split(",").map(|w| w.parse().unwrap()).collect();
        vec.reverse();
        vec
    }).collect();
    lines.sort();

    let sessions: Vec<&[u32]> = lines.iter().map(|l| &l[..]).collect();

    println!("Processing ...");
    let start = SystemTime::now();
    let from_goal: Vec<&[u32]> = sessions.into_iter().filter_map(|s| {
        let last_target_index = s.iter().rposition(|g| g == &50);
        let xs: Option<&[u32]> = last_target_index.map(|i| &s[i..]);
        xs
    }).collect();

    ConversionPaths::build(&from_goal);
    let elapsed = start.elapsed().unwrap();
    let millis = &elapsed.as_secs() * 1000 + u64::from(&elapsed.subsec_nanos() / 1_000_000);
    println!("Processing took {} ms", millis);
}
