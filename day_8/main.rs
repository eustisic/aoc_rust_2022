#![allow(unused)]
use std::fs::File;
use std::io::{BufReader, Read};

fn check_height(index: (usize, usize), trees: &Vec<Vec<u32>>) -> (u32, u32) {
  // row column
  let mut visible = 0;
  let tree: &u32 = &trees[index.0][index.1];
  let mut scores: Vec<u32> = Vec::new();
  let mut final_score = 1;

  let rows_up: Vec<&u32> = trees[0..index.0].iter().map(|row| &row[index.1]).rev().collect();
  let rows_down: Vec<&u32> = trees[index.0+1..].iter().map(|row| &row[index.1]).collect();
  let cols_left: Vec<&u32> = trees[index.0][0..index.1].iter().rev().collect();
  let cols_right: Vec<&u32> = trees[index.0][index.1+1..].iter().collect();

  let slices = [&rows_up, &rows_down, &cols_left, &cols_right];

  for slice in slices {
    if (visible == 0 && !slice.iter().any(|&x| x >= tree)) {
      visible += 1;
    };

    // count from start to end of slice
    let mut counter = 0;
    for num in slice {
      counter += 1;
      if *num >= tree {
        break;
      }
    }
    scores.push(counter);
  };

  for score in scores {
    if score != 0 {
      final_score *= score;
    }
  }

  return (visible, final_score);
}

fn count_trees(trees: &Vec<Vec<u32>>) -> (u32, u32) {
  let length: usize = trees.len();
  let width: usize = trees[0].len();
  let mut best_score = 0;
  // add up all the trees on the edges
  let mut visible = (length * 2 + (width - 2) * 2) as u32;

for (idx, _) in trees.iter().enumerate() {
  if idx == 0 || idx == length-1 {
    continue;
  }
  for (jdx, _) in trees[idx].iter().enumerate() {
    if jdx == 0 || jdx == width-1 {
      continue;
    }
    let resp = check_height((idx, jdx), trees);
    visible += resp.0;
    if resp.1 > best_score {
      best_score = resp.1;
    }
  }
}

  (visible, best_score)
}
fn main() {
    let file = File::open("src/input.txt").expect("file not found");

    let mut buf = BufReader::new(file);
    let mut contents = String::new();
    buf.read_to_string(&mut contents).expect("msg");

    let mut trees: Vec<Vec<u32>> = Vec::new();

    let split_lines: Vec<&str> = contents.split("\n").collect();

    for line in split_lines {
      let chars = line.chars();
      let mut row: Vec<u32> = Vec::new();
      for char in chars {
        row.push(char.to_digit(10).unwrap());
      }
      trees.push(row);
    }

    let resp = count_trees(&trees);

    println!("{:?} {}", resp.0, resp.1);
}
