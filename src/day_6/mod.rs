use std::num::ParseIntError;

fn fast_half_float(f: f32) -> f32 {
    let bits = f.to_bits();
    let exponent = (((bits >> 23) & 0xf7) as u8).wrapping_sub(1);
    let masked_bits = !(0xf7 << 23) & bits;
    f32::from_bits(masked_bits | (exponent as u32) << 23)
}

fn get_solution(time: u64, distance: u64) -> u64 {
    let a = ((time.pow(2) - 4 * distance) as f32).sqrt();
    let min = fast_half_float(time as f32 - a).floor() as u64 + 1;
    let max = fast_half_float(time as f32 + a).ceil() as u64 - 1;
    max - min + 1
}

fn get_line_strs(input: &str) -> eyre::Result<(&str, &str)> {
    let mut lines = input.lines();
    Ok((
        lines
            .next()
            .ok_or(eyre::eyre!("No times given"))?
            .strip_prefix("Time:")
            .ok_or(eyre::eyre!("Invalid times"))?,
        lines
            .next()
            .ok_or(eyre::eyre!("No distances given"))?
            .strip_prefix("Distance:")
            .ok_or(eyre::eyre!("Invalid distances"))?,
    ))
}

fn str_to_u32_vec(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(str::parse)
        .collect()
}

pub fn first(input: &str) -> eyre::Result<u64> {
    let (times_str, distances_str) = get_line_strs(input)?;
    let times = str_to_u32_vec(times_str)?;
    let distances = str_to_u32_vec(distances_str)?;

    let mut solution = 1;
    for (time, distance) in times.into_iter().zip(distances) {
        solution *= get_solution(time as u64, distance as u64);
    }

    Ok(solution)
}

pub fn second(input: &str) -> eyre::Result<u64> {
    let (time_str, distance_str) = get_line_strs(input)?;
    let time: u64 = time_str.replace(" ", "").parse()?;
    let distance: u64 = distance_str.replace(" ", "").parse()?;

    Ok(get_solution(time, distance))
}
