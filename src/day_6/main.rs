static INPUT: &str = include_str!("input.txt");

fn is_completely_different(char_vec: &Vec<char>) -> bool {
    for index_1 in 0..char_vec.len() {
        for index_2 in index_1 + 1..char_vec.len() {
            if char_vec[index_1] == char_vec[index_2] {
                return false;
            }
        }
    }

    true
}

fn main() -> anyhow::Result<()> {
    let bytes = INPUT.as_bytes();
    let mut char_vec_4: Vec<char> = vec![];
    let mut char_vec_14: Vec<char> = vec![];

    for index in 0..4 {
        let current_char = bytes[index] as char;
        char_vec_4.push(current_char);
        char_vec_14.push(current_char);
    }

    for index in 4..14 {
        char_vec_14.push(bytes[index] as char);
    }

    let mut found_start_of_packet = false;

    for index in 4..INPUT.len() {
        let current_char = bytes[index] as char;

        if !found_start_of_packet && is_completely_different(&char_vec_4) {
            println!("Start-of-packet index: {}", index);
            found_start_of_packet = true;
        }

        if index >= 14 {
            if is_completely_different(&char_vec_14) {
                println!("Start-of-message index: {}", index);
                break;
            }

            char_vec_14.remove(0);
            char_vec_14.push(current_char);
        }

        char_vec_4.remove(0);
        char_vec_4.push(current_char);
    }

    Ok(())
}
