#![allow(unused)]
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::Map;

fn compare_strings (strings: &[&str]) -> u32 {
  let mut val: u32 = 0;
  for chr in strings[0].chars() {
    if strings[1].contains(chr) && strings[2].contains(chr) {
      print!("{}", chr);
      if chr.is_ascii_lowercase() {
        val = chr as u32 - 'a' as u32 + 1;
        break;
      } else {
        val = chr as u32 - 'A' as u32 + 27;
        break;
      }
    }
  }
  println!("{}", val);
  return val
}
fn main() {
    let day_3_file_handle = File::open("src/day_3.txt");

    let day_3_file = match day_3_file_handle {
        Ok(day_3_file_handle) => day_3_file_handle,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut buf = BufReader::new(day_3_file);
    let mut contents = String::new();
    buf.read_to_string(&mut contents);

    /*
    For each element of the array 
    - create a new array of arrays with three strings in each sub array

    Map the array to find the common char between all strings to the value of their item type
    a - z (1-26)
    A - Z (27 - 52)
     */

    let mut index: usize = 0;
    let mut contents_vec: Vec<&str> = contents.split_whitespace().collect();
    let mut groups_of_three: Vec<&[&str]> = vec![];
    let cont_len = contents_vec.len();

    while index < cont_len {
      let group = &contents_vec[index..(index+3)];
      groups_of_three.push(group);

      index += 3;
    }

    assert_eq!(groups_of_three.len(), cont_len / 3);

    let mut value: u32 = groups_of_three.iter().map(|x| compare_strings(*x) ).sum();

    println!("sum {}", value);
}
