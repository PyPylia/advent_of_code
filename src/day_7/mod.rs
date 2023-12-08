use crate::try_collect_to_array;
use std::{cmp::Ordering, mem};
use thiserror::Error;

#[allow(dead_code)] // Required because we transmute to Card::Two to Card::Nine
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
#[repr(u8)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn cmp_with_joker(&self, other: &Card) -> Ordering {
        match (*self == Card::J, *other == Card::J) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => self.cmp(other),
        }
    }
}

#[derive(Debug, Error)]
#[error("Invalid card: {0}")]
struct InvalidCard(char);
impl TryFrom<u8> for Card {
    type Error = InvalidCard;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'A' => Self::A,
            b'K' => Self::K,
            b'Q' => Self::Q,
            b'J' => Self::J,
            b'T' => Self::T,
            b'2'..=b'9' => unsafe { std::mem::transmute(value - b'2') },
            other => return Err(InvalidCard(other as char)),
        })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

enum State {
    Default,
    ThreeOfAKind,
    OnePair,
}

impl HandType {
    fn calculate_state(counts: [u8; 13]) -> Self {
        let mut state = State::Default;
        for count in counts {
            match count {
                5 => return Self::FiveOfAKind,
                4 => return Self::FourOfAKind,
                3 => {
                    if let State::Default = state {
                        state = State::ThreeOfAKind;
                    } else {
                        return Self::FullHouse;
                    }
                }
                2 => match state {
                    State::ThreeOfAKind => return Self::FullHouse,
                    State::Default => state = State::OnePair,
                    State::OnePair => return Self::TwoPair,
                },
                _ => {}
            }
        }

        match state {
            State::ThreeOfAKind => Self::ThreeOfAKind,
            State::Default => Self::HighCard,
            State::OnePair => Self::OnePair,
        }
    }

    fn get_counts(cards: &[Card; 5]) -> [u8; 13] {
        let mut counts = [0; 13];
        for card in cards {
            counts[*card as usize] += 1;
        }

        counts
    }

    fn from_cards<const WITH_JOKER: bool>(cards: &[Card; 5]) -> Self {
        let mut counts = Self::get_counts(cards);
        if WITH_JOKER {
            let joker_count = mem::take(&mut counts[Card::J as usize]);
            let original_type = Self::calculate_state(counts);
            if joker_count == 0 {
                return original_type;
            }

            match original_type {
                Self::FiveOfAKind => Self::FiveOfAKind,
                Self::FourOfAKind => Self::FiveOfAKind,
                Self::FullHouse => match joker_count {
                    1 => Self::FourOfAKind,
                    _greater => Self::FiveOfAKind,
                },
                Self::ThreeOfAKind => match joker_count {
                    1 => Self::FourOfAKind,
                    _greater => Self::FiveOfAKind,
                },
                Self::TwoPair => match joker_count {
                    1 => Self::FullHouse,
                    2 => Self::FourOfAKind,
                    _greater => Self::FiveOfAKind,
                },
                Self::OnePair => match joker_count {
                    1 => Self::ThreeOfAKind,
                    2 => Self::FourOfAKind,
                    _greater => Self::FiveOfAKind,
                },
                Self::HighCard => match joker_count {
                    1 => Self::OnePair,
                    2 => Self::ThreeOfAKind,
                    3 => Self::FourOfAKind,
                    _greater => Self::FiveOfAKind,
                },
            }
        } else {
            Self::calculate_state(counts)
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
    bid: u16,
}

impl Hand {
    fn cmp<const WITH_JOKER: bool>(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                for (my_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    match if WITH_JOKER {
                        my_card.cmp_with_joker(other_card)
                    } else {
                        my_card.cmp(other_card)
                    } {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }

                Ordering::Equal
            }
            other => other,
        }
    }

    fn from_str<const WITH_JOKER: bool>(s: &str) -> eyre::Result<Self> {
        let (cards_bytes, bid_bytes) = s.as_bytes().split_array_ref::<5>();
        let bid = lexical_core::parse(&bid_bytes[1..])?;
        let cards =
            try_collect_to_array(cards_bytes.into_iter().map(|byte| Card::try_from(*byte)))?;

        let hand_type = HandType::from_cards::<WITH_JOKER>(&cards);
        Ok(Self {
            cards,
            hand_type,
            bid,
        })
    }
}

fn get_total_winnings<const WITH_JOKER: bool>(input: &str) -> eyre::Result<u64> {
    let hands: eyre::Result<Vec<Hand>> = input
        .lines()
        .map(|line| Hand::from_str::<WITH_JOKER>(line))
        .collect();

    let mut hands = hands?;
    hands.sort_unstable_by(Hand::cmp::<WITH_JOKER>);

    Ok(hands
        .iter()
        .enumerate()
        .map(|(index, hand)| hand.bid as u64 * (index as u64 + 1))
        .sum())
}

pub fn first(input: &str) -> eyre::Result<u64> {
    get_total_winnings::<false>(input)
}

pub fn second(input: &str) -> eyre::Result<u64> {
    get_total_winnings::<true>(input)
}
