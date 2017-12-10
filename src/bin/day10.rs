use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

fn main() {
  println!("Advent of Code Day 10");
  println!("");
  println!("http://adventofcode.com/2017/day/10");
  println!("");

  let mut file = File::open("data/day10/input").expect("file not found");
  let mut content = String::new();
  file.read_to_string(&mut content).expect("Something went wrong reading input file");

  part1(&content);
}

fn part1(input: &String) {
  let mut c = Circular::new((0..256).collect());
  for shift_str in input.trim().split(',') {
    let shift = parse_as::<usize>(&String::from(shift_str));
    c.reverse_and_skip(shift);
  }
  println!("Part 1: {}", c.get(0) * c.get(1));
}

fn parse_as<T : FromStr>(input: &String) -> T {
  let parsed : Result<T, T::Err> = input.parse();
  match parsed {
    Ok(d) => return d,
    Err(_) => panic!("Failed to parse {}", input)
  }
}


#[derive(Debug,PartialEq,Eq)]
struct Circular<T> {
  content: Vec<T>,
  skip: usize,
  pointer: usize
}

// every operation will rotate the list such that the next start point pointer is _always_ zero.
impl<T : PartialEq + Eq + Clone> Circular<T> {
  fn new(content: Vec<T>) -> Circular<T> {
    return Circular {
      content: content,
      skip: 0,
      pointer: 0,
    };
  }

  pub fn reverse_and_skip(&mut self, length: usize) {
    let size = self.size();

    for i in 0..(length / 2) {
      let a = i + self.pointer;
      let b = length - i - 1 + self.pointer;
      self.content.swap(a % size, b % size);
    }

    self.pointer = (self.pointer + length + self.skip) % size;
    self.skip += 1;
  }

  pub fn get_pointer(&self) -> T {
    return self.get(self.pointer);
  }

  pub fn get(&self, idx: usize) -> T {
    let result = self.content[idx].to_owned();
    return result;
  }

  fn size(&self) -> usize {
    return self.content.len();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  //fn read_test_file(filename: &str) -> String {
    //let mut file = File::open(filename).expect("file not found");
    //let mut content = String::new();
    //file.read_to_string(&mut content).expect("Something went wrong reading input file");

    //return content;
  //}

  #[test]
  fn test() {
    let mut t = Circular::new(vec![0,1,2,3,4]);
    assert_eq!(t.get(0) * t.get(1), 0);

    t.reverse_and_skip(3);
    assert_eq!(t.content, vec![2,1,0,3,4]);
    assert_eq!(t.get(0) * t.get(1), 2);
    assert_eq!(t.get_pointer(), 3);

    t.reverse_and_skip(4);
    assert_eq!(t.content, vec![4,3,0,1,2]);
    assert_eq!(t.get(0) * t.get(1), 12);
    assert_eq!(t.get_pointer(), 1);

    t.reverse_and_skip(1);
    assert_eq!(t.content, vec![4,3,0,1,2]);
    assert_eq!(t.get(0) * t.get(1), 12);
    assert_eq!(t.get_pointer(), 3);

    t.reverse_and_skip(5);
    assert_eq!(t.content, vec![3,4,2,1,0]);
    assert_eq!(t.get(0) * t.get(1), 12);
    assert_eq!(t.get_pointer(), 0);


  }
}
