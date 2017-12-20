use std::fmt;
use std::io::prelude::*;
use std::collections::LinkedList;

fn main() {
    println!("Advent of Code Day 17");
    println!("");
    println!("http://adventofcode.com/2017/day/17");
    println!("");

    part1(304);
    part2(304);
}

fn part1(amt: usize) {
    let mut c = spinlocker_walker(amt, 2018);
    c.next();
    println!("Part 1: {}", c.pointer);
}

fn part2(amt: usize) {
    let mut c = spinlocker_walker(amt, 5000001);
    while c.pointer != 0 { c.next(); }
    c.next();
    println!("Part 1: {}", c.pointer);
}


#[derive(PartialEq, Eq, Debug, Clone)]
struct CircularBuffer<T> {
    left: LinkedList<T>,
    pointer: T,
    right: LinkedList<T>
}

impl<T : Clone + Copy> CircularBuffer<T> {

    pub fn new(v: T) -> CircularBuffer<T> {
        CircularBuffer { left: LinkedList::new(), pointer: v, right: LinkedList::new() }
    }

    pub fn next(&mut self) {
        if self.right.is_empty() {
            self.reframe_right();
            self.right.push_back(self.pointer);
        } else {
            self.left.push_back(self.pointer);
        }
        self.pointer = self.right.pop_front().unwrap();
    }

    fn reframe_right(&mut self) {
        for e in self.left.to_owned() {
            self.right.push_back(e);
        }
        self.left.clear();
    }

    pub fn prev(&mut self) {
        if self.left.is_empty() {
            // [] (1) [2,3,4]
            self.reframe_left();
            // [2,3,4] (1) []
            self.left.push_front(self.pointer);
            // [1,2,3,4] (1) []
        } else {
            self.right.push_front(self.pointer);
        }
        self.pointer = self.left.pop_back().unwrap();
    }

    fn reframe_left(&mut self) {
        for e in self.right.to_owned() {
            self.left.push_back(e);
        }
        self.right.clear();
    }

    pub fn insert(&mut self, v: T) {
        self.left.push_back(self.pointer);
        self.pointer = v;
    }

    //pub fn len(&self) -> usize {
        //self.left.len() + self.right.len() + 1
    //}
}

impl<T : fmt::Display + Clone + Copy> fmt::Display for CircularBuffer<T> {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let mut my_self = self.to_owned();

        let _ = write!(f, "[");
        while !my_self.left.is_empty() {
            let _ = write!(f, "{}, ",  my_self.left.pop_front().unwrap());
        }

        if my_self.right.is_empty() {
            write!(f, "({})]", my_self.pointer)
        } else {
            let _ = write!(f, "({}), ", my_self.pointer);
            let last_elt = my_self.right.pop_back().unwrap();
            while !my_self.right.is_empty() {
                let _ = write!(f, "{}, ",  my_self.right.pop_front().unwrap());
            }
            write!(f, "{}]", last_elt)
        }
    }
}

fn spinlocker_walker(step_amt: usize, step_max: usize) -> CircularBuffer<u32> {
    let mut c = CircularBuffer::new(0);
    for i in 1..step_max {
        for _ in 0..step_amt {
           c.next();
        }
        c.insert(i as u32);
    }
    return c;
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_buffer_2() -> CircularBuffer<i32> {
        let mut c = CircularBuffer::new(4);

        c.left.push_front(3);
        c.left.push_front(2);
        c.left.push_front(1);

        return c;
    }

    fn example_buffer() -> CircularBuffer<i32> {
        let mut c = CircularBuffer::new(0);
        c.left.push_front(-1);
        c.left.push_front(-2);
        c.left.push_front(-3);
        c.left.push_front(-4);
        c.left.push_front(-5);

        c.right.push_front(5);
        c.right.push_front(4);
        c.right.push_front(3);
        c.right.push_front(2);
        c.right.push_front(1);

        return c;
    }

    mod prev {
        use super::*;

        #[test]
        fn at_end_2() {
            let mut c = example_buffer_2();

            assert_eq!(c.pointer, 4);
            c.prev();
            assert_eq!(c.pointer, 3);
            c.prev();
            assert_eq!(c.pointer, 2);
            c.prev();
            assert_eq!(c.pointer, 1);
            c.prev();
            assert_eq!(c.pointer, 4);
        }
        #[test]
        fn at_end() {
            let mut c = example_buffer();
            c.prev(); c.prev(); c.prev(); c.prev(); c.prev();
            assert_eq!(c.pointer, -5);
            c.prev();
            assert_eq!(c.pointer, 5);
        }

        #[test]
        fn not_at_end() {
            let mut c = example_buffer();
            assert_eq!(c.pointer, 0);
            c.prev(); c.prev();
            assert_eq!(c.pointer, -2);
            c.prev(); c.prev();
            assert_eq!(c.pointer, -4);
        }
    }

    mod next {
        use super::*;

        #[test]
        fn at_end_2() {
            let mut c = example_buffer_2();
            assert_eq!(c.pointer, 4);
            c.next();
            assert_eq!(c.pointer, 1);
            c.next();
            assert_eq!(c.pointer, 2);
            c.next();
            assert_eq!(c.pointer, 3);
            c.next();
            assert_eq!(c.pointer, 4);
        }
        #[test]
        fn at_end() {
            let mut c = example_buffer();
            c.next();
            c.next();
            c.next();
            c.next();
            c.next();
            assert_eq!(c.pointer, 5);
            c.next();
            assert_eq!(c.pointer, -5);
        }

        #[test]
        fn not_at_end() {
            let mut c = example_buffer();
            assert_eq!(c.pointer, 0);
            c.next(); c.next();
            assert_eq!(c.pointer, 2);
            c.next(); c.next();
            assert_eq!(c.pointer, 4);
        }
    }

    mod insert {
        use super::*;

        #[test]
        fn insert() {
            let mut v = example_buffer();

            v.insert(10);

            assert_eq!(v.pointer, 10);
            v.prev();
            assert_eq!(v.pointer, 0);
            v.next(); v.next();
            assert_eq!(v.pointer, 1);

        }

        #[test]
        fn insert_from_empty() {
            let mut v = CircularBuffer::new(0);
            v.insert(1);
            v.insert(2);

            assert_eq!(v.pointer, 2);
            v.prev();
            assert_eq!(v.pointer, 1);
            v.prev();
            assert_eq!(v.pointer, 0);
            v.prev();
            assert_eq!(v.pointer, 2);

        }
    }

    mod example {
        use super::*;

        #[test]
        fn full_example() {
            let mut c = spinlocker_walker(3, 2018);
            c.next();
            assert_eq!(c.pointer, 638);
        }

        #[test]
        fn small_example() {
            let mut c = spinlocker_walker(3, 4);
            for e in vec![3,1,0,2] {
                assert_eq!(c.pointer, e);
                c.next();
            }
        }
    }
}
