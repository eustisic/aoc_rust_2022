#![allow(unused_variables)]
use std::io::{Read, BufReader};
use std::fs::File;

/*
            [J]             [B] [W]
            [T]     [W] [F] [R] [Z]
        [Q] [M]     [J] [R] [W] [H]
    [F] [L] [P]     [R] [N] [Z] [G]
[F] [M] [S] [Q]     [M] [P] [S] [C]
[L] [V] [R] [V] [W] [P] [C] [P] [J]
[M] [Z] [V] [S] [S] [V] [Q] [H] [M]
[W] [B] [H] [F] [L] [F] [J] [V] [B]
 1   2   3   4   5   6   7   8   9 
 */

fn parse_and_move(line: &str, cargo: &mut Vec<Vec<char>>) {
  let command: Vec<&str> = line.split(" ").collect();

  let to = command.get(5).unwrap().parse::<usize>().unwrap() - 1;
  let from = command.get(3).unwrap().parse::<usize>().unwrap() - 1;
  let mut amount = command.get(1).unwrap().parse::<i32>().unwrap();

  while amount > 0 {
    let popped = cargo[from].pop().expect("no item found");
    cargo[to].push(popped);
    amount -= 1;
  }
}


fn main() {
  let file = File::open("src/input.txt").expect("no file found");

  let mut buf = BufReader::new(file);
  let mut contents = String::new();
  buf.read_to_string(&mut contents).expect("msg");

  let row_1: Vec<char> = "WMLF".chars().collect();
  let row_2 = "BZVMF".chars().collect();
  let row_3 = "HVRSLQ".chars().collect();
  let row_4 = "FSVQPMTJ".chars().collect();
  let row_5 = "LSW".chars().collect();
  let row_6 = "FVPMRJW".chars().collect();
  let row_7 = "JQCPNRF".chars().collect();
  let row_8 = "VHPSZWRB".chars().collect();
  let row_9 = "BMJCGHZW".chars().collect();

  let mut cargo = vec![row_1, row_2, row_3, row_4, row_5, row_6, row_7, row_8, row_9];

  let split_lines: Vec<&str> = contents.split("\n").collect();

  for line in split_lines {
    parse_and_move(line, &mut cargo);
  }

  let mut answer = String::from("");

  for c in cargo {
    answer.push(c[c.len() - 1]);
  }

  println!("{}", answer);
}
