#![feature(array_try_from_fn)]

use std::{array, env, fs, time::Instant};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

const CHALLENGES: &[(
    u8,
    fn(&str) -> eyre::Result<u64>,
    Option<fn(&str) -> eyre::Result<u64>>,
)] = &[
    (1, day_1::first, Some(day_1::second)),
    (2, day_2::first, Some(day_2::second)),
    (3, day_3::first, Some(day_3::second)),
    (4, day_4::first, Some(day_4::second)),
    (5, day_5::first, Some(day_5::second)),
    (6, day_6::first, Some(day_6::second)),
];

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let challenge: u8 = env::args()
        .nth(1)
        .ok_or(eyre::eyre!(
            "The challenge number to run must be passed as a cmdline argument."
        ))?
        .parse()?;

    for (number, first, second) in CHALLENGES {
        if challenge == *number {
            let input = fs::read_to_string(format!("src/day_{}/input.txt", challenge))?;

            let first_start = Instant::now();
            let first_answer = first(&input)?;
            let first_end = Instant::now() - first_start;

            println!(
                "\nThe first answer ({:.5}s) for day {} is: {}",
                first_end.as_secs_f64(),
                challenge,
                first_answer
            );

            if let Some(second) = second {
                let second_start = Instant::now();
                let second_answer = second(&input)?;
                let second_end = Instant::now() - second_start;

                println!(
                    "The second answer ({:.5}s) for day {} is: {}",
                    second_end.as_secs_f64(),
                    challenge,
                    second_answer
                );
            }

            return Ok(());
        }
    }

    Err(eyre::eyre!("Invalid challenge number given."))
}

fn collect_to_array<T, const N: usize>(mut iter: impl Iterator<Item = T>) -> Option<[T; N]> {
    array::try_from_fn(|_| iter.next())
}
