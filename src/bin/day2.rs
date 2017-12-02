use std::fs::File;
use std::io::prelude::*;
use std::ops::Index;

fn main() {
  println!("Advent of Code Day 1");
  println!("");
  println!("Spreadsheet: http://adventofcode.com/2017/day/2");
  println!("");

  let mut part1_spreadsheet_file = File::open("data/day2/input").expect("file not found");
  let mut part1_spreadsheet = String::new();
  part1_spreadsheet_file.read_to_string(&mut part1_spreadsheet).expect("Something went wrong reading part1 file");

  println!("Part1: {}", part1(part1_spreadsheet));

  // there is almost certainly a better way to do this, but this is quick and easy for this purpose
  // avoides the use-after-move thing
  let mut part1_spreadsheet_file = File::open("data/day2/input").expect("file not found");
  let mut part1_spreadsheet = String::new();
  part1_spreadsheet_file.read_to_string(&mut part1_spreadsheet).expect("Something went wrong reading part1 file");

  println!("Part2: {}", part2(part1_spreadsheet));
}

fn part1(input: String) -> i32 {
  let sheet = Spreadsheet::new(input);

  return sheet.part1();
}

fn part2(input: String) -> i32 {
  let sheet = Spreadsheet::new(input);

  return sheet.part2();
}


#[derive(Debug,PartialEq,Eq)]
struct Spreadsheet {
	rows: Vec<SheetRow>,
}

impl Spreadsheet {
  pub fn new(data: String) -> Spreadsheet {
    let mut rows = vec![];
    for line in data.lines() {
      rows.push( SheetRow::new(String::from(line)) );
    }
    return Spreadsheet { rows: rows };
  }

  pub fn part1(&self) -> i32 {
    let mut sum = 0;
    for row in &self.rows {
      sum += row.part1();
    }
    return sum;
  }

  pub fn part2(&self) -> i32 {
    let mut sum = 0;
    for row in &self.rows {
      sum += row.part2();
    }
    return sum;
  }
}

impl Index<usize> for Spreadsheet {
  type Output = SheetRow;

  fn index(&self, index: usize) -> &SheetRow {
    if index > self.rows.len() {
      panic!("{} out of bounds for {:?}", index, self);
    } else {
      return &self.rows[index];
    }
  }
}

#[derive(Debug,PartialEq,Eq)]
struct SheetCell {
	data: i32
}

impl SheetCell {
  pub fn new(data: String) -> SheetCell {
    let parsed : Result<i32, std::num::ParseIntError> = data.parse();
    match parsed {
      Ok(d) => SheetCell::from(d),
      Err(e) => panic!("SheetCell failed to parse {} as a number with error: {}", data, e)
    }
  }

  pub fn from(data: i32) -> SheetCell {
    return SheetCell { data: data };
  }
}

#[derive(Debug,PartialEq,Eq)]
struct SheetRow {
	cells: Vec<SheetCell>
}

impl SheetRow {
  pub fn new(data: String) -> SheetRow {
    let mut cells = vec![];
    for d in data.split_whitespace() {
      cells.push( SheetCell::new(String::from(d)) );
    }
    return SheetRow { cells: cells };
  }

  pub fn part1(&self) -> i32 {
    return self.row_max() - self.row_min();
  }

  pub fn part2(&self) -> i32 {
    for i in 0..(self.len()) {
      for j in 0..(self.len()) {
        let i_data = &self.cells[i].data;
        let j_data = &self.cells[j].data;
        if i_data % j_data == 0 && i != j {
          return i_data / j_data;
        }
      }
    }
    panic!("No divisible value found, this falls outside of the parameters of the problem.");
  }

  pub fn len(&self) -> usize {
    return self.cells.len();
  }

  pub fn row_max(&self) -> i32 {
    let mut max = self.cells.first().expect("").data;
    for cell in &self.cells {
      if cell.data > max {
        max = cell.data;
      }
    }
    return max;
  }

