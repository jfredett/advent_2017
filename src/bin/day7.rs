use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

fn main() {
  println!("Advent of Code Day 7");
  println!("");
  println!("http://adventofcode.com/2017/day/7");
  println!("");

  let mut file = File::open("data/day7/input").expect("file not found");
  let mut content = String::new();
  file.read_to_string(&mut content).expect("Something went wrong reading input file");

  println!("Part 1: {}", part1(&content));
  //println!("Part 2: {}", part2(&content));
}

fn part1(input: &String) -> String {
  let mut pt = ProgramTree::empty();
  pt.parse(input);
  match pt.root() {
    Some(p) => return p.name,
    None => panic!("Couldn't find root, check input and retry.")
  }
}

//fn part2(input: &String) -> i32 {
  //return MemoryBank::new(input, Mode::Part2).run();
//}


#[derive(Debug, PartialEq, Eq, Clone)]
struct Program {
  name: String,
  weight: Option<i32>
}

impl Program {
  // precondition: string looks like: /[a-z]+ \(\d+\))/
  pub fn new(input: &String) -> Program {
    if input.contains("->") {
      panic!("``{}'' is not a well-formed program definition", input);
    }
    if input.contains("(") {
      let mut v = input.split_whitespace();

      let name = v.next().expect("");
      let item = v.next().expect("").chars();
      let mut weight_string : String = String::new(); 

      for c in item {
        if c.is_digit(10) {
          weight_string.push(c);
        }
      }

      let weight = parse_as::<i32>(&weight_string);

      return Program {
        name: String::from(name),
        weight: Some(weight)
      };
    } else {
      return Program {
        name: String::from(input.trim()),
        weight: None 
      };
    }
  }
}

fn parse_as<T : FromStr>(input: &String) -> T {
  let parsed : Result<T, T::Err> = input.parse();
  match parsed {
    Ok(d) => return d,
    Err(_) => panic!("Failed to parse {}", input)
  }
}

#[cfg(test)]
mod program_tests {
  use super::*;
  #[test]
  fn test_new_with_weight() {
    let p = Program::new(&String::from("pbga (66)"));
    assert_eq!(p.name, String::from("pbga"));
    assert_eq!(p.weight, Some(66));
  }

  #[test]
  fn test_new_without_weight() {
    let p = Program::new(&String::from("pbga "));
    assert_eq!(p.name, String::from("pbga"));
    assert_eq!(p.weight, None);
  }

  #[test]
  #[should_panic]
  fn test_new_malformed() {
    let p = Program::new(&String::from("pbga -> "));
    assert_eq!(p,p);
  }

  #[test]
  fn trims_names_for_whitespace_weight() {
    let p = Program::new(&String::from("  pbga     (66)"));
    assert_eq!(p.name, "pbga");
    assert_eq!(p.weight, Some(66));
  }

  #[test]
  fn trims_names_for_whitespace_no_weight() {
    let p = Program::new(&String::from("  pbga     "));
    assert_eq!(p.name, "pbga");
    assert_eq!(p.weight, None);
  }
}


#[derive(Debug, PartialEq, Eq)]
struct ProgramTree {
  vertices: Vec<Program>,
  edges: Vec<(String,String)>
}

impl ProgramTree {
  pub fn empty() -> ProgramTree {
    return ProgramTree {
      vertices: vec![],
      edges: vec![]
    };
  }

  pub fn parse(&mut self, input : &String) {
    for line in input.lines() {
      self.insert_line(&String::from(line));
    }
  }

  pub fn root(&self) -> Option<Program> {
    for v in &self.vertices {
      if self.is_source(&v) && !self.is_target(&v) {
        return Some(v.to_owned());
      }
    }
    return None;
  }

  fn is_source(&self, v: &Program) -> bool {
    for e in &self.edges {
      match *e {
        (ref s, _) if *s == *v.name => return true,
        _ => ()
      }
    }

    return false;
  }

  fn is_target(&self, v: &Program) -> bool {
    for e in &self.edges {
      match *e {
        (_, ref t) if *t == *v.name => return true,
        _ => ()
      }
    }

    return false;
  }

  fn insert_line(&mut self, input: &String) {
    if input.contains("->") {
      let mut v = input.split("->");

      let p = Program::new(&String::from(v.next().expect("")));

      self.add_vertex(&p);

      let edge_targets = v.next().expect("").split(",");
      for edge_target in edge_targets {
        let et = String::from(edge_target.trim());
        self.add_edge(&p.name, &et);
      }
    } else {
      let p = Program::new(input);
      self.add_vertex(&p);
    }
  }

  fn add_vertex(&mut self, p : &Program) {
    let np = p.to_owned();
    self.vertices.push(np);
  }

  fn add_edge(&mut self, source: &String, target: &String) {
    let a = source.to_owned();
    let b = target.to_owned();
    self.edges.push((a,b));
  }

  //fn find_vertex_by_name(&self, name: &String) -> &Option<Program> {
    //for v in &self.vertices {
      //if name == *v.name {
        //return &Some(*v);
      //}
    //}

    //return None;
  //}
}

#[cfg(test)]
mod program_tree_tests {
  use super::*;

  #[test]
  fn part1_test() {
    let mut file = File::open("data/day7/test").expect("file not found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Something went wrong reading test file");

    let mut pt = ProgramTree::empty();
    pt.parse(&content);

    assert_eq!(pt.root().expect("").name, "tknk");
  }
}
