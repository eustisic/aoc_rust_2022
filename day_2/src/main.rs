use std::collections::HashMap;
use std::io::{Read, BufReader};
use std::fs::File;

fn part_1 (line: &str) -> i32 {
  let answers: HashMap<&str, i32> = HashMap::from([
    ("A X", 4),
    ("A Y", 8),
    ("A Z", 3),
    ("B X", 1),
    ("B Y", 5),
    ("B Z", 9),
    ("C X", 7),
    ("C Y", 2),
    ("C Z", 6),
  ]);

  let answer = answers.get(line).expect("no value found");
  answer.clone()
}

fn part_2(line: &str) -> i32 {
  let answers: HashMap<&str, i32> = HashMap::from([
    ("A X", 3),
    ("A Y", 4),
    ("A Z", 8),
    ("B X", 1),
    ("B Y", 5),
    ("B Z", 9),
    ("C X", 2),
    ("C Y", 6),
    ("C Z", 7),
  ]);

  let answer = answers.get(line).expect("no value found");
  answer.clone()
}
fn main() {
  let file = File::open("src/input.txt").expect("no file found");

  let mut buf = BufReader::new(file);
  let mut contents = String::new();
  buf.read_to_string(&mut contents).expect("msg");

  let split_lines: Vec<&str> = contents.split("\n").collect();

  let mut sum_1 = 0;
  let mut sum_2 = 0;

  for line in split_lines {
    sum_1 += part_1(line);
    sum_2 += part_2(line);
  }

  println!("part 1 {}", sum_1);
  println!("part 2 {}", sum_2);
}