  pub fn row_min(&self) -> i32 {
    let mut min = self.cells.first().expect("").data;
    for cell in &self.cells {
      if cell.data < min {
        min = cell.data;
      }
    }
    return min;
  }
}

impl Index<usize> for SheetRow {
  type Output = SheetCell;

  fn index(&self, index: usize) -> &SheetCell {
    return &self.cells[index];
  }
}

#[cfg(test)]
mod spreadsheet_tests {
  use super::*;

  #[test]
  #[should_panic]
  fn test_spreadsheet_new_panic_1() {
    let sr = Spreadsheet::new(String::from("dummy data goes here\nanother dummy row here"));
    println!("We never get to use this {:?}", sr);
  }

  #[test]
  #[should_panic]
  fn test_spreadsheet_new_panic_2() {
    // bad delimiter
    let sr = Spreadsheet::new(String::from("1 2 3 4_5\n1 2 3 4 5"));
    println!("We never get to use this {:?}", sr);
  }

  #[test]
  fn test_spreadsheet_new_happy_heterogenous_row_length() {
    let sr = Spreadsheet::new(String::from("5 40 2 8 1\n6 41 3 9 2\n3 3 4 4"));
    assert_eq!(sr[0][0], SheetCell::from(5));
    assert_eq!(sr[0][1], SheetCell::from(40));
    assert_eq!(sr[0][2], SheetCell::from(2));
    assert_eq!(sr[0][3], SheetCell::from(8));
    assert_eq!(sr[0][4], SheetCell::from(1));
    assert_eq!(sr[1][0], SheetCell::from(6));
    assert_eq!(sr[1][1], SheetCell::from(41));
    assert_eq!(sr[1][2], SheetCell::from(3));
    assert_eq!(sr[1][3], SheetCell::from(9));
    assert_eq!(sr[1][4], SheetCell::from(2));
    assert_eq!(sr[2][0], SheetCell::from(3));
    assert_eq!(sr[2][1], SheetCell::from(3));
    assert_eq!(sr[2][2], SheetCell::from(4));
    assert_eq!(sr[2][3], SheetCell::from(4));
  }

  #[test]
  fn test_spreadsheet_new_happy_uniform_row_length() {
    let sr = Spreadsheet::new(String::from("5 40 2 8 1\n6 41 3 9 2"));
    assert_eq!(sr[0][0], SheetCell::from(5));
    assert_eq!(sr[0][1], SheetCell::from(40));
    assert_eq!(sr[0][2], SheetCell::from(2));
    assert_eq!(sr[0][3], SheetCell::from(8));
    assert_eq!(sr[0][4], SheetCell::from(1));
    assert_eq!(sr[1][0], SheetCell::from(6));
    assert_eq!(sr[1][1], SheetCell::from(41));
    assert_eq!(sr[1][2], SheetCell::from(3));
    assert_eq!(sr[1][3], SheetCell::from(9));
    assert_eq!(sr[1][4], SheetCell::from(2));
  }

  #[test]
  fn test_spreadsheet_indexing_happy() {
    let sr = Spreadsheet::new(String::from("5\n1 2"));
    assert_eq!(sr[0][0], SheetCell::from(5));
    assert_eq!(sr[1][1], SheetCell::from(2));
  }

  #[test]
  #[should_panic]
  fn test_spreadsheet_indexing_out_of_bounds_panics() {
    let sr = Spreadsheet::new(String::from("5"));
    assert_eq!(sr[0][10], SheetCell::from(0));
  }

  #[test]
  #[should_panic]
  fn test_spreadsheet_indexing_out_of_bounds_panics_on_rows() {
    let sr = Spreadsheet::new(String::from("5"));
    assert_eq!(sr[10][10], sr[12][12]);
  }
}

#[cfg(test)]
mod sheet_row_tests {
  use super::*;

