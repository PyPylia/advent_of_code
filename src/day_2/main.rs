static INPUT: &str = include_str!("input.txt");

enum Actions {
    Rock,
    Paper,
    Scissors,
}

impl Actions {
    fn from_char(char: &str) -> Actions {
        match char {
            "A" => Actions::Rock,
            "B" => Actions::Paper,
            "C" => Actions::Scissors,

            "X" => Actions::Rock,
            "Y" => Actions::Paper,
            "Z" => Actions::Scissors,

            _ => Actions::Rock,
        }
    }

    fn get_score(&self, opponent: &Actions) -> u32 {
        match self {
            Actions::Rock => match opponent {
                Actions::Rock => 3,
                Actions::Paper => 0,
                Actions::Scissors => 6,
            },
            Actions::Paper => match opponent {
                Actions::Rock => 6,
                Actions::Paper => 3,
                Actions::Scissors => 0,
            },
            Actions::Scissors => match opponent {
                Actions::Rock => 0,
                Actions::Paper => 6,
                Actions::Scissors => 3,
            },
        }
    }

    fn get_value(&self) -> u32 {
        match self {
            Actions::Rock => 1,
            Actions::Paper => 2,
            Actions::Scissors => 3,
        }
    }

    fn get_action(&self, opponent: &Actions) -> Actions {
        match self {
            Actions::Rock => match opponent {
                // LOSE
                Actions::Rock => Actions::Scissors,
                Actions::Paper => Actions::Rock,
                Actions::Scissors => Actions::Paper,
            },
            Actions::Paper => match opponent {
                // DRAW
                Actions::Rock => Actions::Rock,
                Actions::Paper => Actions::Paper,
                Actions::Scissors => Actions::Scissors,
            },
            Actions::Scissors => match opponent {
                // WIN
                Actions::Rock => Actions::Paper,
                Actions::Paper => Actions::Scissors,
                Actions::Scissors => Actions::Rock,
            },
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut action_total_score: u32 = 0;
    let mut outcome_total_score: u32 = 0;

    for line in INPUT.lines() {
        let chars: Vec<&str> = line.trim().split(" ").collect();

        let opponent = Actions::from_char(chars[0]);
        let you = Actions::from_char(chars[1]);

        action_total_score += you.get_value() + you.get_score(&opponent);

        let you = you.get_action(&opponent);

        outcome_total_score += you.get_value() + you.get_score(&opponent);
    }

    println!(
        "Action total score: {}",
        action_total_score
    );

    println!(
        "Outcome total score: {}",
        outcome_total_score
    );

    Ok(())
}
