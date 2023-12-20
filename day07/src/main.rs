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
        // println!("{} {:?}", idx, hand.to_strength());
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
            "t" => 4,
            "9" => 5,
            "8" => 6,
            "7" => 7,
            "6" => 8,
            "5" => 9,
            "4" => 10,
            "3" => 11,
            "2" => 12,
            "j" => 13, // JOKER
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
}

impl Hand {
    fn to_strength(&self) -> Strengh {
        use std::collections::HashMap;

        let mut count: HashMap<Card, u32> = self.cards.iter().fold(HashMap::new(), |mut acc, c| {
            acc.entry(*c)
                .and_modify(|q: &mut u32| *q += 1)
                .or_insert_with(|| 1);
            acc
        });

        if count.len() > 1 && count.contains_key(&Card(13)) {
            let joke = count.remove(&Card(13)).unwrap();

            let keys = count
                .iter()
                .try_fold((Card(14), 0) , |(acc_c, acc_n), (c, n)| {
                    
                    if *n > acc_n {
                        return Some((*c, *n));
                    } else if *n < acc_n {
                        return Some((acc_c, acc_n));
                    } else if acc_c.0 < c.0 {
                        return Some((*c, *n));
                    } else {
                        return Some((acc_c, acc_n));
                    }
                });

            if let Some((card, _)) = keys {
                count.entry(card).and_modify(|c| *c += joke );
            } else {
                count.entry(Card(13)).or_insert(joke);
            }
        }

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
            return Strengh::FiveKind {
                cards: self.cards.clone(),
            };
        } else if count.len() == 2 {
            let mut iter = count.iter();
            let (_, count_a) = iter.next().unwrap();
            let (_, count_b) = iter.next().unwrap();

            match (count_a, count_b) {
                (3, 2) => {
                    return Strengh::FullHouse {
                        cards: self.cards.clone(),
                    }
                }
                (2, 3) => {
                    return Strengh::FullHouse {
                        cards: self.cards.clone(),
                    }
                }
                (4, 1) => {
                    return Strengh::FourKind {
                        cards: self.cards.clone(),
                    }
                }
                (1, 4) => {
                    return Strengh::FourKind {
                        cards: self.cards.clone(),
                    }
                }
                _ => (),
            };
        } else if count.len() == 3 {
            let mut iter = count.iter();
            let (_, count_a) = iter.next().unwrap();
            let (_, count_b) = iter.next().unwrap();

            match (count_a, count_b) {
                (3, _) => {
                    return Strengh::ThreeKind {
                        cards: self.cards.clone(),
                    }
                }
                (2, 2) => {
                    return Strengh::TwoPair {
                        cards: self.cards.clone(),
                    }
                }
                _ => (),
            };
        } else if count.len() == 4 {
            let mut iter = count.iter();
            let (_, count) = iter.next().unwrap();

            match count {
                2 => {
                    return Strengh::Pair {
                        cards: self.cards.clone(),
                    }
                }
                _ => (),
            };
        }

        return Strengh::High {
            cards: self.cards.clone(),
        };
    }
}
