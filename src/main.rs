use std::{env, fs};

mod day_1;
mod day_2;

const CHALLENGES: &[(
    u8,
    fn(&str) -> eyre::Result<String>,
    Option<fn(&str) -> eyre::Result<String>>,
)] = &[
    (1, day_1::first, Some(day_1::second)),
    (2, day_2::first, Some(day_2::second)),
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
            let first_answer = first(&input)?;
            println!(
                "\nThe first answer for day {} is: {}",
                challenge, first_answer
            );

            if let Some(second) = second {
                let second_answer = second(&input)?;
                println!(
                    "The second answer for day {} is: {}",
                    challenge, second_answer
                );
            }

            return Ok(());
        }
    }

    Err(eyre::eyre!("Invalid challenge number given."))
}
