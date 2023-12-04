use std::{num::NonZeroU8, str::FromStr};

struct Set {
    red: Option<NonZeroU8>,
    green: Option<NonZeroU8>,
    blue: Option<NonZeroU8>,
}

macro_rules! check_colours {
    ($self:ident; $($ident:ident $limit:literal),+) => {
        $(
            if let Some($ident) = $self.$ident {
                if $ident.get() > $limit {
                    return false;
                }
            }
        )+
    };
}

impl Set {
    fn is_valid(&self) -> bool {
        check_colours!(
            self;
            red 12,
            green 13,
            blue 14
        );

        true
    }
}

impl FromStr for Set {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = None;
        let mut green = None;
        let mut blue = None;

        for subset in s.split(", ") {
            let sections: Vec<&str> = subset.split(" ").collect();
            let [number_str, colour] = sections.as_slice() else {
                return Err(eyre::eyre!("Invalid subset: {}", subset));
            };
            let number = number_str.parse()?;

            match *colour {
                "red" => red = Some(number),
                "green" => green = Some(number),
                "blue" => blue = Some(number),
                other => return Err(eyre::eyre!("Unexpected colour: {}", other)),
            }
        }

        Ok(Set { red, green, blue })
    }
}

struct Game {
    id: u8,
    sets: Vec<Set>,
}

macro_rules! minimum_colour {
    ($set:ident; $($ident:ident)+) => {
        $(
            if let Some(value) = $set.$ident {
                let value = value.get() as u32;
                if value > $ident {
                    $ident = value;
                }
            }
        )+
    };
}

impl Game {
    fn is_valid(&self) -> bool {
        self.sets.iter().all(Set::is_valid)
    }

    fn minimum_power(&self) -> u32 {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for set in &self.sets {
            minimum_colour!(set; red green blue);
        }

        red * green * blue
    }
}

impl FromStr for Game {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let remaining = s
            .strip_prefix("Game ")
            .ok_or(eyre::eyre!("Invalid game: {}", s))?;
        let sections: Vec<&str> = remaining.split(": ").collect();
        let [id_str, sets_str] = sections.as_slice() else {
            return Err(eyre::eyre!("Invalid game: {}", s));
        };

        let id = id_str.parse()?;
        let sets = sets_str
            .split("; ")
            .map(Set::from_str)
            .collect::<eyre::Result<Vec<Set>>>()?;

        Ok(Game { id, sets })
    }
}

pub fn first(input: &str) -> eyre::Result<String> {
    let mut counter = 0;

    for line in input.lines() {
        let game: Game = line.parse()?;
        if game.is_valid() {
            counter += game.id as u32;
        }
    }

    Ok(counter.to_string())
}

pub fn second(input: &str) -> eyre::Result<String> {
    let mut counter = 0;

    for line in input.lines() {
        let game: Game = line.parse()?;
        counter += game.minimum_power();
    }

    Ok(counter.to_string())
}