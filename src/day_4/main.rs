static INPUT: &str = include_str!("input.txt");

fn main() -> anyhow::Result<()> {
    let mut containing_count: u32 = 0;
    let mut overlapping_count: u32 = 0;

    for line in INPUT.lines() {
        let line: Vec<&str> = line.trim().split(",").collect();
        let first: Vec<&str> = line[0].split("-").collect();
        let second: Vec<&str> = line[1].split("-").collect();

        let first_min: u8 = first[0].parse()?;
        let first_max: u8 = first[1].parse()?;
        let first_range = first_max - first_min;

        let second_min: u8 = second[0].parse()?;
        let second_max: u8 = second[1].parse()?;
        let second_range = second_max - second_min;

        if (first_range < second_range && first_min >= second_min && first_max <= second_max)
            || (second_min >= first_min && second_max <= first_max)
        {
            containing_count += 1;
        }

        if first_min >= second_min && first_min <= second_max
            || first_max <= second_max && first_max >= second_min
            || second_min >= first_min && second_min <= first_max
            || second_max <= first_max && second_max >= first_min
        {
            overlapping_count += 1;
        }
    }

    println!("Containing count: {}", containing_count);

    println!(
        "Overlapping count: {}",
        overlapping_count
    );

    Ok(())
}
