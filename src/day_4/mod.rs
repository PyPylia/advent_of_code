use std::str::FromStr;

#[derive(Clone)]
struct Scratchcard {
    winning_numbers: u128,
    owned_numbers: u128,
}

impl Scratchcard {
    fn winning_count(&self) -> u32 {
        (self.winning_numbers & self.owned_numbers).count_ones()
    }

    fn calculate_points(&self) -> u32 {
        match self.winning_count() {
            0 => 0,
            other => 2u32.pow(other - 1),
        }
    }
}

impl FromStr for Scratchcard {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let remaining = s
            .strip_prefix("Card ")
            .ok_or(eyre::eyre!("Invalid card: {}", s))?;
        let sections: Vec<&str> = remaining.split(": ").collect();
        let [_id_str, numbers_str] = sections.as_slice() else {
            return Err(eyre::eyre!("Invalid card: {}", s));
        };

        let sections: Vec<&str> = numbers_str.split(" | ").collect();
        let [winning_numbers_str, owned_numbers_str] = sections.as_slice() else {
            return Err(eyre::eyre!("Invalid card: {}", s));
        };

        let mut winning_numbers = 0;
        let mut owned_numbers = 0;

        for number in winning_numbers_str
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(u8::from_str)
        {
            winning_numbers |= 1 << number?;
        }

        for number in owned_numbers_str
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(u8::from_str)
        {
            owned_numbers |= 1 << number?;
        }

        Ok(Self {
            winning_numbers,
            owned_numbers,
        })
    }
}

pub fn first(input: &str) -> eyre::Result<String> {
    let mut sum = 0;

    for line in input.lines() {
        let card: Scratchcard = line.parse()?;
        sum += card.calculate_points();
    }

    Ok(sum.to_string())
}

pub fn second(input: &str) -> eyre::Result<String> {
    let cards: eyre::Result<Vec<Scratchcard>> = input.lines().map(Scratchcard::from_str).collect();
    let original_cards: Vec<(usize, u32)> = cards?
        .into_iter()
        .map(|card| card.winning_count())
        .enumerate()
        .collect();

    let mut resultant_cards = original_cards.clone();
    let mut i = 0u32;

    while let Some((id, winning_count)) = resultant_cards.pop() {
        for x in id + 1..id + winning_count as usize + 1 {
            resultant_cards.push(original_cards[x])
        }

        i += 1;
    }

    Ok(i.to_string())
}
