use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::fmt::{Formatter,Debug,Error};


fn main() {
  println!("Advent of Code Day 14");
  println!("");
  println!("http://adventofcode.com/2017/day/14");
  println!("");

  let mut file = File::open("data/day14/input").expect("file not found");
  let mut content = String::new();
  file.read_to_string(&mut content).expect("Something went wrong reading input file");

  part1(&content);
  part2(&content);
}

fn part1(input: &String) {
  let mut blocks = vec![];
  let source = input.trim().to_owned();
  for i in 0..128 {
    let mut d = source.to_owned();
    d.push_str("-");
    d.push_str(&i.to_string());

    blocks.push(Circular::hash(&d));
  }

  let mut sum = 0;
  for block in blocks {
      for cell in block {
          sum += cell.count_ones();
      }
  }

  println!("Part 1: {}", sum);
}


fn part2(input: &String) {
  let mut bb = BitBoard::from_hashes(&input);
  println!("Part 2: {}", bb.count_regions());
}

fn parse_as<T : FromStr>(input: &String) -> T {
  let parsed : Result<T, T::Err> = input.parse();
  match parsed {
    Ok(d) => return d,
    Err(_) => panic!("Failed to parse {}", input)
  }
}

#[derive(PartialEq, Eq)]
struct BitBoard {
    content: Vec<Vec<u8>>
}

impl BitBoard {
  pub fn from_hashes(input: &String) -> BitBoard {
    let mut blocks = vec![];
    let source = input.trim().to_owned();
    for i in 0..128 {
        let mut d = source.to_owned();
        d.push_str("-");
        d.push_str(&i.to_string());

        blocks.push(Circular::hash(&d));
    }

    return BitBoard { content: blocks }
  }

  pub fn live_squares(&self) -> u32 {
    let mut sum = 0;
    for block in &self.content {
        for cell in block {
            sum += cell.count_ones();
        }
    }
    return sum;
  }

  // Coord system is in the fourth quadrant because it's easier that way.
  // That means that low y values = top of field, high = bottom.
  pub fn is_set(&self, x: usize, y: usize) -> bool {
    let chunk = x / 8;

    //let shift_amt : u32 = (x % 8) as u32;
    //let (bit, _) = 128u8.overflowing_shr(shift_amt);

    let bit: u8;  // this value will never be used.

    if y >= self.content.len() { return false; }
    if chunk >= self.content[y].len() { return false; }

    if      x % 8 == 0 { bit = 128u8; }
    else if x % 8 == 1 { bit = 64u8; }
    else if x % 8 == 2 { bit = 32u8; }
    else if x % 8 == 3 { bit = 16u8; }
    else if x % 8 == 4 { bit = 8u8; }
    else if x % 8 == 5 { bit = 4u8; }
    else if x % 8 == 6 { bit = 2u8; }
    else if x % 8 == 7 { bit = 1u8; }
    else { bit = 0; } // this value will never be used, but the compiler gets grumpy if I don't provide it


    let edit = self.content[y][chunk];
    return (edit & bit) > 0;
  }

  // flips the bit specified if it's in range, nothing otherwise
  pub fn flip(&mut self, x: usize, y: usize) {
    let chunk = x / 8;
    //let shift_amt : u32 = (x % 8) as u32;

    if y >= self.content.len() { return; }
    if chunk >= self.content[y].len() { return; }

    let bit: u8;

    if      x % 8 == 0 { bit = 128u8; }
    else if x % 8 == 1 { bit = 64u8; }
    else if x % 8 == 2 { bit = 32u8; }
    else if x % 8 == 3 { bit = 16u8; }
    else if x % 8 == 4 { bit = 8u8; }
    else if x % 8 == 5 { bit = 4u8; }
    else if x % 8 == 6 { bit = 2u8; }
    else if x % 8 == 7 { bit = 1u8; }
    else { bit = 0; } // this value will never be used, but the compiler gets grumpy if I don't provide it

    //let (bit, _) = 128u8.overflowing_shr(shift_amt);


    self.content[y][chunk] ^= bit;
  }

  pub fn is_unset(&self, x: usize, y: usize) -> bool {
    return !self.is_set(x,y);
  }

  pub fn turn_off(&mut self, x: usize, y: usize) {
    if self.is_set(x,y) { self.flip(x,y); }
  }

  pub fn turn_on(&mut self, x: usize, y: usize) {
    if !self.is_set(x,y) { self.flip(x,y); }
  }

  // if the initial point is set, flip the region around it to 0
  // otherwise do nothing
  pub fn flip_region(&mut self, x: usize, y: usize) {
    if self.is_unset(x,y) { return; }
    let mut stack = vec![(x,y)];
    let mut seen = vec![];

    while !stack.is_empty() {
      let (x0,y0) = stack.pop().unwrap(); // unwrap fine because we check for empty in the loop

      seen.push((x0,y0));

      // there's no 'inclusive range' operator, unfortunate.
      let neighbors = vec![(0, 1), (2,1), (1, 0), (1, 2)];
      for (i, j) in neighbors {
        // don't want to underflow
        if x0 + i == 0 { continue; }
        if y0 + j == 0 { continue; }
        let x1 = x0 + i - 1;
        let y1 = y0 + j - 1;

        if self.is_set(x1, y1) && !seen.contains(&(x1,y1)) {
            stack.push((x1, y1));
        }
      }

      self.turn_off(x0, y0);
    }
  }


