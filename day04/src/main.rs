use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::str::FromStr;

fn main() {
    let mut points = 0;
    let mut count = 0;

    let mut card_queue: VecDeque<u32> = VecDeque::new();

    card_queue.resize_with(25, || 1);

    for line in io::stdin().lines() {
        if let Some(line) = line.ok() {
            let extra = card_queue.pop_front().unwrap_or_default();
            card_queue.push_back(1);

            let card = Card::from_str(&line).unwrap();
            // println!("{:?}", card);

            let duplicates = card.matching();

            for (val, _) in card_queue.iter_mut().zip(0..duplicates) {
                *val += extra;
            }

            points += card.points();
            count += extra;
        }
    }

    println!("Points: {}", points);
    println!("Count: {}", count);
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
    fn matching(&self) -> u32 {
        let intersection = self.winning.intersection(&self.game);

        return intersection.count().try_into().unwrap();
    }

    fn points(&self) -> i32 {
        let count = self.matching();

        if count == 1 {
            return 1;
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
