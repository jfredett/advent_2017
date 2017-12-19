#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

use regex::Regex;


fn main() {
    println!("Advent of Code Day 16");
    println!("");
    println!("http://adventofcode.com/2017/day/16");
    println!("");

	let mut file = File::open("data/day16/input").expect("file not found");
	let mut content = String::new();
	file.read_to_string(&mut content).expect("Something went wrong reading input file");

	part1(&content);
    part2(&content);
}


lazy_static! {
    static ref SPIN: Regex = Regex::new(r"s(\d+)").unwrap();
    static ref EXCHANGE_BY_IDX: Regex = Regex::new(r"x(\d+)/(\d+)").unwrap();
    static ref EXCHANGE_BY_NAME: Regex = Regex::new(r"p([a-p])/([a-p])").unwrap();
}

fn part1(input: &String) {
    let mut d = Dance::parse(&input);
    println!("Part 1 - no opt: {}", d.run());
    d.optimize();
    println!("Part 1 - opt: {}", d.run());
}

fn part2(input: &String) {

    let default_ring = Ring::new();
    let mut r = Ring::new();
    let mut d = Dance::parse(&input);


    let mut cycle : Option<i32> = None;
    let mut i = 0;
    while i != 1000000000 {
        //println!("{}/{} complete", i, 1000000000);

        match cycle {
            None => {
                d.run_with_ring(&mut r);
                i += 1;

                if default_ring == r { cycle = Some(i); }
            },
            Some(order) => {
                if i + order < 1000000000 {
                    i += order; // once we have the order, we can skip ahead
                } else {
                    // back to slow mode
                    d.run_with_ring(&mut r);
                    i += 1;
                    //seen.push(r);
                }
            }
        }
    }

    println!("Part 2 - order opt: {}", r);
}


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Command {
    SPIN(usize),
    EXCHANGE(usize, usize),
    PARTNER(Program, Program)
}

struct Dance {
    instructions: Vec<Command>
}

impl Dance {
    pub fn parse(input: &String) -> Dance {
        let mut commands = vec![];
        for command in input.trim().split(",") {
            commands.push(Command::parse(&String::from(command)));
        }

        return Dance { instructions: commands };
    }

    pub fn run(&self) -> Ring {
        let mut r = Ring::new();
        self.run_with_ring(&mut r);
        return r
    }

    pub fn run_with_ring(&self, r: &mut Ring) {
        for c in &self.instructions {
            r.execute(&c);
        }
    }

    // GROUP THEORY TIME
    //
    // if you have a permutation that maps:
    //
    // ABCDE
    //
    // to 
    //
    // AEDBC
    //
    // -- no matter how complicated the initial map was, every subsequent application will simply
    // be another map which fixes A, maps B -> E, C -> D, D -> B, E -> C
    //
    // This is, in usual permutation notation is the permutation: (BECD). This is applied to an
    // element by looking at the element, `x`, finding it's position in the list, and then taking
    // that position + 1.
    //
    // We can combine to permutations through function composition.
    //
    // The above ((BECD)) is equivalent to a series of disjoint transpositions, namely:
    //
    // (BD)(BC)(BE)
    //
    // verify by function decomp if you like.
    //
    // We can model this as a sequence of by-name exchanges, to calculate it, we simply compare the
    // two lists, finding each closed loop, once we've done that, we decompose them into exchange
    // commands, and then we have an optimized program
    //
    // But we can go further.
    //
    // The Order of a permutation is the number of times it must be applied to result in an
    // identity transformation. For a simple cycle, as above, that's simple to calculate, it's just
    // the length of the permutation.
    //
    // For a more complicated product of several cycles, it's the LCM of the lengths of the cycles
    // (this is pretty easy to prove). Since we'll have calculated the cycles, all we have to do is
    // calculate the LCM of the cycles to find it's order. Then, instead of executing 1 billion
    // times, we simply execute 10^9 % Order(permutation). That makes this thing more or less
    // doable by hand. I'm not going to bother with that.
    //
    // NB. After a bit of work, it turns out that Unfortunately since we have to restart each time,
    // it doesn't work to simply calculate the order, since the program re-runs with the partner
    // operations, the permutation changes each time. It would be possible to double up the
    // instructions a few doesn't times to calculate the order, or you can do it dynamically as
    // part2 does above.
    pub fn optimize(&mut self) {
        let r = self.run();
        let s = Ring::new();

        let mut loops = vec![];
        let mut seen = vec![];

        let mut i = 0;

        while i < 16 {
            let start = s.get(i);
            if seen.contains(&start) {
                i += 1;
            } else {
                seen.push(start);
                let mut new_loop = vec![start];
                let mut next = s.get(r.find(start));

                while next != start {
                    new_loop.push(next);
                    seen.push(next);
                    next = s.get(r.find(next));
                }

                loops.push(new_loop);
            }

        }

        let mut instructions = vec![];

        for l in loops {
            // if the loop is only 1 element, it's an identity and we can ignore it
            if l.len() > 1 {
                let first = l[0];
                let length = l.len();
                for idx in 1..length {
                    let e = l[length - idx];
                    instructions.push(Command::PARTNER(first, e));
                }
            }
        }

        self.instructions = instructions;
    }
}

