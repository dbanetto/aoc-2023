use std::collections::HashMap;
use std::io;
use std::ops::Range;
use std::str::FromStr;

fn main() {
    let lines = io::stdin().lines();

    let mut state = Parser::SeedList;
    let mut seed_list = Vec::new();
    let mut source = Item::Seed;

    let mut building = Mapping {
        destination: Item::Seed,
        translation: Vec::new(),
    };

    let mut mappings: HashMap<Item, Mapping> = HashMap::new();
    let mut rev_mappings: HashMap<Item, Mapping> = HashMap::new();

    for line in lines {
        if let Some(line) = line.ok() {
            if line == "" {
                if state == Parser::Mapping {
                    let rev = building
                        .translation
                        .clone()
                        .into_iter()
                        .map(|(a, b)| (b, a))
                        .collect();

                    rev_mappings.insert(
                        building.destination,
                        Mapping {
                            destination: source,
                            translation: rev,
                        },
                    );

                    mappings.insert(source, building);

                    building = Mapping {
                        destination: Item::Seed,
                        translation: Vec::new(),
                    }
                }
                state = Parser::Header;
                continue;
            }

            match state {
                Parser::SeedList => {
                    let pairs: Vec<u64> = line
                        .strip_prefix("seeds: ")
                        .unwrap()
                        .split(" ")
                        .map(|s| u64::from_str(s).unwrap())
                        .collect();

                    for pair in pairs.chunks(2) {
                        let range: Range<u64> = pair[0]..(pair[0] + pair[1]);
                        seed_list.push(range)
                    }
                }
                Parser::Header => {
                    let (from, to) = line
                        .strip_suffix(" map:")
                        .unwrap()
                        .split_once("-to-")
                        .unwrap();

                    let from = Item::from_str(from).unwrap();
                    let to = Item::from_str(to).unwrap();

                    source = from;
                    building.destination = to;

                    state = Parser::Mapping
                }
                Parser::Mapping => {
                    let parts: Vec<u64> =
                        line.split(" ").map(|l| u64::from_str(l).unwrap()).collect();

                    let (to_start, from_start, len) = (parts[0], parts[1], parts[2]);

                    let to = to_start..(to_start + len);
                    let from = from_start..(from_start + len);

                    building.translation.push((from, to));
                }
            };
        }
    }

    println!("Seeds {:?}", seed_list);
    println!("Mappings {:?}", mappings);

    let mut smallest: Option<(u64, u64)> = None;

    'outer: for location in 0..u64::MAX {
        let mut cur = Item::Location;
        let mut val = location;

        if location % 1_000_000 == 0 {
            println!("Looking at {}", location);
        }

        while cur != Item::Seed {
            let map = rev_mappings.get(&cur).unwrap();

            (val, cur) = map.translate(val);
        }

        for seed_range in seed_list.iter() {
            if seed_range.contains(&val) {
                println!("Found {} ({}) from location {}", val, seed_range.start, location); 
                break 'outer;
            }
        }
    }

    // for seed_range in seed_list {
    //     println!("Starting {:?}", seed_range);
    //     let range_start = seed_range.start;
    //     for seed in seed_range {
    //         let mut cur = Item::Seed;
    //         let mut val = seed;

    //         while cur != Item::Location {
    //             let map = mappings.get(&cur).unwrap();

    //             (val, cur) =  map.translate(val);
    //         };

    //         smallest = match smallest {
    //             Some((_, sval )) if sval > val => Some(( range_start, val )),
    //             None => Some(( range_start, val )),
    //             _ => smallest,
    //         };
    //     }
    // }

    println!("Smallest {:?}", smallest);
}

#[derive(Debug, PartialEq)]
struct ParseError {}

#[derive(Debug, PartialEq)]
enum Parser {
    SeedList,
    Header,
    Mapping,
}

#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
enum Item {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for Item {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Item::*;
        match s {
            "seed" => Ok(Seed),
            "soil" => Ok(Soil),
            "fertilizer" => Ok(Fertilizer),
            "water" => Ok(Water),
            "light" => Ok(Light),
            "temperature" => Ok(Temperature),
            "humidity" => Ok(Humidity),
            "location" => Ok(Location),
            _ => Err(ParseError {}),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Mapping {
    destination: Item,

    translation: Vec<(Range<u64>, Range<u64>)>,
}

impl Mapping {
    fn translate(&self, num: u64) -> (u64, Item) {
        for (from, to) in self.translation.iter() {
            if from.contains(&num) {
                let trans = (num - from.start) + to.start;

                return (trans, self.destination);
            }
        }

        return (num, self.destination);
    }
}
