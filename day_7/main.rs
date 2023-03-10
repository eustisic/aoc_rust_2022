#![allow(unused)]
use core::num;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufReader, Read};
use std::rc::Rc;
mod tree;
use tree::TreeNode;

const MAX_SIZE: u32 = 70000000;
const UPDATE_SIZE: u32 = 30000000;

fn parse_command(vector: &Vec<&str>) -> Rc<RefCell<TreeNode>> {
    let root = Rc::new(RefCell::new(TreeNode::new()));
    let mut current_node = Rc::clone(&root);
    for line in vector.iter() {
        let mut split: Vec<&str> = line.split_whitespace().collect();
        if split[0] == "$" {
            // command rather than file
            if split[1] == "cd" {
                let current_clone = Rc::clone(&current_node);
                if split[2] == ".." {
                    // change to parent
                    current_node = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                } else {
                    let name = split[2].to_string();
                    if name == "/" {
                        continue;
                    }
                    // change referenced tree node
                    // iterate through children to find name of directory
                    current_node = Rc::clone(
                        current_clone
                            .borrow()
                            .children
                            .iter()
                            .find(|child| {
                                let x = match child.borrow().name.as_ref() {
                                    Some(val) => { 
                                      *val == name
                                    },
                                    None => false,
                                };
                                x
                            })
                            .unwrap(),
                    )
                }
            } else {
                // ls command do nothing
                continue;
            }
        } else {
            let child = Rc::new(RefCell::new(TreeNode::new()));
            current_node.borrow_mut().add_child(Rc::clone(&child));
            let mut mut_child = child.borrow_mut();
            mut_child.name = Some(String::from(split[1]));
            // file
            if split[0] == "dir" {
                mut_child.parent = Some::<Rc<RefCell<TreeNode>>>(Rc::clone(&current_node));
            } else {
                mut_child.value = Some::<u32>(split[0].parse::<u32>().unwrap());
            }
        }
    }

    return root;
}

fn find_closest(numbers: Vec<u32>, target: u32) {
  let mut end = numbers.len() - 1;
  let mut closest: u32 = *numbers.get(end).expect("no value");
  
  while end > 0 {
    let current = numbers[end];
    if current > target && current < closest {
      closest = current;
    }
    end -= 1;
  }

  println!("closest: {}", closest);

}

fn main() {
    let file = File::open("src/input.txt").expect("file not found");

    let mut buf = BufReader::new(file);
    let mut contents = String::new();
    buf.read_to_string(&mut contents);

    let split_lines: Vec<&str> = contents.split("\n").collect();

    let tree = parse_command(&split_lines);

    let total: u32 = tree.borrow().sum_children(0);
    let free_space = MAX_SIZE - total;
    let mut sum: u32 = 0;
    let space_array = tree.borrow().bfs();
    let soln_2 = 0;

    for i in space_array.iter() {
      if *i < 100000 {
        sum += *i
      }
    }
    println!("total {}, free_space {}, solution1 {}", total, free_space, sum);
    find_closest(space_array, UPDATE_SIZE - free_space);
}
