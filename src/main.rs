#![feature(
    array_try_from_fn,
    iter_array_chunks,
    split_array,
    maybe_uninit_uninit_array
)]

use std::{array, env, error::Error, fs, time::Instant};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

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
    (7, day_7::first, Some(day_7::second)),
    (8, day_8::first, Some(day_8::second)),
];

fn time_challenge(
    day: u8,
    part: &'static str,
    challenge: &fn(&str) -> eyre::Result<u64>,
    input: &str,
) -> eyre::Result<()> {
    let initial_start = Instant::now();
    let initial_answer = challenge(input);
    let initial_end = Instant::now() - initial_start;
    let initial_answer = initial_answer?;

    println!(
        "\nThe {} answer ({:?}) for day {} is: {}",
        part, initial_end, day, initial_answer
    );

    #[cfg(not(debug_assertions))]
    {
        use std::time::Duration;

        let average_times = 10u128.pow(9) / initial_end.as_nanos() * 2;
        let average_start = Instant::now();
        for _ in 0..average_times {
            if challenge(input)? != initial_answer {
                return Err(eyre::eyre!(
                    "Got mismatching answers on day {}, {} part",
                    day,
                    part
                ));
            }
        }
        let average_end = Instant::now() - average_start;

        println!(
            "    Averaged time over {:?}: {:?}",
            average_end,
            Duration::from_nanos((average_end.as_nanos() / average_times) as u64)
        );
    }

    Ok(())
}

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
            time_challenge(challenge, "first", first, &input)?;
            if let Some(second) = second {
                time_challenge(challenge, "second", second, &input)?;
            }

            return Ok(());
        }
    }

    Err(eyre::eyre!("Invalid challenge number given."))
}

fn collect_to_array<T, const N: usize>(mut iter: impl Iterator<Item = T>) -> Option<[T; N]> {
    array::try_from_fn(|_| iter.next())
}

fn try_collect_to_array<T, E: Error + Send + Sync + 'static, const N: usize>(
    mut iter: impl Iterator<Item = Result<T, E>>,
) -> eyre::Result<[T; N]> {
    array::try_from_fn(|_| {
        iter.next()
            .ok_or_else(|| eyre::eyre!("Not enough elements to build array"))
            .and_then(|inner| inner.map_err(Into::into))
    })
}
