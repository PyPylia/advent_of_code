use crate::collect_to_array;
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
        let [_id_str, numbers_str] =
            collect_to_array(remaining.split(": ")).ok_or(eyre::eyre!("Invalid card: {}", s))?;
        let [winning_numbers_str, owned_numbers_str] =
            collect_to_array(numbers_str.split(" | ")).ok_or(eyre::eyre!("Invalid card: {}", s))?;

        let mut winning_numbers = 0;
        let mut owned_numbers = 0;

        for number in winning_numbers_str
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| lexical_core::parse::<u8>(s.as_bytes()))
        {
            winning_numbers |= 1 << number?;
        }

        for number in owned_numbers_str
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| lexical_core::parse::<u8>(s.as_bytes()))
        {
            owned_numbers |= 1 << number?;
        }

        Ok(Self {
            winning_numbers,
            owned_numbers,
        })
    }
}

pub fn first(input: &str) -> eyre::Result<u64> {
    let mut sum = 0;

    for line in input.lines() {
        let card: Scratchcard = line.parse()?;
        sum += card.calculate_points() as u64;
    }

    Ok(sum)
}

const MAX_CARD_COUNT: usize = 200;
pub fn second(input: &str) -> eyre::Result<u64> {
    let mut winning_counts = heapless::Vec::<usize, MAX_CARD_COUNT>::new();
    for line in input.lines() {
        let card: Scratchcard = line.parse()?;
        winning_counts.push(card.winning_count() as usize).ok();
    }

    let mut copy_array = [1u32; MAX_CARD_COUNT];
    let mut sum = 0;
    let mut i = 0;

    while i < winning_counts.len() {
        match &mut copy_array[i] {
            0 => i += 1,
            this => {
                sum += 1;
                *this -= 1;
                let winning_count = winning_counts[i];
                let next = i + 1;
                for card in &mut copy_array[next..next + winning_count] {
                    *card += 1;
                }
            }
        }
    }

    Ok(sum)
}
