use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

#[derive(PartialEq)]
pub struct TreeNode {
  pub value: Option<u32>,
  pub name: Option<String>,
  pub children: Vec<Rc<RefCell<TreeNode>>>,
  pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  pub fn new() -> TreeNode {
    return TreeNode {
      value: None,
      name: None,
      children: vec![],
      parent: None,
    };
  }

  pub fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
    self.children.push(new_node);
  }

  pub fn sum_children(&self, mut sum: u32) -> u32 {
    // if the self has a value
    if let Some(value) = self.value {
      return value
    } else {
      for child in self.children.iter() {
        sum += child.borrow().sum_children(0);
      }
    }


    return sum
  }

  pub fn bfs(&self) -> Vec<u32> {
    let mut stack: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    let mut space_vec: Vec<u32> = Vec::new();

    for child in self.children.iter() {
      if child.borrow().value.is_none() {
        stack.push_back(Rc::clone(&child));
      }
    }

    while !stack.is_empty() {
      let p = stack.pop_front().unwrap();
      
      let sub_total = p.borrow().sum_children(0);

      space_vec.push(sub_total);

      for child in p.borrow().children.iter() {
        if child.borrow().value.is_none() {
          stack.push_back(Rc::clone(&child));
        }
      }
    }
    
    space_vec.sort();
    space_vec
  }

  pub fn print(&self) -> String {
    if let Some(value) = self.value {
      return value.to_string();
    } else {
      return String::from("[")
        + &self
          .children
          .iter()
          .map(|tn| tn.borrow().print())
          .collect::<Vec<String>>()
          .join(",")
        + "]";
    }
  }
}