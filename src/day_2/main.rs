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

    fn get_score(self, opponent: &Actions) -> u32 {
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
                Actions::Rock => 6,
                Actions::Paper => 3,
                Actions::Scissors => 0,
            },
        }
    }

    fn get_value(self: &Actions) -> u32 {
        match self {
            Actions::Rock => 1,
            Actions::Paper => 2,
            Actions::Scissors => 3,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut total_score: u32 = 0;

    for line in INPUT.lines() {
        let chars: Vec<&str> = line.trim().split(" ").collect();

        let opponent = Actions::from_char(chars[0]);
        let you = Actions::from_char(chars[1]);

        total_score += you.get_value() + you.get_score(&opponent);
    }

    println!("Total score: {}", total_score);

    Ok(())
}
