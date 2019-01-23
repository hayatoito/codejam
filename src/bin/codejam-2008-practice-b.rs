use codejam::{Scanner, Solve};
use std::cmp::*;
use std::io::{Read, Result, Write};

use std::collections::HashMap;

// https://code.google.com/codejam/contest/32003/dashboard#s=p1
// Problem B. Always Turn Left

#[derive(Clone, Copy)]
enum Dir {
    N = 0,
    S = 1,
    W = 2,
    E = 3,
}

use Dir::{E, N, S, W};

impl Dir {
    fn left_turn(self) -> Dir {
        match self {
            N => W,
            S => E,
            W => S,
            E => N,
        }
    }

    fn right_turn(self) -> Dir {
        self.left_turn().neg()
    }

    fn neg(self) -> Dir {
        match self {
            N => S,
            S => N,
            W => E,
            E => W,
        }
    }

    fn mark(self) -> u8 {
        1 << self as u8
    }
}

struct AlwaysTurnLeft;

impl AlwaysTurnLeft {
    fn new() -> AlwaysTurnLeft {
        AlwaysTurnLeft
    }
}

type Maze = HashMap<(isize, isize), u8>;

impl Solve for AlwaysTurnLeft {
    fn solve<R: Read, W: Write>(&mut self, read: R, write: &mut W) -> Result<()> {
        let mut s = Scanner::new(read);
        let n = s.next();
        for i in 0..n {
            let path0 = s.next_bytes();
            let path1 = s.next_bytes();
            let mut maze = Maze::new();

            let mut r: isize = 0;
            let mut c: isize = 0;

            let mut maxr = r;
            let mut minc = c;
            let mut maxc = c;

            #[allow(clippy::too_many_arguments)]
            fn walk_path(
                path: &[u8],
                maxr: &mut isize,
                minc: &mut isize,
                maxc: &mut isize,
                r: &mut isize,
                c: &mut isize,
                dir: Dir,
                maze: &mut Maze,
            ) -> Dir {
                let mut dir = dir;
                for &b in path.iter() {
                    // println!("r: {}, c: {}", r, c);
                    match b as char {
                        'W' => {
                            *(maze.entry((*r, *c)).or_insert(0)) |= dir.mark();
                            match dir {
                                N => *r -= 1,
                                S => *r += 1,
                                W => *c -= 1,
                                E => *c += 1,
                            };
                            *(maze.entry((*r, *c)).or_insert(0)) |= dir.neg().mark();
                            *maxr = max(*maxr, *r);
                            *minc = min(*minc, *c);
                            *maxc = max(*maxc, *c)
                        }
                        'L' => dir = dir.left_turn(),
                        'R' => dir = dir.right_turn(),
                        _ => unreachable!(),
                    }
                }
                dir
            };

            let dir = walk_path(
                &path0, &mut maxr, &mut minc, &mut maxc, &mut r, &mut c, S, &mut maze,
            );
            walk_path(
                &path1,
                &mut maxr,
                &mut minc,
                &mut maxc,
                &mut r,
                &mut c,
                dir.neg(),
                &mut maze,
            );

            assert_eq!((r, c), (0, 0));

            let r1 = match dir {
                S => maxr - 1,
                W | E | N => maxr,
            };
            let (c0, c1) = match dir {
                S | N => (minc, maxc),
                W => (minc + 1, maxc),
                E => (minc, maxc - 1),
            };

            writeln!(write, "Case #{}:", i + 1)?;
            for row in 1..=r1 {
                writeln!(
                    write,
                    "{}",
                    (c0..=c1)
                        .map(|col| format!("{:x}", maze[&(row, col)]))
                        .collect::<Vec<String>>()
                        .concat()
                )?;
            }
        }
        Ok(())
    }
}

#[test]
fn test() {
    codejam::assert_output(
        AlwaysTurnLeft::new(),
        "2008-practice/B-small-practice.in",
        "2008-practice/B-small-practice.expected",
    );
    codejam::assert_output(
        AlwaysTurnLeft::new(),
        "2008-practice/B-large-practice.in",
        "2008-practice/B-large-practice.expected",
    );
}

fn main() {
    AlwaysTurnLeft::new()
        .solve(std::io::stdin(), &mut std::io::stdout())
        .unwrap();
}
