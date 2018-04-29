mod conversion_paths;

use std::time::SystemTime;
use std::env::args;
use std::io::BufReader;
use std::fs::File;
use std::collections::BTreeSet;
use std::io::prelude::*;
use conversion_paths::ConversionPaths;

fn main() {
    println!("Putting file in memory");
    let file = args().nth(1).unwrap();
    println!("Arg: {}", file);
    let f = File::open(file).unwrap();
    let reader = BufReader::new(f);

    let sessions: Vec<BTreeSet<u32>> = reader.lines().map(|l| {
        let line = l.unwrap();
        let set: BTreeSet<u32> = line.split(",").map(|w| w.parse().unwrap()).collect();
        set
    }).collect();

    println!("Processing ...");
    let start = SystemTime::now();
    ConversionPaths::build(50, sessions.into_iter());
    let elapsed = start.elapsed().unwrap();
    let millis = &elapsed.as_secs() * 1000 + u64::from(&elapsed.subsec_nanos() / 1_000_000);
    println!("Processing took {} ms", millis);
}