impl Command {
    fn parse(command: &String) -> Command {
        if SPIN.is_match(command) {
            let parsed = SPIN.captures(command).unwrap();
            let amt : usize = parse_as::<usize>(&String::from(&parsed[1]));
            return Command::SPIN(amt);
        } else if EXCHANGE_BY_IDX.is_match(command) {
            let parsed = EXCHANGE_BY_IDX.captures(command).unwrap();
            let i : usize = parse_as::<usize>(&String::from(&parsed[1]));
            let j : usize = parse_as::<usize>(&String::from(&parsed[2]));
            return Command::EXCHANGE(i,j);
        } else if EXCHANGE_BY_NAME.is_match(command) {
            let parsed = EXCHANGE_BY_NAME.captures(command).unwrap();
            let i : Program = Program::from_string(&String::from(&parsed[1]));
            let j : Program = Program::from_string(&String::from(&parsed[2]));
            return Command::PARTNER(i,j);
        } else {
            panic!("Unrecognized command: {}", command);
        }

    }
}


fn parse_as<T : FromStr>(input: &String) -> T {
  let parsed : Result<T, T::Err> = input.trim().parse();
  match parsed {
    Ok(d) => return d,
    Err(_) => panic!("Failed to parse {}", input)
  }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Ring {
    entries: [Program; 16]
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Program {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P
}

impl Program {
	fn from_string(s : &String) -> Program {
        if *s == String::from("a") { return Program::A; }
        else if *s == String::from("b") { return Program::B; }
        else if *s == String::from("c") { return Program::C; }
        else if *s == String::from("d") { return Program::D; }
        else if *s == String::from("e") { return Program::E; }
        else if *s == String::from("f") { return Program::F; }
        else if *s == String::from("g") { return Program::G; }
        else if *s == String::from("h") { return Program::H; }
        else if *s == String::from("i") { return Program::I; }
        else if *s == String::from("j") { return Program::J; }
        else if *s == String::from("k") { return Program::K; }
        else if *s == String::from("l") { return Program::L; }
        else if *s == String::from("m") { return Program::M; }
        else if *s == String::from("n") { return Program::N; }
        else if *s == String::from("o") { return Program::O; }
        else if *s == String::from("p") { return Program::P; }
        else { panic!("Unrecognized program: {}", s) }
	}
}

impl Ring {
    pub fn new() -> Ring {
        Ring { entries: [ 
            Program::A, Program::B, Program::C, Program::D,
            Program::E, Program::F, Program::G, Program::H,
            Program::I, Program::J, Program::K, Program::L,
            Program::M, Program::N, Program::O, Program::P
        ]}
    }

    pub fn spin(&mut self, shift: usize) {
        let mut new_arr : [Program; 16] = [ 
            Program::A, Program::B, Program::C, Program::D,
            Program::E, Program::F, Program::G, Program::H,
            Program::I, Program::J, Program::K, Program::L,
            Program::M, Program::N, Program::O, Program::P
        ];

        for i in 0..16 {
            new_arr[(i + shift) % 16] = self.entries[i];
        }

        self.entries = new_arr;
    }

    pub fn execute(&mut self, c: &Command) {
        match *c {
            Command::SPIN(amt) => self.spin(amt),
            Command::EXCHANGE(i,j) => self.exchange(i,j),
            Command::PARTNER(a,b) => self.exchange_by_name(a,b)
        }
    }

    pub fn exchange(&mut self, i: usize, j: usize) {
        self.entries.swap(i,j);
    }

    pub fn exchange_by_name(&mut self, i: Program, j: Program) {
        let idx_i = self.find(i);
        let idx_j = self.find(j);
        self.exchange(idx_i, idx_j);
    }

    pub fn get(&self, idx: usize) -> Program {
        self.entries[idx]
    }

    pub fn find(&self, name: Program) -> usize {
        for i in 0..16 {
            if self.entries[i] == name { return i; }
        }
        panic!("Unreachable");
    }
}

impl fmt::Display for Program {

    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Program::A => write!(f, "A"),
            Program::B => write!(f, "B"),
            Program::C => write!(f, "C"),
            Program::D => write!(f, "D"),
            Program::E => write!(f, "E"),
            Program::F => write!(f, "F"),
            Program::G => write!(f, "G"),
            Program::H => write!(f, "H"),
            Program::I => write!(f, "I"),
            Program::J => write!(f, "J"),
            Program::K => write!(f, "K"),
            Program::L => write!(f, "L"),
            Program::M => write!(f, "M"),
            Program::N => write!(f, "N"),
            Program::O => write!(f, "O"),
            Program::P => write!(f, "P")
        }
    }
}

impl fmt::Display for Ring {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        for i in 0..16 {
            write!(f, "{}", self.entries[i]);
        }
        write!(f,"")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spin() {
        let mut r = Ring::new();
        r.spin(2);


        assert_eq!(r.get(0), Program::O);
        assert_eq!(r.get(1), Program::P);
        assert_eq!(r.get(2), Program::A);
    }

    // this should be quick-checkable, but two tests is easier for now
    #[test]
    fn exchange_high_low() {
        let mut r = Ring::new();

        assert_eq!(r.get(3), Program::D);
        assert_eq!(r.get(5), Program::F);

        r.exchange(5,3);

        assert_eq!(r.get(5), Program::D);
        assert_eq!(r.get(3), Program::F);
    }

    #[test]
    fn exchange_low_high() {
        let mut r = Ring::new();

        assert_eq!(r.get(3), Program::D);
        assert_eq!(r.get(5), Program::F);

        r.exchange(3,5);

        assert_eq!(r.get(5), Program::D);
        assert_eq!(r.get(3), Program::F);
    }


    //
    #[test]
    fn exchange_by_name() {
        let mut r = Ring::new();

        assert_eq!(r.get(3), Program::D);
        assert_eq!(r.get(5), Program::F);

        r.exchange_by_name(Program::D, Program::F);

        assert_eq!(r.get(5), Program::D);
        assert_eq!(r.get(3), Program::F);
    }

    mod program {
        use super::*;

            #[test]
            fn from_string() {
                let p = Program::from_string(&String::from("g"));
                assert_eq!(p, Program::G);
            }
    }
}
