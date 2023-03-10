#![allow(unused)]
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::Map;

fn has_overlap(pair: &str) -> bool {
  let mut ranges: Vec<&str> = pair.split(',').collect();
  let mut range_a: Vec<u32> = ranges[0].split('-').map(|x| x.parse().unwrap()).collect();
  let mut range_b: Vec<u32> = ranges[1].split('-').map(|x| x.parse().unwrap()).collect();

  return (range_a[1] >= range_b[0] && range_a[0] <= range_b[1])
}

fn find_pairs(pairs: Vec<&str>) -> u32 {
  let mut overlaps: u32 = 0;

  for pair in pairs {
    if has_overlap(pair) {
      overlaps += 1;
    }
  }

  overlaps
}

fn main() {
    let file = File::open("src/input.txt");

    let data = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut buf = BufReader::new(data);
    let mut contents = String::new();
    buf.read_to_string(&mut contents);

    let pairs = find_pairs(contents.split_ascii_whitespace().collect());

    println!("{}", pairs);
}
