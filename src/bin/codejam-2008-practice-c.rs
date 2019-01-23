use codejam::{Scanner, Solve};
use std::cmp::*;
use std::io::{Read, Result, Write};

// https://code.google.com/codejam/contest/32003/dashboard#s=p2
// Problem C. Egg Drop

// http://illya-keeplearning.blogspot.jp/2010/04/google-code-jam-egg-drop-solution.html
// https://groups.google.com/forum/#!topic/google-code/HEgM6Ar9fkw

struct EggDrop {
    md: usize,
    mb: usize,
    cache: Vec<Vec<u64>>,
    maxf: u64,
}

impl EggDrop {
    fn new() -> EggDrop {
        // f(30000, 2) > 4294967295
        let md = 30_000;
        // f(32, 32) >= 4294967295
        let mb = 33;
        let maxf = u64::from(std::u32::MAX);
        let mut cache = vec![vec![0; mb]; md];
        for d in 0..md {
            for b in 0..mb {
                cache[d][b] = {
                    if d == 0 || b == 0 {
                        0
                    } else if b == 1 {
                        d as u64
                    } else {
                        min(cache[d - 1][b] + cache[d - 1][b - 1] + 1, maxf)
                    }
                }
            }
        }
        EggDrop {
            md,
            mb,
            cache,
            maxf,
        }
    }

    fn fmax(&self, d: usize, b: usize) -> u64 {
        // self.cache.get((d, b)).map(|a| *a).unwrap_or_else(|| {
        self.cache
            .get(d)
            .and_then(|rd| rd.get(b))
            .cloned()
            .unwrap_or_else(|| {
                if d == 0 || b == 0 {
                    0
                } else if b == 1 {
                    d as u64
                } else if d < b {
                    self.fmax(d, d)
                } else {
                    self.maxf
                }
            })
    }
}

#[allow(clippy::many_single_char_names)]
impl Solve for EggDrop {
    fn solve<R: Read, W: Write>(&mut self, read: R, write: &mut W) -> Result<()> {
        let mut s = Scanner::new(read);
        let n: usize = s.next();
        for i in 0..n {
            let f: u64 = s.next();
            let d: usize = s.next();
            let b: usize = s.next();

            let fmax = self.fmax(d, b);
            let dmin = codejam::binary_search(1, self.md, |mid| f <= self.fmax(mid, b));
            let bmin = codejam::binary_search(1, self.mb, |mid| f <= self.fmax(d, mid));
            writeln!(
                write,
                "Case #{}: {} {} {}",
                i + 1,
                if fmax == self.maxf { -1 } else { fmax as isize },
                dmin,
                bmin
            )?;
        }
        // for b in 0..100 {
        //     println!("f({}, {}) = {}", b, b, self.fmax(b, b));
        // }
        Ok(())
    }
}

#[test]
fn test() {
    codejam::assert_output(
        EggDrop::new(),
        "2008-practice/C-small-practice.in",
        "2008-practice/C-small-practice.expected",
    );
    codejam::assert_output(
        EggDrop::new(),
        "2008-practice/C-large-practice.in",
        "2008-practice/C-large-practice.expected",
    );
}

fn main() {
    env_logger::init();
    EggDrop::new()
        .solve(std::io::stdin(), &mut std::io::stdout())
        .unwrap();
}
