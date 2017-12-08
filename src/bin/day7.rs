use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::collections::HashMap;

fn main() {
  println!("Advent of Code Day 7");
  println!("");
  println!("http://adventofcode.com/2017/day/7");
  println!("");

  let mut file = File::open("data/day7/input").expect("file not found");
  let mut content = String::new();
  file.read_to_string(&mut content).expect("Something went wrong reading input file");

  println!("Part 1: {}", part1(&content));
  part2(&content);
}

fn part1(input: &String) -> String {
  let mut pt = ProgramTree::empty();
  pt.parse(input);
  match pt.root() {
    Some(p) => return p.name,
    None => panic!("Couldn't find root, check input and retry.")
  }
}

fn part2(input: &String) {
  let mut pt = ProgramTree::empty();
  pt.parse(input);

  let root_name = pt.root().expect("").name;
  for target in pt.vertex_targets(&root_name) {
    let name = target.name.to_owned();
    let weight = pt.vertex_total_weight(&target.name);
    let initial = target.initial_weight.unwrap();

    println!("node: {}, weight: {}, initial: {}", name, weight, initial);
  }

  //step_part2(pt, "qawlwzi");
  step_part2(pt, "jfrda"); 
  //step_part2(pt, "lnpuarm"); // needs to be initial 910, not 918.
}

fn step_part2(mut pt: ProgramTree, input: &str) {
  println!("----------------");
  for target in pt.vertex_targets(&String::from(input)) {
    let name = target.name.to_owned();
    let weight = pt.vertex_total_weight(&target.name);
    let initial = target.initial_weight.unwrap();

    println!("node: {}, weight: {}, initial: {}", name, weight, initial);
  }
}


#[derive(Debug, PartialEq, Eq, Clone)]
struct Program {
  name: String,
  initial_weight: Option<i32>,
  total_weight: Option<i32>
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

      let initial_weight = parse_as::<i32>(&weight_string);

      return Program {
        name: String::from(name),
        initial_weight: Some(initial_weight),
        total_weight: None
      };
    } else {
      return Program {
        name: String::from(input.trim()),
        initial_weight: None,
        total_weight: None
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
    assert_eq!(p.initial_weight, Some(66));
  }

  #[test]
  fn test_new_without_weight() {
    let p = Program::new(&String::from("pbga "));
    assert_eq!(p.name, String::from("pbga"));
    assert_eq!(p.initial_weight, None);
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
    assert_eq!(p.initial_weight, Some(66));
  }

  #[test]
  fn trims_names_for_whitespace_no_weight() {
    let p = Program::new(&String::from("  pbga     "));
    assert_eq!(p.name, "pbga");
    assert_eq!(p.initial_weight, None);
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

  //pub fn part2(&mut self) {
    //let mut stack = vec![];
    //let mut result;
    //let mut cursor;
    //let mut prev_weight = -1;

    //stack.push(self.root().expect(""));

    //while !stack.empty() {
      //cursor = stack.pop();
      //let mut sorter = HashMap::new();
      //for target in self.vertex_targets(cursor.name) {
        //let w = self.vertex_total_weight(target.name) {
        //sorter.insert(w, 1 + sorter.get(w).unwrap_or(0)));
      //}

      //let mut offweight;

      //for key in sorter.keys() {
        //if let offweight = sorter.get(key).unwrap() == 1 {
          //for target in self.vertex_targets(cursor.name) {
            //if self.vertex_total_weight(target.name) == offweight {
              //stack.push(target);
              //result = target.name.to_owned();
              //break;
            //}
          //}
          //break;
        //}
      //}
    //}
  //}

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

  pub fn vertex_total_weight(&mut self, v_name: &String) -> i32 {
    let mut source = self.find_vertex_by_name(v_name).expect("Could not find vertex");

    match source.total_weight {
      Some(v) => return v,
      None => {
        let targets = self.vertex_targets(v_name);

        let mut sum = source.initial_weight.expect("Unset initial weight, cowardly exiting");

        for target in targets {
          sum += self.vertex_total_weight(&target.name);
        }

        source.total_weight = Some(sum);
        return sum;
      }
    }
  }

  pub fn vertex_targets(&self, v_name: &String) -> Vec<Program> {
    let mut ret = vec![];
    for e in &self.edges {
      match *e {
        (ref s, ref t) if s == v_name => {
          let v = self.find_vertex_by_name(&t).expect("");
          ret.push(v);
        },
        _ => ()
      }
    }
    return ret;
  }

  fn find_vertex_by_name(&self, name: &String) -> Option<Program> {
    for v in &self.vertices {
      if v.name == *name {
        let ret = Some(v.to_owned());
        return ret;
      }
    }

    return None;
  }
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

  #[test]
  fn test_vertex_targets() {
    let mut file = File::open("data/day7/test").expect("file not found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Something went wrong reading test file");

    let mut pt = ProgramTree::empty();
    pt.parse(&content);

    assert_eq!(pt.vertex_targets(&String::from("fwft")).len(), 3);
    assert_eq!(pt.vertex_targets(&String::from("qoyq")).len(), 0);
  }

  #[test]
  fn part2_test() {
    let mut file = File::open("data/day7/test").expect("file not found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Something went wrong reading test file");

    let mut pt = ProgramTree::empty();
    pt.parse(&content);

    assert_eq!(pt.vertex_total_weight(&String::from("ugml")), 251);
    assert_eq!(pt.vertex_total_weight(&String::from("padx")), 243);
    assert_eq!(pt.vertex_total_weight(&String::from("fwft")), 243);
  }
}
