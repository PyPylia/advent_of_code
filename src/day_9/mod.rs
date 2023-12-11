const HISTORY_COUNT: usize = 21;

fn parse_input(input: &str) -> eyre::Result<Vec<heapless::Vec<i32, HISTORY_COUNT>>> {
    let mut lines = vec![];

    for line in input.lines() {
        let mut history_values = heapless::Vec::new();
        let line_bytes = line.as_bytes();

        let mut offset = 0usize;
        while offset < line_bytes.len() {
            let (value, read_count) = lexical_core::parse_partial(&line_bytes[offset..])?;
            history_values.push(value).ok();
            offset += read_count + 1;
        }

        lines.push(history_values);
    }

    Ok(lines)
}

fn get_difference<const COUNT: usize>(sequence: &[i32]) -> (heapless::Vec<i32, { COUNT - 1 }>, bool)
where
    [i32; COUNT - 1]:,
{
    let mut is_zero = true;
    (
        sequence
            .array_windows()
            .map(|[a, b]| b - a)
            .inspect(|val| {
                if *val != 0 {
                    is_zero = false
                }
            })
            .collect(),
        is_zero,
    )
}

fn extrapolate<const COUNT: usize>(sequence: &[i32]) -> i32
where
    [i32; COUNT - 1]:,
{
    let (difference, is_zero) = get_difference(sequence);
    let last = sequence[sequence.len() - 1];
    if is_zero {
        last
    } else {
        extrapolate(&difference) + last
    }
}

pub fn first(input: &str) -> eyre::Result<u64> {
    let lines = parse_input(input)?;

    Ok(lines
        .into_iter()
        .map(|sequence| extrapolate::<HISTORY_COUNT>(&sequence))
        .sum::<i32>() as u64)
}

fn extrapolate_backwards<const COUNT: usize>(sequence: &[i32]) -> i32
where
    [i32; COUNT - 1]:,
{
    let (difference, is_zero) = get_difference(sequence);
    let first = sequence[0];
    if is_zero {
        first
    } else {
        first - extrapolate_backwards(&difference)
    }
}

pub fn second(input: &str) -> eyre::Result<u64> {
    let lines = parse_input(input)?;

    Ok(lines
        .into_iter()
        .map(|sequence| extrapolate_backwards::<HISTORY_COUNT>(&sequence))
        .sum::<i32>() as u64)
}