  #[test]
  fn test_sheetrow_part1() {
    let sr = SheetRow::new(String::from("1 2 3 4 5"));
    assert_eq!(sr.part1(), 4);
  }

  #[test]
  fn test_sheetrow_part2_ex1() {
    let sr = SheetRow::new(String::from("5 9 2 8"));
    assert_eq!(sr.part2(), 4);
  }

  #[test]
  fn test_sheetrow_part2_ex2() {
    let sr = SheetRow::new(String::from("9 4 7 3"));
    assert_eq!(sr.part2(), 3);
  }

  #[test]
  fn test_sheetrow_part2_ex3() {
    let sr = SheetRow::new(String::from("3 8 6 5"));
    assert_eq!(sr.part2(), 2);
  }

  #[test]
  fn test_sheetrow_rowmax() {
    let sr = SheetRow::new(String::from("1 2 3 4 5"));
    assert_eq!(sr.row_max(), 5);
  }

  #[test]
  fn test_sheetrow_rowmin() {
    let sr = SheetRow::new(String::from("1 2 3 4 5"));
    assert_eq!(sr.row_min(), 1);
  }

  #[test]
  fn test_sheetrow_len() {
    let sr = SheetRow::new(String::from("1 2 3 4 5"));
    assert_eq!(sr.len(), 5);
  }

  #[test]
  #[should_panic]
  fn test_sheetrow_new_panic_1() {
    let sr = SheetRow::new(String::from("dummy data goes here"));
    println!("We never get to use this {:?}", sr);
  }

  #[test]
  #[should_panic]
  fn test_sheetrow_new_panic_2() {
    // bad delimiter
    let sr = SheetRow::new(String::from("1 2 3 4_5"));
    println!("We never get to use this {:?}", sr);
  }

  #[test]
  fn test_sheetrow_new_happy() {
    let sr = SheetRow::new(String::from("5 40 2 8 1"));
    assert_eq!(sr[0], SheetCell::from(5));
    assert_eq!(sr[1], SheetCell::from(40));
    assert_eq!(sr[2], SheetCell::from(2));
    assert_eq!(sr[3], SheetCell::from(8));
    assert_eq!(sr[4], SheetCell::from(1));
  }

  #[test]
  fn test_sheetrow_indexing_happy() {
    let sr = SheetRow::new(String::from("5"));
    assert_eq!(sr[0], SheetCell::from(5));
  }

  #[test]
  #[should_panic]
  fn test_sheetrow_indexing_out_of_bounds_panics() {
    let sr = SheetRow::new(String::from("5"));
    assert_eq!(sr[10], SheetCell::from(0));
  }
}

#[cfg(test)]
mod sheet_cell_tests {
  use super::*;

  #[test]
  #[should_panic]
  fn test_sheetcell_new_panic() {
    let sc = SheetCell::new(String::from("dummy"));
    println!("We never get to use this {:?}", sc);
  }

  #[test]
  fn test_sheetcell_new_happy() {
    let sc = SheetCell::new(String::from("1"));
    assert_eq!(sc.data, 1);
  }

  #[test]
  fn test_sheetcell_from_happy() {
    let sc = SheetCell::from(1);
    assert_eq!(sc.data, 1);
  }
}

#[cfg(test)]
mod advent_example_tests {
  use super::*;

  #[test]
  fn test_part1_ex1() {
    let mut test_file = File::open("data/day2/tests/part1_ex1").expect("file not found");
    let mut test_content = String::new();
    test_file.read_to_string(&mut test_content).expect("Something went wrong reading part1 file");

    assert!(part1(test_content) == 18);
  }

  #[test]
  fn test_part2_ex1() {
    let mut test_file = File::open("data/day2/tests/part2_ex1").expect("file not found");
    let mut test_content = String::new();
    test_file.read_to_string(&mut test_content).expect("Something went wrong reading part2 file");

    assert!(part2(test_content) == 9);
  }
}
