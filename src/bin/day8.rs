extern crate regex;
#[macro_use] extern crate lazy_static;

use regex::Regex;

use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::hash_map::Keys;
use std::hash::Hash;

fn main() {
  let mut file = File::open("data/day8/input").expect("file not found");
  let mut content = String::new();
  file.read_to_string(&mut content).expect("Something went wrong reading input file");

  let mut rt = RegisterMachine::new(&content);
  rt.run();
  println!("Part 1: {}, {}", rt.largest_register(), rt.largest_register_value());
  println!("Part 2: {}", rt.high_mem);
}

fn parse_as<T : FromStr>(input: &String) -> T {
  let parsed : Result<T, T::Err> = input.parse();
  match parsed {
    Ok(d) => return d,
    Err(_) => panic!("Failed to parse {}", input)
  }
}



#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Op {
  INC(i32),
  DEC(i32),
}

type Register = String;

#[derive(Debug, PartialEq, Eq, Clone)]
enum ConditionOp {
  GT(Register,i32),
  LT(Register,i32),
  EQ(Register,i32),
  NEQ(Register,i32),
  GTEQ(Register,i32),
  LTEQ(Register,i32)
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
  target: Register,
  op: Op,
  condition: ConditionOp
}

impl Instruction {
  fn parse(input: &String) -> Instruction {
    lazy_static! {
      static ref PARSER: Regex = Regex::new(r"([a-z]+) ((?:inc)|(?:dec)) (-?\d+) if ([a-z]+) ([<>=!]+) (-?\d+)").unwrap(); 
    }
    let parsed = PARSER.captures(input).unwrap();

    let register = &parsed[1];
    let opcode;
    let op_amt: i32 = parse_as::<i32>(&String::from(&parsed[3]));

    if &parsed[2] == "inc" {
      opcode = Op::INC(op_amt); 
    } else {
      opcode = Op::DEC(op_amt);
    }

    let cond_register = String::from(&parsed[4]);
    let cond_amt: i32 = parse_as::<i32>(&String::from(&parsed[6]));
    let condition;

    match &parsed[5] {
      ">" => condition = ConditionOp::GT(cond_register, cond_amt),
      "<" => condition = ConditionOp::LT(cond_register, cond_amt),
      ">=" => condition = ConditionOp::GTEQ(cond_register, cond_amt),
      "<=" => condition = ConditionOp::LTEQ(cond_register, cond_amt),
      "==" => condition = ConditionOp::EQ(cond_register, cond_amt),
      "!=" => condition = ConditionOp::NEQ(cond_register, cond_amt),
      e => panic!("Unrecognized condition operation: ``{}''", e)
    }

    return Instruction {
      target: String::from(register),
      op: opcode,
      condition: condition
    };
  }
}

#[cfg(test)]
mod instruction_tests {
  use super::*;

  #[test]
  fn test_parser() {
    let s = String::from("a inc 1 if b <= 2");
    let i = Instruction::parse(&s);
    assert_eq!(i.target, String::from("a"));
    assert_eq!(i.op, Op::INC(1));
    assert_eq!(i.condition, ConditionOp::LTEQ(String::from("b"), 2));
  }

  #[test]
  fn test_sanity() {
    let r = Regex::new(r"((?:inc)|(?:dec))").unwrap(); 
    assert!(r.is_match("inc"));
    assert!(r.is_match("dec"));
    assert!(!r.is_match("asdf"));

  }
}

#[derive(Debug)]
struct HashMapWithDefault<K,V> 
  where K: Hash + PartialEq + Eq {
  hash: HashMap<K,V>,
  default: V
}

impl<K: PartialEq + Eq + Hash, V> HashMapWithDefault<K,V> {
  pub fn new(default: V) -> HashMapWithDefault<K,V> {
    return HashMapWithDefault { hash: HashMap::new(), default: default };
  }

  pub fn insert(&mut self, key: K, value: V) {
    self.hash.insert(key, value);
  }

  pub fn get(&self, key: K) -> &V {
    return self.hash.get(&key).unwrap_or(&self.default);
  }

  pub fn keys(&self) -> Keys<K,V> {
    return self.hash.keys();
  }

  pub fn clear(&mut self) {
    self.hash.clear();
  }
}


#[derive(Debug)]
struct RegisterMachine {
  source: String,
  instructions: Vec<Instruction>,
  registers: HashMapWithDefault<String, i32>,
  high_mem: i32
}


impl RegisterMachine {
  pub fn new(input: &String) -> RegisterMachine {
    let mut r = RegisterMachine { source: input.to_owned(), instructions: vec![], registers: HashMapWithDefault::new(0), high_mem: 0 };
    r.reboot();
    return r;
  }

  pub fn run(&mut self) {
    for i in &self.instructions {
      let cond_status: bool;
      let condition = i.condition.to_owned();
      match condition {
        ConditionOp::GT(reg, amt) => cond_status = self.register_value(&reg) > amt,
        ConditionOp::LT(reg, amt) => cond_status = self.register_value(&reg) < amt,
        ConditionOp::GTEQ(reg, amt) => cond_status = self.register_value(&reg) >= amt,
        ConditionOp::LTEQ(reg, amt) => cond_status = self.register_value(&reg) <= amt,
        ConditionOp::EQ(reg, amt) => cond_status = self.register_value(&reg) == amt,
        ConditionOp::NEQ(reg, amt) => cond_status = self.register_value(&reg) != amt
      }

      if !cond_status { continue; }

      let target = i.target.to_owned();
      let current_value = self.register_value(&target);

      match i.op {
        Op::INC(amt) => { self.registers.insert(target, current_value + amt); }
        Op::DEC(amt) => { self.registers.insert(target, current_value - amt); }
      }

      let high = self.largest_register_value();
      if high > self.high_mem {
        self.high_mem = high;
      }
    }
  }

  pub fn largest_register_value(&self) -> i32 {
    return self.register_value(&self.largest_register());
  }

  pub fn largest_register(&self) -> String {
    let mut max = 0;
    let mut reg_name = String::new();
    for key in self.register_names() {
      let value = self.register_value(key);
      if value > max {
        max = value;
        reg_name = key.to_owned();
      }
    }

    return reg_name;
  }

  fn register_value(&self, name: &String) -> i32 {
    return *self.registers.get(name.to_owned());
  }

  fn register_names(&self) -> Keys<String,i32> {
    return self.registers.keys();
  }

  fn reboot(&mut self) {
    self.instructions = vec![];
    self.registers.clear();
    self.parse();
  }

  // parse source to instruction objects
  fn parse(&mut self) {
    for line in self.source.lines() {
      let i = Instruction::parse(&String::from(line));
      self.instructions.push(i);
    }
  }
}


#[cfg(test)]
mod register_machine_tests {
  use super::*;

  #[test]
  fn part1_ex() {
    let mut file = File::open("data/day8/test").expect("file not found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Something went wrong reading test file");

    let mut rt = RegisterMachine::new(&content);
    rt.run();
    assert_eq!(String::from("a"), rt.largest_register());
    assert_eq!(1, rt.largest_register_value());

  }

  #[test]
  fn part2_ex() {
    let mut file = File::open("data/day8/test").expect("file not found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Something went wrong reading test file");

    let mut rt = RegisterMachine::new(&content);
    rt.run();
    assert_eq!(String::from("a"), rt.largest_register());
    assert_eq!(1, rt.largest_register_value());
    assert_eq!(10, rt.high_mem);

  }
}


