//extern crate regex;
//#[macro_use] extern crate lazy_static;

//use regex::Regex;

use std::fs::File;
use std::io::prelude::*;
//use std::collections::HashMap;
//use std::collections::hash_map::Keys;
//use std::hash::Hash;

fn main() {
  let mut file = File::open("data/day9/input").expect("file not found");
  let mut content = String::new();
  file.read_to_string(&mut content).expect("Something went wrong reading input file");

  let (total, total_garbage) = solve(&content);
  println!("Part 1: {}", total);
  println!("Part 2: {}", total_garbage);
}

fn solve(input: &String) -> (i32, i32) {
  let source = input.trim().to_owned();
  let mut stack_a : Vec<char> = source.chars().collect();
  let mut stack_b : Vec<char> = vec![];

  // Make sequence cancel-free
  while stack_a.len() != 0 {
    let ptr = stack_a.remove(0);
    if ptr == '!' {
      stack_a.remove(0); // burn the next character, discard both
    } else {
      stack_b.push(ptr); // keep the character
    }
  }

  stack_a.clear();

  // make sequence garbage free
  let mut garbage_mode : bool = false;
  let mut total_garbage_chars = 0;
  while stack_b.len() != 0 {
    let ptr = stack_b.remove(0);
    if garbage_mode {
      if ptr == '>' { 
        garbage_mode = false; // if we see the end of garbage, reset the flag.
      }
      else {
        total_garbage_chars += 1;
      }
    } else {
      if ptr == '<' { // if we see the beginning of garbage, set the flag.
        garbage_mode = true;
      } else {
        stack_a.push(ptr);
      }
    }
  }

  stack_b.clear();

  // now count the groups, since all the garbage is removed.
  let mut total = 0;
  let mut depth = 0;
  while stack_a.len() != 0 {
    let ptr = stack_a.remove(0);
    if ptr == '{' {
      depth += 1;
    } else if ptr == '}' {
      total += depth;
      depth -= 1;
    } else if ptr == ',' || ptr == '\n' || ptr == '\r' {
      // do nothing, these mean nothing.
    } else {
      panic!("Found garbage character: `{}' in string `{}' ", ptr, input);
    }
  }

  return (total, total_garbage_chars);
}

#[cfg(test)]
mod tests {
  use super::*;

  fn read_test_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("file not found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Something went wrong reading input file");

    return content;
  }


  #[test]
  fn test_1() {
    let content = read_test_file("data/day9/test1");
    assert_eq!(solve(&content), (1,0));
  }

  #[test]
  fn test_2() {
    let content = read_test_file("data/day9/test2");
    assert_eq!(solve(&content), (6,0));
  }

  #[test]
  fn test_3() {
    let content = read_test_file("data/day9/test3");
    assert_eq!(solve(&content), (5,0));
  }

  #[test]
  fn test_4() {
    let content = read_test_file("data/day9/test4");
    assert_eq!(solve(&content), (16,0));
  }

  #[test]
  fn test_5() {
    let content = read_test_file("data/day9/test5");
    assert_eq!(solve(&content), (1,4));
  }

  #[test]
  fn test_6() {
    let content = read_test_file("data/day9/test6");
    assert_eq!(solve(&content), (9,8));
  }

  #[test]
  fn test_7() {
    let content = read_test_file("data/day9/test7");
    assert_eq!(solve(&content), (9,0));
  }

  #[test]
  fn test_8() {
    let content = read_test_file("data/day9/test8");
    assert_eq!(solve(&content), (3,17));
  }
}
