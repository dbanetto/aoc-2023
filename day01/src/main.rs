use std::io;

fn main() {
    let mut count = 0;

    let lines = io::stdin().lines();

    for line in lines {
        if let Some(line) = line.ok() {
            let (first, last) = translate_line(line.as_ref());
            count += (first * 10) + last;
        }
    }

    println!("{:?}", count);
}

fn translate_line(line: &str) -> (i32, i32) {
    let mut first: Option<(usize, i32)> = None;
    let mut last: Option<(usize, i32)> = None;

    for (word, val) in [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ] {
        if let Some(idx) = line.find(word) {
            first = match first {
                None => Some((idx, val)),
                Some((fidx, _)) if fidx >= idx => Some((idx, val)),
                _ => first,
            };
        }

        if let Some(idx) = line.rfind(word) {
            last = match last {
                None => Some((idx, val)),
                Some((lidx, _)) if lidx <= idx => Some((idx, val)),
                _ => last,
            };
        }
    }

    return (first.unwrap_or_default().1, last.unwrap_or_default().1);
}
