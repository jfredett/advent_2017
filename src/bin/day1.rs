use std::str::Chars;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  println!("Advent of Code Day 1");
  println!("");
  println!("Captcha: http://adventofcode.com/2017/day/1");
  println!("");

  let mut part1_captcha_file = File::open("data/day1/part1_input").expect("file not found");
  let mut part1_captcha = String::new();
  part1_captcha_file.read_to_string(&mut part1_captcha).expect("Something went wrong reading part1 file");

  // it doesn't actually change, but this avoids the use-after-move issue
  let mut part2_captcha_file = File::open("data/day1/part1_input").expect("file not found");
  let mut part2_captcha = String::new();
  part2_captcha_file.read_to_string(&mut part2_captcha).expect("Something went wrong reading part2 file");

  println!("Part 1: {}", day1_part1(part1_captcha));
  println!("Part 2: {}", day1_part2(part2_captcha));
}

fn day1_part1(input: String) -> u32 {
  let captcha: Chars = input.chars();
  let mut digits: Vec<u32> = vec![];

  // convert everything to digits
  for c in captcha {
    match c.to_digit(10) {
      Some(d) => digits.push(d),
      _ => ()
    }
  }

  let mut sum = 0;

  if digits.last() == digits.first() {
    sum += digits[0];
  }
  for idx in 0..(digits.len() - 1) {
    if digits[idx] == digits[idx+1] {
      sum += digits[idx];
    }
  }

  return sum;
}

fn day1_part2(input: String) -> u32 {
  let captcha: Chars = input.chars();
  let mut digits: Vec<u32> = vec![];

  // convert everything to digits
  for c in captcha {
    match c.to_digit(10) {
      Some(d) => digits.push(d),
        None => ()
    }
  }

  let mut sum = 0;
  let jump = digits.len() / 2;

  for idx in 0..(digits.len() ) {
    if digits[idx] == digits[(idx + jump) % digits.len()] {
      sum += digits[idx];
    }
  }
  return sum;
}

// Part 1

#[test]
fn test_part1_ex1() {
  assert!(day1_part1(String::from("1122")) == 3);
}

#[test]
fn test_part1_ex2() {
  assert!(day1_part1(String::from("1111")) == 4);
}

#[test]
fn test_part1_ex3() {
  assert!(day1_part1(String::from("1234")) == 0);
}

#[test]
fn test_part1_ex4() {
  assert!(day1_part1(String::from("91212129")) == 9);
}

// Part 2

#[test]
fn test_part2_ex1() {
  assert!(day1_part2(String::from("1212")) == 6);
}

#[test]
fn test_part2_ex2() {
  assert!(day1_part2(String::from("1221")) == 0);
}

#[test]
fn test_part2_ex3() {
  assert!(day1_part2(String::from("123425")) == 4);
}

#[test]
fn test_part2_ex4() {
  assert!(day1_part2(String::from("123123")) == 12);
}

#[test]
fn test_part2_ex5() {
  assert!(day1_part2(String::from("12131415")) == 4);
}

