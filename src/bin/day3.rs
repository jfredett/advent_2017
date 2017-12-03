fn main() {
  println!("Advent of Code Day 3");
  println!("");
  println!("Spreadsheet: http://adventofcode.com/2017/day/3");
  println!("");


  /* /////////////////////////////////////////////////////////

     First, notice that the side of each successive ring around the center is equal
     to the `2n+1`, starting at `n=0`.

     Next, notice that drawing a line starting at 1 and proceeding down and to the
     right one square is always equal to `(2n+1)^2`. We can work out this relative
     difference for the whole grid like this:

     -----

     Let N(n) = (2n+1)^2

           N(n-1) + 4n              N(n-1) + 3n                   N(n-1) + 2n

                         \               |                    /

                            N(0) + 4   N(0) + 3    N(0) + 2

           N(n) - 3n  ...   N(1) - 3   N(0)        N(0) + 1  ...  N(n-1)  + n

                            N(1) - 2   N(1) - 1    N(1)      -> increment `n` and repeat

                         /               |                    \   < this contains 2n - 1 elements (half the bar), not including the center or corner>

           N(n) - 2n                 N(n) - n                     N(n)



     Using this, and the fact that routing _from_ the target _to_ the origin is the
     same as routing _from_ the origin _to_ the target, all we need to establish is:

     1. What the relative shift left/right of the center the value is, call it `LR`
     2. What the shift above/below the center we are, call it `UD`

     The Manhattan distance is just `LR + UD`.

     However, one of those values will actually just be the ring value. If the target
     is on a horizontal 'bar' of the ring, then the UD value will be the ring value,
     and the LR value will be it's absolute shift off the centerline; if it's on the
     vertical 'bar', then vice versa. Fortunately, once we know the ring #, it's very
     easy to exploit the structure of the spiral to figure out the rest.

     We know the length of a side is `n`, so we can start by determining how far the
     target is from the nearest perfect square (or any other corner). Our target
     (given in `day3/part1/input` is: 368078

     It's square root is: 606.69, so the next highest square is `607^2 = 368449`

     This indicates it's in ring `2n + 1 = 607 => 303`

     To determine which row it is in, we can take the difference, `368449 - 368078 =
     371` The bars of that ring is 607 values wide, so this indicates it's on the
     left side of ring 303. So `UD = ring = 303`. Additionally, since `371 <
     607`, we know we're on the bottom bar (we would need to be `~3*607` to end up
     walking around to the top bar.

     Since the center of the bottom bar is `N(n) - n`, we can take the absolute
     difference of this from our target to get `LR`, it should be less than half the
     bar length (for obvious reasons):

     607^2 - 303 = 368146; 368146 - 368078 = 68

     So we're relatively close to the centerline, `LR = 68`,

     Therefore, the Manhattan distance is `LR + UD = 303 + 68 = 371` -- the fact that
     this is equal to the difference is a coincidence, I believe.

  */ /////////////////////////////////////////////////////////
  println!("Part1: 371");
  /* /////////////////////////////////////////////////////////

     Part 2 is a little tougher. We need to distill that weird function into
     something a bit more contained.

     The complications come in two parts. First, every piece is now
     self-referential, it is the sum of the squares around it, and notably it's
     only filled with values from the square around it that _are already
     filled_.

     The second is that we're looking for the first value that's _larger_ than
     our target (the same as before).

     Here's an approach, though. Let's assume that this sequence is recurrent
     polynomial, that is, it is definied by a recurrence relation that is
     polynomial over `n`, the index of recurrence, and it's function, in
     notation:

         M(n) = An^k + An^k-1 + ... + BM(n)^j + ...

     With this assumption, we can generate a few example values, and then look
     at their discrete derivatives to try to suss out a pattern. Here's a chunk
     of the spiral I pulled from the example (with a few extra entries filled in:


      147 142 133 122  59
      304   5   4   2  57
      330  10   1   1  54
      351  11  23  25  26
      362 747 806 854 905  931

     The sequence, `M(n)` is:

        1, 1, 54, 57, 59, 122, 133, 142, 147, 304, 330, 351, 362, 747, 806, 854, 905, 931

     The first discrete derivative is:

        D(M,n) = M(n) - M(n-1) = {X, 0, 53, 3, 2, 63, 11, 9, 5, 157, 21 11, 385, 59, 48, 51, 26}

     The second discrete derivative is:

        D(D(M,n),n) = {X, X, 53, -50, -1, ...

     This is unlikely to be fruitful. I think this is where we dump out to
     computing to solve the problem.

     There might be an interesting approach where we regard the spiral as a
     graph, and then maybe we can exploit some pattern in the graph structure to
     make the thing easier to compute, but in any case, the datastructure for
     this is pretty natural -- just a list of coordinates and their values,
     along with a function to 'populate' the next value in the spiral. We need
     to know the function to map from the one-dimensional index to the
     two-dimensional position on the graph, we can use the work from above to
     help with that. Each ring `n` starts with a lowest value of  `N(n-1) + n`
     and has a maximum value of  `N(n) + 2n - 1`, i.e., each ring contains:

          (N(n) + 2n - 1) - (N(n-1) + n) = (2n+1)^2 + 2n - 1 - (2(n-1) + 1)^2 - n)
                                         = 9n - 1

     (note that this applies to all but the 0th ring, which has 1 element).

     Since each corner of the ring contains `2n + 1` elements, we simply need to 
     travel up `2n` elements (including the corner), left `2n`, down `2n`, and
     right `2n` before incrementing `n` and repeating. The natural datastructure
     here is a sparse matrix.

     Another approach (and the one I use) is to work out the conversion I detail
     above so you can convert a 'spiral' index directly to a point, with (0,0)
     at the origin.

  */ /////////////////////////////////////////////////////////
}


