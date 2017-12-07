use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
  println!("Advent of Code Day 6");
  println!("");
  println!("http://adventofcode.com/2017/day/5");
  println!("");

  let mut file = File::open("data/day6/input").expect("file not found");
  let mut content = String::new();
  file.read_to_string(&mut content).expect("Something went wrong reading input file");

  println!("Part 1: {}", part1(&content));
  println!("Part 2: {}", part2(&content));
}

fn part1(input: &String) -> i32 {
  return MemoryBank::new(input, Mode::Part1).run();
}

fn part2(input: &String) -> i32 {
  return MemoryBank::new(input, Mode::Part2).run();
}


#[derive(Debug, PartialEq, Eq)]
enum Mode {
  Part1,
  Part2
}

#[derive(Debug, PartialEq, Eq)]
struct MemoryBank {
  source: String,
  history: HashSet<Vec<i32>>,
  blocks: Vec<i32>,
  mode: Mode,
  steps: i32,
  loop_found: bool
}

impl MemoryBank {
  pub fn new(source: &String, mode: Mode) -> MemoryBank {
    return MemoryBank {
      source: source.to_owned(),
      history: HashSet::new(),
      blocks: vec![],
      mode: mode,
      steps: 0,
      loop_found: false
    };
  }

  pub fn run(&mut self) -> i32 {
    self.reboot();

    self.execution_loop();

    if self.mode == Mode::Part2 {
      self.steps = 0;        // reset to count the total steps in the loop
      self.clear_history();   // wipe history
      self.record_history();  // but populate it with state so we fail quick
      self.execution_loop();  // rerun
    }

    return self.steps
  }

  fn execution_loop(&mut self) {
    while !self.loop_found {
      self.step();
      self.steps += 1;
      self.record_history();
    }
  }

  fn step(&mut self) {
    let start_idx = self.largest_block_id();
    let memory = self.blocks[start_idx];
    let len = self.blocks.len();

    self.blocks[start_idx] = 0;
    for offset in 0..memory {
      self.blocks[(start_idx + (offset as usize) + 1) % len] += 1;
    }
  }

  fn seen(&self, entry: &Vec<i32>) -> bool {
    return self.history.get(entry).is_some();
  }

  fn record_history(&mut self) {
    let entry = self.blocks.to_owned();
    self.loop_found = self.seen(&entry);
    self.history.insert(entry.to_owned());
  }

  fn largest_block_id(&self) -> usize {
    let max = self.largest_block_allocation();

    for idx in 0..self.blocks.len() {
      if self.blocks[idx] == max {
        return idx;
      }
    }
    panic!("Cannot occur!");
  }

  fn largest_block_allocation(&self) -> i32 {
    match self.blocks.iter().max() {
      Some(v) => return v.to_owned(),
      None => panic!("No max value found!")
    }
  }

  fn reboot(&mut self) {
    self.blocks = vec![];
    self.clear_history();
    self.steps = 0;
    self.loop_found = false;
    self.parse_source();
  }

  fn clear_history(&mut self) {
    self.history.clear();
  }

  fn parse_source(&mut self) {
    for item in self.source.split_whitespace() {
      let parsed : Result<i32, std::num::ParseIntError> = item.parse();
      match parsed {
        Ok(d) => self.blocks.push(d),
        Err(e) => panic!("Memory failed to parse {} as a number with error: {}", item, e)
      }
    }
  }

}

#[test]
fn part1_test() {
  let mut file = File::open("data/day6/test").expect("file not found");
  let mut content = String::new();
  file.read_to_string(&mut content).expect("Something went wrong reading test file");

  assert_eq!(MemoryBank::new(&content, Mode::Part1).run(), 5);
}

#[test]
fn part2_test() {
  let mut file = File::open("data/day5/test").expect("file not found");
  let mut content = String::new();
  file.read_to_string(&mut content).expect("Something went wrong reading test file");

  assert_eq!(MemoryBank::new(&content, Mode::Part2).run(), 4);
}