  // this is one-way
  pub fn count_regions(&mut self) -> u32 {
    let mut sum = 0;
    //let mut copy_self = self.to_owned();

    let row_count = self.content.len();

    let mut i = 0;

    for row in self.content.to_owned() {
      let mut offset = 0;

      for chunk in row {

        for j in 0..8 {
          if self.is_set(j + 8*offset, i) {
              sum += 1;
              self.flip_region(j + 8*offset, i);
          }
        }

        offset += 1;
      }

      i += 1;
    }

    return sum;
  }
}

impl Debug for BitBoard {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f);
        for block in &self.content {
            for cell in block {
                write!(f, "{:08b}", cell);
            }
            writeln!(f);
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod bitboard_test {
  use super::*;

  #[test]
  fn is_set() {
    // something like
    // 01000000
    // 00000000
    // 00100000
    // 00000000
    let bb = BitBoard { content: vec![
        vec![64],
        vec![0],
        vec![32],
        vec![0]
    ]};

    assert!(bb.is_set(1,0));
    assert!(!bb.is_set(4,1));
    assert!(bb.is_set(2,2));
    assert!(!bb.is_set(3,3));
    assert!(!bb.is_set(3,103));
  }

  #[test]
  fn flip() {
    // something like
    // 01000000
    // 00000000
    // 00100000
    // 00000000
    let mut bb = BitBoard { content: vec![
        vec![64],
        vec![0],
        vec![32],
        vec![0]
    ]};

    assert!(bb.is_set(2,2));
    bb.flip(2,2);
    assert!(!bb.is_set(2,2));
    bb.flip(2,2);
    assert!(bb.is_set(2,2));


  }

  #[test]
  fn region_large() {
    // something like
    // 11000011 00000001
    // 10000000 10110110
    // 11100000 00000000
    // 00000000 11111111
    let bb = BitBoard { content: vec![
        vec![195 , 1],
        vec![128, 182],
        vec![224, 0],
        vec![0, 255]
    ]};


    assert!(bb.is_set(1,0));
    assert!(bb.is_set(2,2));
    assert!(bb.is_set(6,0));
    assert!(bb.is_set(7,0));

    assert!(bb.is_set(15,0));
    assert!(bb.is_set(8,1));
    assert!(!bb.is_set(8,2));
    assert!(bb.is_set(11,3));
  }

  #[test]
  fn region_count() {
    // something like
    // 11000011 00000001
    // 10000000 11111111
    // 11100000 00000000
    // 00000000 11111111
    let mut bb = BitBoard { content: vec![
        vec![195 , 1],
        vec![128, 255],
        vec![224, 0],
        vec![0, 255]
    ]};

    assert_eq!(bb.count_regions(), 4);
  }

  #[test]
  fn all_possible_is_set_poistions() {
    let bb = BitBoard { content: vec![
        vec![128, 0],
        vec![64, 0],
        vec![32, 0],
        vec![16, 0],
        vec![8, 0],
        vec![4, 0],
        vec![2, 0],
        vec![1, 0],
        vec![0, 128],
        vec![0, 64],
        vec![0, 32],
        vec![0, 16],
        vec![0, 8],
        vec![0, 4],
        vec![0, 2],
        vec![0, 1]
    ]};

    for i in 0..16 {
      assert!(bb.is_set(i,i));
    }
  }

  #[test]
  fn flip_region() {
    // something like
    // 01100010
    // 01000000
    // 01110000
    // 00000000
    let mut bb = BitBoard { content: vec![
        vec![98],
        vec![64],
        vec![112],
        vec![0]
    ]};

    println!("{:?}", bb);

    assert!(bb.is_set(6,0)); // isolated point

    assert!(bb.is_set(3,2)); // lower bar
    assert!(bb.is_set(2,2)); // lower bar
    assert!(bb.is_set(1,2)); // lower bar

    assert!(bb.is_set(1,1)); // center

    assert!(bb.is_set(1,0)); // upper bar
    assert!(bb.is_set(2,0)); // upper bar

    bb.flip_region(1,1);
    println!("{:?}", bb);


    assert!(bb.is_set(6,0)); // isolated point still set

    assert!(!bb.is_set(3,2)); // lower bar unset
    assert!(!bb.is_set(2,2)); // lower bar unset
    assert!(!bb.is_set(1,2)); // lower bar unset

    assert!(!bb.is_set(1,1)); // upper bar unset

    assert!(!bb.is_set(1,0)); // upper bar unset
    assert!(!bb.is_set(2,0)); // upper bar unset
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

    pub fn hash(input: &String) -> Vec<u8> {
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

        return c.condense();
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

    fn read_test_file(filename: &str) -> String {
        let mut file = File::open(filename).expect("file not found");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Something went wrong reading input file");

        return String::from(content.trim());
    }

    #[test]
    fn day14_test() {
        let mut blocks = vec![];
        let input = read_test_file("data/day14/test");
        for i in 0..128 {
            let mut d = input.to_owned();
            d.push_str("-");
            d.push_str(&i.to_string());

            blocks.push(Circular::hash(&d));
        }

        let mut sum = 0;
        for block in blocks {
            for cell in block {
                sum += cell.count_ones();
            }
        }

        assert_eq!(sum, 8108);
    }


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
