use std::collections::HashMap;
use std::io;
use std::str::FromStr;

fn main() {
    let mut map: HashMap<String, Doors> = HashMap::new();
    let mut parser = Parser::Directions;
    let mut choices = Vec::new();

    let mut pointers = Vec::new();

    for line in io::stdin().lines() {
        if let Some(line) = line.ok() {
            match parser {
                Parser::Directions => {
                    if line.is_empty() {
                        parser = Parser::Doors;
                    } else {
                        choices = line
                            .chars()
                            .map(|c| match c {
                                'L' => Choice::Left,
                                'R' => Choice::Right,
                                _ => panic!("??"),
                            })
                            .collect();
                    };
                }
                Parser::Doors => {
                    if let Ok(door) = Doors::from_str(&line) {
                        if door.label.ends_with("A") {
                            pointers.push(door.label.clone());
                        }

                        let _ = map.insert(door.label.clone(), door);
                    }
                }
            };
        }
    }

    let mut counter = 0;

    for choice in choices.iter().cycle() {
        counter += 1;

        pointers = pointers
            .into_iter()
            .map(|pointer| {
                if let Some(next) = map.get(&pointer) {
                    match *choice {
                        Choice::Left => next.left.clone(),
                        Choice::Right => next.right.clone(),
                    }
                } else {
                    panic!("{} not found", pointer);
                }
            })
            .collect();

        if pointers.iter().enumerate().fold(
                true,
                |acc, (idx, p)| {
                    if p.ends_with("Z") {
                        // collect the first iteration of each of these then
                        // calculate the LCM :tada:
                        println!("{} - {}", idx, counter);
                        acc && true
                    } else {
                        false
                    }
                },
            )
        {
            break;
        }

        // println!("{} - {:?}", counter, pointers);
    }

    println!("Got to it in: {} steps", counter);
}

enum Parser {
    Directions,
    Doors,
}

#[derive(Debug)]
enum Choice {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Doors {
    label: String,

    left: String,
    right: String,
}

#[derive(Debug)]
struct ParseError {}

impl FromStr for Doors {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, vals) = s.split_once("=").unwrap();

        let (l, r) = vals.split_once(", ").unwrap();

        return Ok(Doors {
            label: label.trim().to_owned(),

            left: l.trim_start_matches(" (").to_owned(),
            right: r.trim_end_matches(")").to_owned(),
        });
    }
}
