use std::io;
use std::str::FromStr;

fn main() {
    let mut counter = 0;
    let mut prev = 0;

    for line in io::stdin().lines() {
        if let Some(line) = line.ok() {
            if let Ok(s) = Series::from_str(&line) {
                println!("{}", line);
                counter += s.predict_next();
                prev += s.predict_prev();
            } else {
                panic!("AHHH {}", line);
            }
        }
    }

    println!("Count: {}", counter);
    println!("Previous: {}", prev);
}

struct Series {
    numbers: Vec<i32>,
}

struct ParseError {}

impl FromStr for Series {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split_ascii_whitespace()
            .map(|n| i32::from_str(n).unwrap())
            .collect();

        return Ok(Series { numbers: nums });
    }
}

impl Series {
    fn predict_next(&self) -> i32 {
        if self.numbers.iter().fold(true, |acc, n| *n == 0 && acc) {
            0
        } else {
            self.numbers.iter().last().unwrap() + self.deriviative().predict_next()
        }
    }

    fn predict_prev(&self) -> i32 {
        if self.numbers.iter().fold(true, |acc, n| *n == 0 && acc) {
            0
        } else {
            self.numbers.iter().next().unwrap() - self.deriviative().predict_prev()
        }
    }

    fn deriviative(&self) -> Series {
        let mut derv = self.numbers.clone().into_iter();

        let mut prev = derv.next().unwrap();

        let derv = derv.map(|n| {
            let diff = n - prev;
            prev = n;
            diff
        });

        return Series {
            numbers: derv.collect(),
        };
    }
}
