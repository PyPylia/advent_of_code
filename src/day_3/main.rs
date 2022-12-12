static INPUT: &str = include_str!("input.txt");

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() -> anyhow::Result<()> {
    let mut line_index = 0;
    let mut common_chars: Vec<char> = vec![];
    let mut error_sum = 0;
    let mut group_sum = 0;

    for line in INPUT.lines() {
        let half_len = line.len() / 2;
        let mut line_iter = line.chars();
        let mut first_half: Vec<char> = vec![];

        for _index in 0..half_len {
            first_half.push(line_iter.next().unwrap());
        }

        for _index in 0..half_len {
            let current_char = line_iter.next().unwrap();
            if first_half.contains(&current_char) {
                error_sum += ALPHABET.find(current_char).unwrap() + 1;
                break;
            }
        }

        match line_index {
            0 => {
                common_chars = line.chars().collect::<Vec<char>>();
            }
            1 => {
                let old_chars = common_chars.clone();
                common_chars = vec![];
                for current_char in line.chars() {
                    if old_chars.contains(&current_char) {
                        common_chars.push(current_char);
                    }
                }
            }
            2 => {
                for current_char in line.chars() {
                    if common_chars.contains(&current_char) {
                        group_sum += ALPHABET.find(current_char).unwrap() + 1;
                        break;
                    }
                }
            }
            _ => (),
        }

        line_index += 1;
        line_index %= 3;
    }

    println!("Sum of errors: {}", error_sum);
    println!("Sum of common groups: {}", group_sum);

    Ok(())
}
