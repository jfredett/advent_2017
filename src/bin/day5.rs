use std::fs::File;
use std::io::prelude::*;

fn main() {
  println!("Advent of Code Day 5");
  println!("");
  println!("http://adventofcode.com/5017/day/5");
  println!("");

  let mut file = File::open("data/day5/input").expect("file not found");
  let mut content = String::new();
  file.read_to_string(&mut content).expect("Something went wrong reading input file");

  println!("Part1: {}", part1(&content));
  println!("Part2: {}", part2(&content));
}

fn part1(input: &String) -> i32 {
  return Machine::new(input.to_owned(), Mode::Part1, false).run();
}

fn part2(input: &String) -> i32 {
  return Machine::new(input.to_owned(), Mode::Part2, false).run();
}

#[derive(Debug, PartialEq, Eq)]
enum Mode {
  Part1,
  Part2
}

#[derive(Debug, PartialEq, Eq)]
struct Machine {
  source: String,
  code: Vec<i32>,
  pointer: i32,
  steps: i32,
  mode: Mode,
  debug: bool
}

impl Machine {
  pub fn new(input: String, mode: Mode, debug: bool) -> Machine {
    return Machine {
      source: input,
      code: vec![],
      steps: 0,
      pointer: 0,
      mode: mode,
      debug: debug
    };
  }

  pub fn reboot(&mut self) {
    self.parse_source();
    self.pointer = 0;
    self.steps = 0;
  }

  pub fn display(&self) {
    println!("----");
    for inst in self.code.to_owned() {
      println!("{}", inst);
    }
    println!("----");
  }

  pub fn run(&mut self) -> i32 {
    self.reboot();
    while self.in_bounds() {
      if self.debug {
        self.display()
      }
      self.step();
    }
    return self.steps;
  }

  fn parse_source(&mut self) {
    for line in self.source.lines() {
      let parsed : Result<i32, std::num::ParseIntError> = line.parse();
      match parsed {
        Ok(d) => self.code.push(d),
        Err(e) => panic!("Machine failed to parse {} as a number with error: {}", line, e)
      }
    }
  }

  fn step(&mut self) {
    let previous = self.pointer;

    self.pointer += self.current_instruction();
    self.update_instruction(previous);

    self.steps += 1
  }

  fn update_instruction(&mut self, ptr: i32) {
    match self.mode {
      Mode::Part1 => self.code[ptr as usize] += 1,
      Mode::Part2 => {
        let instruction = self.code[ptr as usize];
        if instruction >= 3 {
          self.code[ptr as usize] -= 1;
        } else {
          self.code[ptr as usize] += 1;
        }
      }
    }
  }


  fn current_instruction(&self) -> i32 {
    return self.code[self.pointer as usize];
  }

  fn upper_bound(&self) -> i32 {
    return self.code.len() as i32;
  }

  fn in_bounds(&self) -> bool {
    return self.pointer >= 0 && self.pointer < self.upper_bound();
  }
}

#[test]
fn part1_test() {
  let mut file = File::open("data/day5/test").expect("file not found");
  let mut content = String::new();
  file.read_to_string(&mut content).expect("Something went wrong reading test file");

  assert_eq!(Machine::new(content, Mode::Part1, true).run(), 5);
}

#[test]
fn part2_test() {
  let mut file = File::open("data/day5/test").expect("file not found");
  let mut content = String::new();
  file.read_to_string(&mut content).expect("Something went wrong reading test file");

  assert_eq!(Machine::new(content, Mode::Part2, false).run(), 10);
}
