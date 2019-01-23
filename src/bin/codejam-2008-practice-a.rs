use codejam::{Scanner, Solve};
use std::io::{Read, Result, Write};

// https://code.google.com/codejam/contest/32003/dashboard
// Problem A. Alien Numbers
struct AlienNumbers;

impl AlienNumbers {
    fn new() -> AlienNumbers {
        AlienNumbers
    }
}

impl Solve for AlienNumbers {
    fn solve<R: Read, W: Write>(&mut self, read: R, write: &mut W) -> Result<()> {
        let mut s = Scanner::new(read);
        let n = s.next();
        for i in 0..n {
            let num_s = s.next_bytes();
            let source = s.next_bytes();
            let target = s.next_bytes();
            let mut n = 0;
            for c in &num_s {
                n *= source.len();
                n += source.iter().position(|i| i == c).unwrap();
            }
            let mut s = Vec::new();
            while n > 0 {
                let a = n % target.len();
                s.push(target[a]);
                n /= target.len();
            }
            let s = s.into_iter().rev().collect::<Vec<_>>();
            writeln!(
                write,
                "Case #{}: {}",
                i + 1,
                std::str::from_utf8(&s).unwrap()
            )?;
        }
        Ok(())
    }
}

#[test]
fn test() {
    codejam::assert_output(
        AlienNumbers::new(),
        "2008-practice/A-small-practice.in",
        "2008-practice/A-small-practice.expected",
    );
    codejam::assert_output(
        AlienNumbers::new(),
        "2008-practice/A-large-practice.in",
        "2008-practice/A-large-practice.expected",
    );
}

fn main() {
    AlienNumbers::new()
        .solve(std::io::stdin(), &mut std::io::stdout())
        .unwrap();
}
