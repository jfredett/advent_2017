fn main() {
    println!("Advent of Code Day 16");
    println!("");
    println!("http://adventofcode.com/2017/day/16");
    println!("");
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

    pub fn get(&self, idx: usize) -> Program {
        self.entries[idx]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spin() {
        let mut r = Ring::new();
        r.spin(2);

        assert_eq!(r.get(0), Program::C);
        assert_eq!(r.get(2), Program::A);
    }

}