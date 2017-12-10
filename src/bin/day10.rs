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
  part2(&content);
}

fn part1(input: &String) {
  let mut c = Circular::new((0..255).collect());
  for shift_str in input.trim().split(',') {
    let shift = parse_as::<usize>(&String::from(shift_str));
    c.reverse_and_skip(shift);
  }
  println!("Part 1: {}", c.get(0) as i32 * c.get(1) as i32);
}


fn part2(input: &String) {
  let mut v = vec![];
  for i in 0..256 {
    v.push(i as u8);
  }
  let mut c = Circular::new(v);
  let mut parsed_input : Vec<u8> = input.trim().bytes().collect();
  let mut appendage : Vec<u8> = vec![17,31,73,47,23];

  parsed_input.append(&mut appendage);

  for _ in 0..64 {
    for shift in parsed_input.to_owned() {
      c.reverse_and_skip(shift as usize);
    }
  }


  println!("Part 2: {:?} -- Convert this to Hex by hand.", c.condense());
}

fn parse_as<T : FromStr>(input: &String) -> T {
  let parsed : Result<T, T::Err> = input.parse();
  match parsed {
    Ok(d) => return d,
    Err(_) => panic!("Failed to parse {}", input)
  }
}


#[derive(Debug,PartialEq,Eq)]
struct Circular {
  content: Vec<u8>,
  skip: usize,
  pointer: usize
}

// every operation will rotate the list such that the next start point pointer is _always_ zero.
impl Circular {
  fn new(content: Vec<u8>) -> Circular {
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

  pub fn condense(&self) -> Vec<u8> {
    let mut out = vec![];
    for i in 0..16 {
      let mut chunk: u8 = 0;
      for j in 0..16 {
        chunk ^= self.content[i*16 + j] as u8;
      }
      out.push(chunk);
    }

    return out;
  }

  pub fn get_pointer(&self) -> u8 {
    return self.get(self.pointer);
  }

  pub fn get(&self, idx: usize) -> u8 {
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
