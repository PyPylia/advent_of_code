pub fn first(input: &str) -> eyre::Result<u64> {
    Ok(input
        .lines()
        .map(|line| {
            let mut numbers =
                line.chars()
                    .filter_map(|c| if c.is_numeric() { c.to_digit(10) } else { None });
            let first = numbers.next().unwrap_or(0);
            let last = numbers.last().unwrap_or(first);

            first * 10 + last
        })
        .sum::<u32>() as u64)
}

const NUMBER_MAP: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn get_first_number(
    input: impl Iterator<Item = char>,
    number_map: impl Iterator<Item = (String, u32)> + Clone,
) -> Option<u32> {
    let mut buf = String::new();
    for ch in input {
        if ch.is_numeric() {
            if let Some(digit) = ch.to_digit(10) {
                return Some(digit);
            } else {
                continue;
            }
        }

        buf.push(ch);
        for (pattern, number) in number_map.clone() {
            if buf.ends_with(&pattern) {
                return Some(number);
            }
        }
    }

    None
}

pub fn second(input: &str) -> eyre::Result<u64> {
    let number_map = NUMBER_MAP
        .iter()
        .map(|(pattern, number)| (pattern.to_string(), *number));

    let reversed_number_map = NUMBER_MAP
        .iter()
        .map(|(pattern, number)| (pattern.chars().rev().collect(), *number));

    Ok(input
        .lines()
        .map(|line| {
            let first = get_first_number(line.chars(), number_map.clone()).unwrap_or(0);
            let last =
                get_first_number(line.chars().rev(), reversed_number_map.clone()).unwrap_or(first);

            first * 10 + last
        })
        .sum::<u32>() as u64)
}
