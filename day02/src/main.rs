use std::str::FromStr;
use std::io;

fn main() {
    let lines = io::stdin().lines();

    let mut count = 0;
    let mut power = 0;

    for line in lines {
        if let Some(line) = line.ok() {
            let game = match GameSet::from_str(&line) {
                Ok(game) => game,
                Err(err) => panic!("{:?}", err)
            };


            if game
                .games
                .iter()
                .fold(true, |acc , g| acc && g.is_valid(12, 14, 13)) {

                count += game.id;
            }

            power += match game.power() {
                Some(v) => v,
                None => 0,
            };
        }
    }

    println!("Possible: {}", count);
    println!("Power: {}", power);
}


#[derive(Debug, PartialEq, Eq)]
struct ParseGameError {
    reason: &'static str,
}

#[derive(Debug, PartialEq, Default)]
struct Game {
    red: Option<u32>,
    blue: Option<u32>,
    green: Option<u32>,
}

#[derive(Debug, PartialEq)]
struct GameSet {
    id: u32,
    games: Vec<Game>,
}

impl GameSet {
    fn power(&self) -> Option<u32> {

        let red = self.games.iter()
            .filter_map(|g| g.red)
            .reduce(|acc, m| acc.max(m) );

        let blue = self.games.iter()
            .filter_map(|g| g.blue)
            .reduce(|acc, m| acc.max(m) );

        let green = self.games.iter()
            .filter_map(|g| g.green)
            .reduce(|acc, m| acc.max(m) );

        match (red, blue, green) {
            (Some(r), Some(b), Some(g)) => Some(r * g * b),
            _ => None
        }
    }

}

impl Game {
    fn is_valid(&self, max_red: u32, max_blue: u32, max_green: u32) -> bool {
        match self.red {
            Some(r) if r > max_red => return false,
            _ => (),
        };

        match self.blue {
            Some(b) if b > max_blue => return false,
            _ => (),
        };

        match self.green {
            Some(g) if g > max_green => return false,
            _ => (),
        };
        return true
    }

}

impl FromStr for GameSet {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let (id, rest) = match s.split_once(":") {
            Some(b) => b,
            None => { return Err(ParseGameError{
                reason: "Split failed",
            }) },
        };

        let num: u32 = u32::from_str(id.strip_prefix("Game ").unwrap()).unwrap();

        let mut games = vec![];

        for game in rest.split(";") {

            let game = match Game::from_str(game) {
                Ok(g) => g,
                Err(e) => return Err(e),
            };

            games.push(game);
        }

        return Ok(GameSet{
            id: num,
            games: games,
        })
    }
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = Game::default();

        for section in s.split(",") {

            let (num, name) = match section.trim().split_once(" ") {
                Some(b) => b,
                None => { return Err(ParseGameError { reason: "bad game" }) },
            };

            let num = u32::from_str(num.trim()).unwrap();

            match name {
                "red" => {
                    game.red = Some(num);
                },
                "blue" => {
                    game.blue = Some(num);
                },
                "green" => {
                    game.green = Some(num);
                },
                _ => {
                    return Err(ParseGameError { reason: "bad name" })
                }
            }

        }

        Ok(game)
    }
}
