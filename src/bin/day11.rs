use std::fs::File;
use std::io::prelude::*;
use std::cmp;

fn main() {
  println!("Advent of Code Day 11");
  println!("");
  println!("http://adventofcode.com/2017/day/11");
  println!("");

  let mut file = File::open("data/day11/input").expect("file not found");
  let mut content = String::new();
  file.read_to_string(&mut content).expect("Something went wrong reading input file");

  solve(&content);
}

fn solve(input: &String) {
  let mut hp = HexPointer::at_origin();
  for dir in input.trim().split(',') {
    hp.step(HexDirection::parse(dir)); 
  }
  println!("Part 1: {}", hp.distance_to_origin());
  println!("Part 2: {}", hp.max_dist);
}

// see http://keekerdc.com/2011/03/hexagon-grids-coordinate-systems-and-distance-calculations/
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct HexPointer {
  x: i32,
  y: i32,
  z: i32,
  max_dist: i32
}

enum HexDirection {
  N,
  S,
  NE,
  NW,
  SE,
  SW
}

impl HexDirection {
  //   \ n  /
  // nw +--+ ne
  //   /    \
  // -+      +-
  //   \    /
  // sw +--+ se
  //   / s  \
  pub fn to_vector(self) -> (i32, i32, i32) {
    match self {
      HexDirection::N => (0,1,-1),
      HexDirection::S => (0,-1,1),
      HexDirection::NE => (1,0,-1),
      HexDirection::NW => (-1,1,0),
      HexDirection::SE => (1,-1,0),
      HexDirection::SW => (-1,0,1)
    }
  }

  pub fn parse(dir: &str) -> HexDirection {
    if dir == "n" {
      return HexDirection::N;
    } else if dir == "s" {
      return HexDirection::S;
    } else if dir == "ne" {
      return HexDirection::NE;
    } else if dir == "nw" {
      return HexDirection::NW;
    } else if dir == "se" {
      return HexDirection::SE;
    } else if dir =="sw" {
      return HexDirection::SW;
    } else {
      panic!("Unrecognized direction: ``{}''", dir);
    }
  }
}

impl HexPointer {
  pub fn at_origin() -> HexPointer {
    return HexPointer { x: 0, y: 0, z: 0, max_dist: 0 };
  }


  pub fn step(&mut self, dir: HexDirection) {
    let (x1,y1,z1) = dir.to_vector();
    self.x += x1;
    self.y += y1;
    self.z += z1;

    if self.distance_to_origin() > self.max_dist {
      self.max_dist = self.distance_to_origin();
    }
  }

  pub fn distance_to_origin(&mut self) -> i32 {
    let c1 = cmp::max(self.x.abs(), self.y.abs());
    return cmp::max(c1, self.z.abs());
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ex1() {
    let mut hp = HexPointer::at_origin();
    hp.step(HexDirection::NE);
    hp.step(HexDirection::NE);
    hp.step(HexDirection::NE);
    assert_eq!(hp.distance_to_origin(), 3);
  }
  #[test]
  fn test_ex2() {
    let mut hp = HexPointer::at_origin();
    hp.step(HexDirection::NE);
    hp.step(HexDirection::NE);
    hp.step(HexDirection::SW);
    hp.step(HexDirection::SW);
    assert_eq!(hp.distance_to_origin(), 0);
  }
  #[test]
  fn test_ex3() {
    let mut hp = HexPointer::at_origin();
    hp.step(HexDirection::NE);
    hp.step(HexDirection::NE);
    hp.step(HexDirection::S);
    hp.step(HexDirection::S);
    assert_eq!(hp.distance_to_origin(), 2);
  }
  #[test]
  fn test_ex4() {
    let mut hp = HexPointer::at_origin();
    hp.step(HexDirection::SE);
    hp.step(HexDirection::SW);
    hp.step(HexDirection::SE);
    hp.step(HexDirection::SW);
    hp.step(HexDirection::SW);

    assert_eq!(hp.distance_to_origin(), 3);
  }
}
