use std::cmp::Ordering;
use std::io;
use std::str::FromStr;

fn main() {
    let mut hands: Vec<Hand> = Vec::new();
    for line in io::stdin().lines() {
        if let Some(line) = line.ok() {
            let hand = Hand::from_str(&line).unwrap();
            // println!("{:?} {:?} {:?}", hand.cards, hand.to_strength(), hand.points);

            hands.push(hand);
        }
    }

    hands.sort_by(|a, b| {
        let a_str = a.to_strength();
        let b_str = b.to_strength();

        return a_str.partial_cmp(&b_str).unwrap();
    });
    hands.reverse();

    let mut total = 0;

    for (idx, hand) in hands.iter().enumerate() {
        println!("{} {:?}", idx, hand.to_strength());
        total += (idx as u32 + 1) * hand.points;
    }
    println!("Total {}", total);
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
struct Card(u8);

#[derive(Debug, PartialEq)]
enum Winner {}

#[derive(Debug)]
struct ParseError {}

impl FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = match s.to_lowercase().as_str() {
            "a" => 0,
            "k" => 1,
            "q" => 2,
            "j" => 3,
            "t" => 4,
            "9" => 5,
            "8" => 6,
            "7" => 7,
            "6" => 8,
            "5" => 9,
            "4" => 10,
            "3" => 11,
            "2" => 12,
            _ => return Err(ParseError {}),
        };

        return Ok(Card(val));
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    points: u32,
}

impl FromStr for Hand {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hand = Hand {
            cards: Vec::new(),
            points: 0,
        };

        let (left, right) = s.split_once(" ").ok_or(ParseError {})?;

        for card in left.chars() {
            hand.cards.push(Card::from_str(&card.to_string())?)
        }

        hand.points = u32::from_str(right).map_err(|_| ParseError {})?;

        Ok(hand)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Strengh {
    FiveKind { cards: Vec<Card> },
    FourKind { cards: Vec<Card> },
    FullHouse { cards: Vec<Card> },
    ThreeKind { cards: Vec<Card> },
    TwoPair { cards: Vec<Card> },
    Pair { cards: Vec<Card> },
    High { cards: Vec<Card> },
    // FiveKind {
    //     card: Card
    // },
    // FourKind {
    //     card: Card
    // },
    // FullHouse {
    //     three: Card,
    //     two: Card,
    // },
    // ThreeKind {
    //     three: Card,
    // },
    // TwoPair {
    //     first: Card,
    //     second: Card,
    // },
    // Pair {
    //     card: Card,
    // },
    // High {
    //     card: Card,
    // },
}

impl Hand {
    fn to_strength(&self) -> Strengh {
        use std::collections::HashMap;

        let count: HashMap<Card, u32> = self.cards.iter().fold(HashMap::new(), |mut acc, c| {
            acc.entry(*c)
                .and_modify(|q: &mut u32| *q += 1)
                .or_insert_with(|| 1);
            acc
        });

        let mut count: Vec<(Card, u32)> = count.into_iter().collect::<Vec<_>>();

        count.sort_by(|(a_card, a_count), (b_card, b_count)| {
            let count_cmp = a_count.cmp(b_count);
            if count_cmp == Ordering::Equal {
                a_card.cmp(b_card)
            } else {
                count_cmp
            }
        });
        count.reverse();

        if count.len() == 1 {
            let (card, _) = count.get(0).unwrap();
            return Strengh::FiveKind {
                cards: self.cards.clone(),
            };
        } else if count.len() == 2 {
            let mut iter = count.iter();
            let (card_a, count_a) = iter.next().unwrap();
            let (card_b, count_b) = iter.next().unwrap();

            match (card_a, count_a, card_b, count_b) {
                (a, 3, b, 2) => {
                    return Strengh::FullHouse {
                        cards: self.cards.clone(),
                    }
                }
                (a, 2, b, 3) => {
                    return Strengh::FullHouse {
                        cards: self.cards.clone(),
                    }
                }
                (a, 4, _, 1) => {
                    return Strengh::FourKind {
                        cards: self.cards.clone(),
                    }
                }
                (_, 1, b, 4) => {
                    return Strengh::FourKind {
                        cards: self.cards.clone(),
                    }
                }
                _ => (),
            };
        } else if count.len() == 3 {
            let mut iter = count.iter();
            let (card_a, count_a) = iter.next().unwrap();
            let (card_b, count_b) = iter.next().unwrap();

            match (card_a, count_a, card_b, count_b) {
                (a, 3, _, _) => {
                    return Strengh::ThreeKind {
                        cards: self.cards.clone(),
                    }
                }
                (a, 2, b, 2) => {
                    return Strengh::TwoPair {
                        cards: self.cards.clone(),
                    }
                }
                _ => (),
            };
        } else if count.len() == 4 {
            let mut iter = count.iter();
            let (card_a, count_a) = iter.next().unwrap();

            match (card_a, count_a) {
                (a, 2) => {
                    return Strengh::Pair {
                        cards: self.cards.clone(),
                    }
                }
                _ => (),
            };
        }

        let mut iter = count.iter();
        let (card_a, _) = iter.next().unwrap();
        return Strengh::High {
            cards: self.cards.clone(),
        };
    }
}