mod day3 {
  use std::collections::HashMap;

  #[derive(Debug, PartialEq, Eq, Hash)]
  struct Point {
    x: i32,
    y: i32
  }

  impl Point {
    pub fn new(x: i32, y: i32) -> Point {
      return Point { x: x, y: y };
    }
  }

  #[derive(Debug, PartialEq, Eq)]
  struct SparseMatrix {
    pointer: Point,
    coordinates: HashMap<Point, i32>
  }

  impl SparseMatrix {
    pub fn new() -> SparseMatrix {
      let mut sm = SparseMatrix { 
        pointer: Point::new(0,0),
        coordinates: HashMap::new()
      };

      sm.set(Point::new(0,0), 1);

      return sm;
    }

    pub fn set(&mut self, p: Point, v: i32) {
      self.coordinates.insert(p,v);
    }

    pub fn get(&self, p: Point) -> i32 {
      match self.coordinates.get(&p) {
        Some(v) => return *v,
        None => return 0
      }
    }

    pub fn spiral_to_point(idx: i32) -> Point {
      // the ring in which the index resides
      let mut ring = (idx as f32).sqrt().ceil() as i32;
      // we want the perfect square on the down-left diagonal
      if ring % 2 == 0 { ring += 1; }

      let n = (ring - 1) / 2; // this is the index of the ring, we'll need it later

      let x: i32;
      let y: i32;

      let bottom_left_corner = (2*n + 1).pow(2) - 2*n;
      let top_left_corner = (2*n - 1).pow(2) + 4*n;
      let top_right_corner = (2*n - 1).pow(2) + 2*n;

      if idx > bottom_left_corner {
        let bottom_axis = (2*n + 1).pow(2) - n;
        // if idx > center, then this is negative (and we're to the left of
        // the y-axis, otherwise we're positive and to the right).
        x = idx - bottom_axis;
        // we're left the y-axis
        y = -n;
      } else if idx > top_left_corner {
        let left_axis = (2*n + 1).pow(2) - 3*n;
        // below the x axis
        x = -n;
        y = idx - left_axis;
      } else if idx > top_right_corner {
        let top_axis = (2*n - 1).pow(2) + 3*n;
        x = top_axis - idx;
        // we're right of the y-axis
        y = n;
      } else { // we're on the right bar
        let right_axis = (2*n - 1).pow(2) + n;
        // we're above the x-axis
        x = n;
        y = idx - right_axis;
      }

      return Point::new(x,y);

    }
  }

  #[cfg(test)]
  mod point_tests {
    use super::*;

    #[test]
    fn new_happy() {
      let p = Point::new(1,2);
      assert_eq!(p.x, 1);
      assert_eq!(p.y, 2);
    }
  }

  #[cfg(test)]
  mod sparse_matrix_tests {
    use super::*;

    #[test]
    fn new_happy()  {
      let sm = SparseMatrix::new();
      assert_eq!(sm.get(Point::new(0,0)), 1);
    }

    //#[test]
    //fn get_automatically_calculates_and_populates() {
      //let sm = SparseMatrix::new();
      //assert_eq(sm.get(Point::new(1,
    //}

    #[test]
    fn spiral_to_point_1() {
      let p = SparseMatrix::spiral_to_point(5);
      assert_eq!(p.x, -1);
      assert_eq!(p.y, 1);
    }

    #[test]
    fn spiral_to_point_2() {
      let p = SparseMatrix::spiral_to_point(16);
      assert_eq!(p.x, -1);
      assert_eq!(p.y, 2);
    }

    #[test]
    fn spiral_to_point_3() {
      let p = SparseMatrix::spiral_to_point(12);
      assert_eq!(p.x, 2);
      assert_eq!(p.y, 1);
    }

    #[test]
    fn spiral_to_point_4() {
      let p = SparseMatrix::spiral_to_point(22);
      assert_eq!(p.x, -1);
      assert_eq!(p.y, -2);
    }

    #[test]
    fn spiral_to_point_5() {
      let p = SparseMatrix::spiral_to_point(22);
      assert_eq!(p.x, -1);
      assert_eq!(p.y, -2);
    }

    #[test]
    fn spiral_to_point_on_axis_1() {
      let p = SparseMatrix::spiral_to_point(11);
      assert_eq!(p.x, 2);
      assert_eq!(p.y, 0);
    }

    #[test]
    fn part1_test() {
      // we don't need this for solving the puzzle, but it's a good test of the
      // spiral/index conversion
      let p = SparseMatrix::spiral_to_point(368078);
      assert_eq!(p.x, -68);
      assert_eq!(p.y, -303);
    }
  }
}

