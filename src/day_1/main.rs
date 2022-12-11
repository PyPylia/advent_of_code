static INPUT: &str = include_str!("input.txt");

fn main() -> anyhow::Result<()> {
    let mut num: u32 = 0;
    let mut totals: Vec<u32> = vec![];

    for line in INPUT.lines() {
        let line = line.trim();

        if line == "" {
            totals.push(num);
            num = 0;
        } else {
            let line: u32 = line.parse()?;
            num += line;
        }
    }

    totals.push(num);
    totals.sort();

    let totals_last = totals.len() - 1;

    println!(
        "Highest total calories: {}",
        totals[totals_last]
    );

    println!(
        "Total of top 3: {}",
        totals[totals_last] + totals[totals_last - 1] + totals[totals_last - 2]
    );

    Ok(())
}
