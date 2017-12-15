
fn main() {
    println!("Advent of Code Day 15");
    println!("");
    println!("http://adventofcode.com/2017/day/15");
    println!("");

    let mut judge = Judge::new(vec![
                               Generator::new(722, 16807), //generator A
                               Generator::new(354, 48271), //generator B
    ]);


    for _ in 0..40000000 {
        judge.judge_round();
    }

    println!("Part 1: {}", judge.score);
}

const MODULUS : u64 = 2147483647;
const MASK : u64 = 0b1111111111111111;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Generator {
    current_value: u64,
    generation_factor: u64,
    //history: Vec<u64>
}

impl Generator {
    pub fn new(initial: u64, factor: u64) -> Generator {
        return Generator {
            current_value: initial,
            generation_factor: factor
            //history: vec![] 
        };
    }

    pub fn generate(&mut self)  {
        //self.history.push(self.current_value);
        self.current_value = (self.current_value * self.generation_factor) % MODULUS;
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Judge {
    gen_a: Generator,
    gen_b: Generator,
    score: u64
}

impl Judge {

    pub fn new(generators: Vec<Generator>) -> Judge {
        return Judge {
            gen_a: generators[0].to_owned(),
            gen_b: generators[1].to_owned(),
            score: 0
        };
    }

    pub fn judge_round(&mut self) {
        self.gen_a.generate();
        self.gen_b.generate();

        let lower_a = self.gen_a.current_value & MASK;
        let lower_b = self.gen_b.current_value & MASK;

        if lower_a == lower_b {
            self.score += 1
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let mut judge = Judge::new(vec![
           Generator::new(65, 16807), //generator A
           Generator::new(8921, 48271), //generator B
        ]);


        for _ in 0..5 {
            judge.judge_round();
        }

        assert_eq!(judge.score, 1);
    }
}
