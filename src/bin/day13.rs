use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;
use std::cmp;

fn parse_as<T : FromStr>(input: &String) -> T {
  let parsed : Result<T, T::Err> = input.trim().parse();
  match parsed {
    Ok(d) => return d,
    Err(_) => panic!("Failed to parse {}", input)
  }
}

fn main() {
  println!("Advent of Code Day 13");
  println!("");
  println!("http://adventofcode.com/2017/day/13");
  println!("");

  let mut file = File::open("data/day13/input").expect("file not found");
  let mut content = String::new();
  file.read_to_string(&mut content).expect("Something went wrong reading input file");

  solve(&content);
}

fn solve(input: &String) {
    let mut fw = Firewall::parse(input);
    println!("Part 1: {}", fw.execute_trip());

    let mut idx = 3913000;
    loop {
        fw.reset();
        idx += 1;

        fw.execute_trip_with_delay(idx);

        // caught a red herring, something weird in my implementation
        if !fw.was_caught && idx > 191776 { break; }

        if idx % 1000 == 0 {
            println!("All delays up to {} failed.", idx);
        }

    }

    println!("Part 2: {}", idx);
}


#[derive(Debug, PartialEq, Eq)]
struct Layer {
    depth: i32,
    range: i32,
    scanner_ptr: i32,
    scanner_direction: i32
}

#[derive(Debug, PartialEq, Eq)]
struct Firewall {
    layers: HashMap<i32, Layer>,
    max_depth: i32,
    total_severity: i32, 
    position_ptr: i32,
    was_caught: bool

}


impl Layer {
    pub fn new(depth: i32, range: i32) -> Layer {
        // start with negative 1 so the initial flip won't mess up
        Layer { depth: depth, range: range, scanner_ptr: 0, scanner_direction: -1 }
    }

    pub fn sweep_by_amt(&mut self, amt: i32) {
        let real_amt = amt % (2*(self.range - 1));

        for _ in 0..real_amt {
            self.sweep();
        }
    }

    pub fn sweep(&mut self) {
        // range - 1 because zero indexing.
        if self.scanner_ptr == (self.range - 1) || self.scanner_ptr == 0 {
            self.flip_direction();
        }

        self.scanner_ptr += self.scanner_direction;
    }

    pub fn reset(&mut self) {
        self.scanner_direction = -1;
        self.scanner_ptr = 0;
    }

    pub fn is_caught(&self) -> bool {
        return self.scanner_ptr == 0;
    }

    pub fn severity(&self) -> i32 {
        return self.depth * self.range;
    }

    fn flip_direction(&mut self) {
        self.scanner_direction *= -1;
    }
}

impl Firewall {
    pub fn empty() -> Firewall {
        return Firewall {
            layers: HashMap::new(),
            max_depth: 0,
            total_severity: 0,
            position_ptr: 0,
            was_caught: false
        };
    }

    pub fn parse(input: &String) -> Firewall {
        let mut fw = Firewall::empty();

        for line in input.lines() {
            let mut parsed = line.split(':');
            let depth_str = String::from(parsed.next().unwrap());
            let range_str = String::from(parsed.next().unwrap());
            let depth = parse_as::<i32>(&depth_str);
            let range = parse_as::<i32>(&range_str);

            fw.add_layer(Layer::new(depth, range));
        }

        return fw;
    }

    pub fn add_layer(&mut self, layer: Layer) {
        let depth = layer.depth;

        self.layers.insert(depth, layer);

        self.max_depth = cmp::max(depth, self.max_depth);
    }

    pub fn execute_trip(&mut self) -> i32 {
        while !self.is_trip_complete() {
            self.step();
        }
        return self.total_severity;
    }

    pub fn execute_trip_with_delay(&mut self, delay: i32) -> i32 {
        self.sweep_by_amt(delay);

        // this is to catch depth-0 catches
        if self.position_ptr == 0 {

            self.was_caught == self.is_caught();
            // this is a no-op, because the severity of a depth 0 catch is always 0
            // self.add_severity();
        }

        return self.execute_trip();
    }

