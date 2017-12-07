use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
  println!("Advent of Code Day 4");
  println!("");
  println!("Spreadsheet: http://adventofcode.com/4017/day/2");
  println!("");

  let mut file = File::open("data/day4/input").expect("file not found");
  let mut content = String::new();
  file.read_to_string(&mut content).expect("Something went wrong reading input file");

  println!("Part1: {}", part1(&content));
  println!("Part2: {}", part2(&content));

}


fn part1(input: &String) -> i32 {
  let mut invalid = 0;
  let mut total = 0;
  let mut hash: HashMap<String, i32>;

  for line in input.lines() {
    total += 1;
    hash = HashMap::new();

    for word in line.split_whitespace() {
      let wordcount: i32;

      match hash.get(word) {
        Some(&v) => wordcount = v,
        None => wordcount = 0
      }
      hash.insert(String::from(word), wordcount + 1);
    }

    for key in hash.keys() {
      if *hash.get(key).expect("") > 1 {
        invalid += 1;
        break;
      }
    }
  }

  return total - invalid;
}

fn part2(input: &String) -> i32 {
  let mut invalid = 0;
  let mut total = 0;
  let mut hash: HashMap<String, i32>;

  for line in input.lines() {
    total += 1;
    hash = HashMap::new();

    for word in line.split_whitespace() {
      let wordcount: i32;
      let sorted_word = sort_string(String::from(word));

      match hash.get(&sorted_word) {
        Some(&v) => wordcount = v,
        None => wordcount = 0
      }
      hash.insert(sorted_word, wordcount + 1);
    }

    for key in hash.keys() {
      if *hash.get(key).expect("") > 1 {
        invalid += 1;
        break;
      }
    }
  }

  return total - invalid;
}

fn sort_string(input: String) -> String {
  let mut chars: Vec<char> = input.chars().collect();

  chars.sort_by(|a,b| a.cmp(b));

  return chars.into_iter().collect();
}

#[test]
fn test_part1() {
  let mut file = File::open("data/day4/test").expect("file not found");
  let mut content = String::new();
  file.read_to_string(&mut content).expect("Something went wrong reading test file");

  assert_eq!(part1(&content), 2);
}

#[test]
fn test_part2() {
  let mut file = File::open("data/day4/test2").expect("file not found");
  let mut content = String::new();
  file.read_to_string(&mut content).expect("Something went wrong reading test file");

  assert_eq!(part2(&content), 3);
}

#[test]
fn test_sort_string() {
  assert_eq!(sort_string(String::from("bdca")), "abcd");
  assert_eq!(sort_string(String::from("dcba")), "abcd");
}
