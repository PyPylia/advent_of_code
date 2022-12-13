static INPUT: &str = include_str!("input.txt");

fn main() -> anyhow::Result<()> {
    let mut crate_stacks: Vec<Vec<char>> = vec![];
    let mut stack_count = 0;

    for line in INPUT.lines().rev() {
        if line.starts_with(" 1") {
            stack_count = line.replace(" ", "").len();
            for _index in 0..stack_count {
                crate_stacks.push(vec![]);
            }
        } else if stack_count != 0 {
            let bytes = line.as_bytes();
            for index in 0..stack_count {
                let current_char = bytes[index * 4 + 1] as char;
                if current_char != ' ' {
                    crate_stacks[index].push(current_char);
                }
            }
        }
    }

    let mut crate_stacks_9001 = crate_stacks.clone();

    for line in INPUT.lines() {
        if line.starts_with("move") {
            let line = line.trim().replace("move ", "");
            let line: Vec<&str> = line.split(" from ").collect();
            let mut amount: usize = line[0].parse().unwrap();
            let line: Vec<&str> = line[1].split(" to ").collect();
            let from: usize = line[0].parse().unwrap();
            let to: usize = line[1].parse().unwrap();

            let mut buffer_9001: Vec<char> = vec![];

            while amount > 0 {
                match crate_stacks[from - 1].pop() {
                    Some(char) => crate_stacks[to - 1].push(char),
                    None => (),
                }
                match crate_stacks_9001[from - 1].pop() {
                    Some(char) => buffer_9001.push(char),
                    None => (),
                }
                amount -= 1;
            }

            buffer_9001.reverse();
            crate_stacks_9001[to - 1].append(&mut buffer_9001);
        }
    }

    let mut stack_tops = String::new();
    for stack in crate_stacks {
        stack_tops.push(*stack.last().unwrap());
    }

    let mut stack_tops_9001 = String::new();
    for stack in crate_stacks_9001 {
        stack_tops_9001.push(*stack.last().unwrap());
    }

    println!("Stack tops 9000: {}", stack_tops);
    println!("Stack tops 9001: {}", stack_tops_9001);

    Ok(())
}
