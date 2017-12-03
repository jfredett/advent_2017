


fn main() {
  println!("Advent of Code Day 3");
  println!("");
  println!("Spreadsheet: http://adventofcode.com/2017/day/3");
  println!("");

  let mut part1_spreadsheet_file = File::open("data/day3/input").expect("file not found");
  let mut part1_spreadsheet = String::new();
  part1_spreadsheet_file.read_to_string(&mut part1_spreadsheet).expect("Something went wrong reading part1 file");

  /* /////////////////////////////////////////////////////////

     First, notice that the side of each successive ring around the center is equal
     to the `2n+1`, starting at `n=0`.

     Next, notice that drawing a line starting at 1 and proceeding down and to the
     right one square is always equal to `(2n+1)^2`. We can work out this relative
     difference for the whole grid like this:

     -----

     Let N(n) = (2n+1)^2


     N(n-1) + 4n                             N(n) + 3n                             N(n-1) + 2n

     N(n) - 4n + 1   N(1) + 8    N(1) + 7    N(1) + 6     N(1) + 5      N(1) + 4   ...

     N(2) - 7    N(0) + 4    N(0) + 3     N(0) + 2      N(1) + 3   ...

     N(n) - 3n       N(2) - 6    N(1) - 3    N(0)         N(0) + 1      N(1) + 2   ...     N(n) + n

     N(2) - 5    N(1) - 2    N(1) - 1     N(1)          N(1) + 1   ...

     N(2) - 4    N(2) - 3    N(2) - 2     N(2) - 1      N(2)       ...

     ...                                           N(n) + 2

     N(n) - 4n                               N(n) - n     N(n) - (n-1)  ...        N(n)    N(n) + 1


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
  println!("Part1: 371"));
}

