use std::io::Read;
use std::fs::File;
use std::io::BufReader;

fn main() {
  let file = File::open("src/input.txt").expect("file not found");

  let mut buf = BufReader::new(file);
  let mut contents = String::new();
  buf.read_to_string(&mut contents).expect("msg");


  let split_lines: Vec<&str> = contents.split("\n").collect();
  let mut totals: Vec<u32> = Vec::new();

  let mut current = 0;
  for line in split_lines {

    if line == "" {
     totals.push(current);
     current = 0;
     continue;
    }
    current += line.parse::<u32>().expect("error parsing line ");
  }

  totals.push(current);

  totals.sort_by(|a, b| b.cmp(a));

  let top_three: u32 = totals[..3].iter().sum();

  println!("{:?}", top_three);
}
