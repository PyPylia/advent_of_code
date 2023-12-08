use crate::{collect_to_array, try_collect_to_array};
use std::{cmp::Ordering, hint::unreachable_unchecked};
use thiserror::Error;

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
impl TryFrom<char> for Card {
    type Error = InvalidCard;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            other => return Err(InvalidCard(other)),
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
                1 | 0 => (),
                // SAFETY: There are a max of 5 cards so nothing can be outside of the range 0-5
                _ => unsafe { unreachable_unchecked() },
            }
        }

        match state {
            State::ThreeOfAKind => Self::ThreeOfAKind,
            State::Default => Self::HighCard,
            State::OnePair => Self::OnePair,
        }
    }

    fn from_cards_without_joker(cards: &[Card; 5]) -> Self {
        let mut counts: [u8; 13] = [0; 13];
        for card in cards {
            counts[*card as usize] += 1;
        }

        Self::calculate_state(counts)
    }

    fn from_cards_with_joker(cards: &[Card; 5]) -> Self {
        let mut joker_count: u8 = 0;
        let mut counts: [u8; 13] = [0; 13];
        for card in cards {
            if *card == Card::J {
                joker_count += 1;
            } else {
                counts[*card as usize] += 1;
            }
        }

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
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
    bid: u16,
}

impl Hand {
    fn sort(&self, other: &Self, card_sorter: fn(&Card, &Card) -> Ordering) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                for (my_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    match card_sorter(my_card, other_card) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }

                Ordering::Equal
            }
            other => other,
        }
    }

    fn sort_without_joker(&self, other: &Self) -> Ordering {
        self.sort(other, Card::cmp)
    }

    fn sort_with_joker(&self, other: &Self) -> Ordering {
        self.sort(other, Card::cmp_with_joker)
    }

    fn from_str(s: &str, with_joker: bool) -> eyre::Result<Self> {
        let [cards_str, bid_str] =
            collect_to_array(s.split(" ")).ok_or_else(|| eyre::eyre!("Invalid hand: {}", s))?;
        let cards = try_collect_to_array(cards_str.chars().map(Card::try_from))?;
        let hand_type = if with_joker {
            HandType::from_cards_with_joker(&cards)
        } else {
            HandType::from_cards_without_joker(&cards)
        };
        let bid = lexical_core::parse(bid_str.as_bytes())?;

        Ok(Self {
            cards,
            hand_type,
            bid,
        })
    }
}

fn get_total_winnings(input: &str, with_joker: bool) -> eyre::Result<u64> {
    let hands: eyre::Result<Vec<Hand>> = input
        .lines()
        .map(|line| Hand::from_str(line, with_joker))
        .collect();

    let mut hands = hands?;
    hands.sort_unstable_by(if with_joker {
        Hand::sort_with_joker
    } else {
        Hand::sort_without_joker
    });

    Ok(hands
        .iter()
        .enumerate()
        .map(|(index, hand)| hand.bid as u64 * (index as u64 + 1))
        .sum())
}

pub fn first(input: &str) -> eyre::Result<u64> {
    get_total_winnings(input, false)
}

pub fn second(input: &str) -> eyre::Result<u64> {
    get_total_winnings(input, true)
}
