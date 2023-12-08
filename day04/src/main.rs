use std::collections::HashSet;
use std::io;
use std::str::FromStr;

fn main() {
    let mut points = 0;

    for line in io::stdin().lines() {
        if let Some(line) = line.ok() {

            let card = Card::from_str(&line).unwrap();
            println!("{:?}", card);

            points += card.points()
        }
    }

    println!("Points: {}", points)
}

#[derive(Debug)]
struct Card {
    id: i32,
    winning: HashSet<i32>,
    game: HashSet<i32>,
}

#[derive(Debug)]
struct ParseCardErr;

impl FromStr for Card {
    type Err = ParseCardErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (prefix, nums) = s.split_once(":").ok_or(ParseCardErr {})?;

        let id = i32::from_str(prefix.strip_prefix("Card").ok_or(ParseCardErr {})?.trim())
            .map_err(|_| ParseCardErr {})?;

        let (winning, game) = nums.split_once("|").ok_or(ParseCardErr {})?;

        Ok(Card {
            id,
            winning: nums_to_list(winning),
            game: nums_to_list(game),
        })
    }
}

impl Card {
    fn points(&self) -> i32 {
        let intersection = self.winning.intersection(&self.game);

        let count: u32 = intersection.count().try_into().unwrap();
        if count == 1 {
            return 1
        } else if count > 1 {
            let base: i32 = 2;
            base.pow(count - 1)
        } else {
            0
        }
    }
}

fn nums_to_list(s: &str) -> HashSet<i32> {
    return HashSet::from_iter(
        s.trim()
            .split(" ")
            .filter_map(|s| i32::from_str(s.trim()).ok()),
    );
}
