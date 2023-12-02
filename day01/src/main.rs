use std::io;

fn main() {

    let mut count = 0;

    let lines = io::stdin().lines();

    for line in lines {

        if let Ok(line) = line {
            let digits = line
                .chars()
                .filter_map(|char| {
                    char.to_digit(10)
                });

            
            let first = digits.clone().next();
            let last = digits.last();

            let val = match (first, last) {
                (Some(f), Some(l)) => (f * 10) + l,
                (_, _) => continue,
            };
            
            count += val;
        }
    }

    println!("{:?}", count);
}
