use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;
use std::str::FromStr;

fn parse_as<T : FromStr>(input: &String) -> T {
  let parsed : Result<T, T::Err> = input.trim().parse();
  match parsed {
    Ok(d) => return d,
    Err(_) => panic!("Failed to parse {}", input)
  }
}

fn main() {
  println!("Advent of Code Day 12");
  println!("");
  println!("http://adventofcode.com/2017/day/12");
  println!("");

  let mut file = File::open("data/day12/input").expect("file not found");
  let mut content = String::new();
  file.read_to_string(&mut content).expect("Something went wrong reading input file");

  solve(&content);
}

fn solve(input: &String) {
  let g = Graph::parse(input);
  println!("Part 1: {}", g.spanning_set_for(0).len());

  println!("Part 2: {}", g.number_of_groups());
}

// undirected graph
struct Graph<T> {
  vertices: HashSet<T>,
  edges: HashSet<(T,T)>
}

impl<T : PartialEq + Eq + Hash + FromStr + Copy> Graph<T> {
  pub fn empty() -> Graph<T> {
    Graph {
      vertices: HashSet::new(),
      edges: HashSet::new()
    }
  }

  pub fn parse(input: &String) -> Graph<T> {
    let mut g = Graph::empty();

    for line in input.lines() {
      let mut split_str = line.trim().split("<->");
      let vertex_str = &String::from(split_str.next().unwrap());
      let vertex = parse_as::<T>(vertex_str);
      let edge_list = split_str.next().unwrap();

      g.add_vertex(vertex);

      for edge in edge_list.split(",") {
        let target = parse_as::<T>(&String::from(edge));
        // we force it so we can build the graph in one pass,
        // if the input is bad, we'll get dangling elements
        // on the graph.
        g.force_add_edge(vertex, target);
      }
    }

    return g;
  }
  // the set of all vertices in G that are in the spanning
  // tree rooted at v
  pub fn spanning_set_for(&self, v: T) -> HashSet<T> {
    let mut spanning_set = HashSet::new();
    let mut search_set = self.all_connected(v);

    while !search_set.is_empty() {
      // unwrap safe because of the above check
      let c = search_set.pop().unwrap();

      spanning_set.insert(c);

      for connection in self.all_connected(c) {
        if !spanning_set.contains(&connection) {
          search_set.push(connection);
        }
      }
    }

    return spanning_set;
  }

  // the set of all the vertices connected by an edge to the
  // given one.
  pub fn all_connected(&self, v: T) -> Vec<T> {
    let mut ret = HashSet::new();
    for &edge in &self.edges {
      match edge {
        (s,t) if s == v => { ret.insert(t); }
        (s,t) if t == v => { ret.insert(s); }
        _ => continue
      }
    }
    return ret.into_iter().collect();
  }

  pub fn add_vertex(&mut self, v: T) {
    self.vertices.insert(v);
  }

  pub fn add_edge(&mut self, v: T, u: T) {
    if self.has_vertex(v) && self.has_vertex(u) {
      self.force_add_edge(v,u);
    } else {
      panic!("Graph does not contain on of the vertices you tried to draw an edge between!");
    }
  }

  fn force_add_edge(&mut self, v: T, u: T) {
    self.edges.insert((v,u));
  }

  pub fn has_vertex(&self, v: T) -> bool {
    return self.vertices.contains(&v);
  }

  pub fn number_of_groups(&self) -> i32 {
    let mut seen = HashSet::new();
    let mut count = 0;

    for v in &self.vertices {
      if !seen.contains(v) {
        for s in self.spanning_set_for(*v) {
          seen.insert(s);
        }
        count += 1;
      }

      if seen.len() == self.vertices.len() { break; }
    }

    return count;
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_spanning_set() {
    let mut g = Graph::empty();

    g.add_vertex(0);
    g.add_vertex(1);
    g.add_vertex(2);
    g.add_edge(1,2);

    assert!(g.spanning_set_for(1).contains(&1));
    assert!(g.spanning_set_for(1).contains(&2));
    assert!(!g.spanning_set_for(1).contains(&0));
  }

  #[test]
  fn test_number_of_groups() {
    let mut g = Graph::empty();

    g.add_vertex(0);
    g.add_vertex(1);
    g.add_vertex(2);
    g.add_edge(1,2);

    assert_eq!(g.number_of_groups(), 2);
  }

  #[test]
  fn test_ex1_spanning() {
    let mut file = File::open("data/day12/test").expect("file not found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Something went wrong reading test file");

    let g : Graph<i32> = Graph::parse(&content);
    let spanning = g.spanning_set_for(0);

    assert!(spanning.contains(&0));
    assert!(spanning.contains(&2));
    assert!(spanning.contains(&3));
    assert!(spanning.contains(&4));
    assert!(spanning.contains(&5));
    assert!(spanning.contains(&6));

    assert!(!spanning.contains(&1));
  }

  #[test]
  fn test_ex1_groups() {
    let mut file = File::open("data/day12/test").expect("file not found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Something went wrong reading test file");

    let g : Graph<i32> = Graph::parse(&content);

    assert_eq!(g.number_of_groups(), 2);
  }

}