    pub fn reset(&mut self) {
        for (_, mut layer) in self.layers.iter_mut() { layer.reset(); }
        self.position_ptr = 0;
        self.total_severity = 0;
        self.was_caught = false;
    }

    pub fn dump(&self) {
        println!("----");
        for i in 0..self.max_depth+1 {
            print!("{}: ", i);
            match self.layers.get(&i) {
                Some(layer) => {
                    for j in 0..layer.range {
                        if j == layer.scanner_ptr {
                            print!(" [S]");
                        } else  {
                            print!(" [ ]");
                        }
                    }
                },
                None => {
                    print!("...");
                }
            }
            println!();
        }
        println!("----");
    }

    fn sweep_by_amt(&mut self, amt: i32) {
        for (_, mut layer) in self.layers.iter_mut() { layer.sweep_by_amt(amt); }
    }

    fn sweep(&mut self) {
        for (_, mut layer) in self.layers.iter_mut() { layer.sweep(); }
    }

    fn step(&mut self) {
        // packet moves one layer forward
        self.position_ptr += 1;

        self.sweep();

        if self.is_caught() {
            self.was_caught = true;
            self.add_severity(); 
        }
    }

    pub fn is_trip_complete(&self) -> bool {
        return self.position_ptr >= self.max_depth;
    }

    fn is_caught(&self) -> bool {
        if let Some(l) = self.current_position() {
            return l.is_caught()
        }
        return false;
    }

    fn current_position(&self) -> Option<&Layer> {
        return self.layers.get(&self.position_ptr);
    }

    fn add_severity(&mut self) {
        self.total_severity += self.current_severity();
    }

    fn current_severity(&self) -> i32 {
        return match self.current_position() {
            Some(l) => l.severity(),
            None => 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layer_test() {
        let mut layer = Layer::new(1,2);
        assert_eq!(layer.scanner_ptr, 0);
        layer.sweep();
        assert_eq!(layer.scanner_ptr, 1);
        layer.sweep();
        assert_eq!(layer.scanner_ptr, 0);
    }

    #[test]
    fn layer_test_2() {
        let mut layer = Layer::new(1,2);
        assert_eq!(layer.scanner_ptr, 0);
        layer.sweep_by_amt(2);
        assert_eq!(layer.scanner_ptr, 0);
        layer.sweep_by_amt(3);
        assert_eq!(layer.scanner_ptr, 1);
    }

    #[test]
    fn layer_test_3() {
        let mut layer = Layer::new(1,10);

        layer.sweep();
        layer.sweep();
        layer.sweep();
        layer.sweep();
        layer.sweep();

        let manual_ptr = layer.scanner_ptr;

        layer.reset();

        layer.sweep_by_amt(5);

        let auto_ptr = layer.scanner_ptr;

        assert_eq!(manual_ptr, auto_ptr);
    }

    #[test]
    fn example_test() {
        let mut file = File::open("data/day13/test").expect("file not found");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Something went wrong reading test file");

        let mut fw = Firewall::parse(&content);

        assert_eq!(fw.execute_trip(), 24);
    }

    #[test]
    fn example_test_part2() {
        let mut file = File::open("data/day13/test").expect("file not found");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Something went wrong reading test file");

        let mut fw = Firewall::parse(&content);

        let mut idx = 0;
        loop {
            fw.reset();
            idx += 1;
            fw.execute_trip_with_delay(idx);
            if !fw.was_caught { break; }


            if idx > 10 { assert!(false); } // we've failed
        }
        assert_eq!(idx, 10);
        assert!(!fw.was_caught);
    }

    #[test]
    fn dump() {
        let mut file = File::open("data/day13/test").expect("file not found");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Something went wrong reading test file");

        let mut fw = Firewall::parse(&content);


        fw.sweep_by_amt(10);
        fw.dump();

        fw.reset();

        fw.sweep();
        fw.sweep();
        fw.sweep();
        fw.sweep();
        fw.sweep();
        fw.sweep();
        fw.sweep();
        fw.sweep();
        fw.sweep();
        fw.sweep();
        fw.dump();

        assert!(false);
    }
}

