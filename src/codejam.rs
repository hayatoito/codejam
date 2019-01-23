use regex::Regex;

use std;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use std::ops::Deref;
use std::str::FromStr;

pub struct Scanner<R: Read> {
    reader: BufReader<R>,
    buffer: VecDeque<String>,
}

impl<R: Read> Scanner<R> {
    pub fn new(read: R) -> Scanner<R> {
        Scanner {
            reader: BufReader::new(read),
            buffer: VecDeque::new(),
        }
    }

    fn read_line(&mut self) {
        let mut input = String::new();
        self.reader.read_line(&mut input).ok();
        for word in input.split_whitespace() {
            self.buffer.push_back(word.to_string())
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next<T: FromStr>(&mut self) -> T {
        if self.buffer.is_empty() {
            self.read_line();
        }
        let front = self.buffer.pop_front().unwrap();
        front.parse::<T>().ok().unwrap()
    }

    pub fn next_to_end_of_line<T: FromStr>(&mut self) -> Vec<T> {
        if self.buffer.is_empty() {
            self.read_line();
        }
        self.buffer
            .drain(0..)
            .map(|i| i.parse::<T>().ok().unwrap())
            .collect()
    }

    pub fn next_bytes(&mut self) -> Vec<u8> {
        self.next::<String>().into_bytes()
    }
}

pub trait Solve {
    fn solve<R: Read, W: Write>(&mut self, read: R, write: &mut W) -> Result<()>;
}

fn read(path: &str) -> String {
    let f = File::open(path).expect("Can not open");
    let mut f = BufReader::new(f);
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s
}

struct TestCasesOutput(Vec<String>);

impl FromStr for TestCasesOutput {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let re = Regex::new(r#"Case #(\d+):"#).unwrap();
        let pos = re.find_iter(s).collect::<Vec<_>>();
        let mut cases = vec![];
        for (m0, m1) in pos.iter().zip(pos.iter().skip(1)) {
            cases.push(s[m0.start()..m1.start()].to_string());
        }
        cases.push(s[(pos.last().unwrap().start())..].to_string());
        Ok(TestCasesOutput(cases))
    }
}

impl Deref for TestCasesOutput {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn assert_output<S: Solve>(mut s: S, input: &str, expected: &str) {
    fn file_path(path: &str) -> String {
        format!("{}/problems/{}", env!("CARGO_MANIFEST_DIR"), path)
    }

    let input_path = file_path(input);
    let mut output: Vec<u8> = vec![];
    s.solve(
        File::open(&input_path).expect("can not open input file"),
        &mut output,
    )
    .unwrap();
    let actual: TestCasesOutput = std::str::from_utf8(&output).unwrap().parse().unwrap();

    let expected_path = file_path(expected);
    let expected: TestCasesOutput = read(&expected_path).parse().unwrap();

    assert_eq!(actual.len(), expected.len());
    for (a, e) in actual.iter().zip(expected.iter()) {
        assert_eq!(a, e);
    }
}

pub fn binary_search<T, F>(mut low: T, mut high: T, mut f: F) -> T
where
    T: num::Num + std::cmp::PartialOrd + Copy,
    F: FnMut(T) -> bool,
{
    let one = T::one();
    let two = one + one;
    while low < high {
        let mid = low + (high - low) / two;
        if f(mid) {
            high = mid;
        } else {
            low = mid + one;
        }
    }
    low
}

#[cfg(test)]
mod tests {

    use super::binary_search;
    use std::collections::BTreeSet;

    #[test]
    fn b0() {
        let xs = vec![14, 14];
        let x = 14;
        let bound0 = binary_search(0, xs.len(), |i| x <= xs[i]);
        assert_eq!(bound0, 0);
    }

    #[test]
    fn b1() {
        let xs = vec![14, 14];
        let x = 14;
        let bound1 = {
            match xs.binary_search(&x) {
                Ok(n) => n,
                Err(n) => n,
            }
        };
        // Vec::binary_search does not return a lower bound if vec has dups.
        // assert_eq!(bound1, 0);
        assert_eq!(bound1, 1);
    }

    quickcheck! {
      fn binary_search_is_identical(xs: Vec<isize>, x: isize) -> bool {
          let xs = xs
              .into_iter()
              .collect::<BTreeSet<_>>()
              .into_iter()
              .collect::<Vec<_>>();
          let bound0 = binary_search(0, xs.len(), |i| x <= xs[i]);
          let bound1 = {
              match xs.binary_search(&x) {
                  Ok(n) => n,
                  Err(n) => n,
              }
          };
          bound0 == bound1
      }
    }

}
