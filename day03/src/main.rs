use std::io;

fn main() {
    let mut board = Board {
        last: None,
        current: None,
        next: None,
        counter: 0,
        gear_ratio: 0,
    };

    for line in io::stdin().lines() {
        if let Some(line) = line.ok() {
            board = board.next(&line);
        }
    }
    // Carry over line for the end
    board = board.next("");

    println!("Counter: {}", board.counter);
    println!("Gear Ratio {}", board.gear_ratio);
}

#[derive(Debug)]
enum Span {
    Number {
        start: usize,
        end: usize,
        value: i32,
    },
    Symbol(usize),
    Gear(usize),
}

#[derive(Debug)]
struct Board {
    last: Option<Vec<Span>>,
    current: Option<Vec<Span>>,
    next: Option<Vec<Span>>,
    counter: i32,
    gear_ratio: i32,
}

impl Board {
    fn next(self, line: &str) -> Board {
        let mut block = Vec::new();
        let mut buf = String::new();

        for (idx, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                buf.push(char);
            } else if buf.len() != 0 {
                let val = buf.parse::<i32>().unwrap();

                block.push(Span::Number {
                    start: idx - buf.len(),
                    end: idx - 1,
                    value: val,
                });

                buf.clear();
            }
            if !char.is_digit(10) && char == '*' {
                block.push(Span::Gear(idx));
            } else if !char.is_digit(10) && char != '.' {
                block.push(Span::Symbol(idx));
            }
        }
        if buf.len() != 0 {
            let val = buf.parse::<i32>().unwrap();

            block.push(Span::Number {
                start: line.len() - buf.len(),
                end: line.len(),
                value: val,
            });

            buf.clear();
        }

        let counter = self.counter + self.calc();
        let ratio = self.gear_ratio + self.ratio();

        return Board {
            last: self.current,
            current: self.next,
            next: Some(block),

            counter: counter,
            gear_ratio: ratio,
        };
    }

    fn calc(&self) -> i32 {
        let current = match &self.current {
            Some(line) => line,
            None => return 0,
        };

        let symbols = current.iter().filter_map(|p| match p {
            Span::Symbol(i) => Some(i),
            Span::Gear(i) => Some(i),
            _ => None,
        });

        let mut val: i32 = 0;
        for symbol in symbols {
            let symbol = *symbol;

            if let Some(prev) = &self.last {
                val += find_adjacent(symbol, prev).iter().sum::<i32>();
            }

            if let Some(cur) = &self.current {
                val += find_adjacent(symbol, cur).iter().sum::<i32>();
            }

            if let Some(nex) = &self.next {
                val += find_adjacent(symbol, nex).iter().sum::<i32>();
            }
        }

        return val;
    }

    fn ratio(&self) -> i32 {
        let current = match &self.current {
            Some(line) => line,
            None => return 0,
        };

        let symbols = current.iter().filter_map(|p| match p {
            Span::Gear(i) => Some(i),
            _ => None,
        });

        let mut val: i32 = 0;
        for symbol in symbols {
            let symbol = *symbol;
            let mut found = Vec::new();

            if let Some(prev) = &self.last {
                found.append(&mut find_adjacent(symbol, prev));
            }

            if let Some(cur) = &self.current {
                found.append(&mut find_adjacent(symbol, cur));
            }

            if let Some(nex) = &self.next {
                found.append(&mut find_adjacent(symbol, nex));
            }

            if found.len() == 2 {
                val += found.iter().fold(1, |acc, i| acc * i)
            }
        }

        return val;
    }
}

fn find_adjacent(idx: usize, list: &Vec<Span>) -> Vec<i32> {
    list.iter()
        .filter_map(|s| match s {
            Span::Number { start, end, value } => {
                if idx + 1 >= *start && idx - 1 <= *end {
                    Some(*value)
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect()
}
