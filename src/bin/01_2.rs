use std::fs;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = fs::read_to_string("src/in/01.in").expect("No indata file (01.in)");
    let mut sum: u32 = 0;

    for line in input.split('\n') {
        let left = find_left(line);
        let right = find_right(line);

        sum += left * 10;
        sum += right;
    }

    println!("Answer: {}", sum);
}

fn find_left(line: &str) -> u32 {
    let line_chars: Vec<char> = line.chars().collect();
    let mut queue: Vec<(usize, usize)> = Vec::new();
    let mut temp_queue: Vec<(usize, usize)> = Vec::new();
    let mut i: usize = 0;

    while i < line.len() {
        // Is it a digit 1-9?
        if line_chars[i].is_digit(10) {
            return line_chars[i].to_digit(10).unwrap() as u32;
        }

        // Is it spelled out in letters?
        for (digit_index, streak_len) in queue.iter() {
            let digit_chars: Vec<char> = DIGITS[*digit_index].chars().collect();

            // Is the current line char the correct one? Then push it to the queue.
            if digit_chars[*streak_len] == line_chars[i] {
                if *streak_len == digit_chars.len() - 1 {
                    return *digit_index as u32 + 1;
                }

                temp_queue.push((digit_index.clone(), streak_len + 1));
            }
        }

        // Add digits to queue
        for (digit_index, digit) in DIGITS.iter().enumerate() {
            let digit_chars: Vec<char> = digit.chars().collect();
            if digit_chars[0] == line_chars[i] {
                temp_queue.push((digit_index, 1));
            }
        }

        queue.append(&mut temp_queue);
        i += 1;
    }

    0
}

fn find_right(line: &str) -> u32 {
    let line_chars: Vec<char> = line.chars().collect();
    let mut queue: Vec<(usize, usize)> = Vec::new();
    let mut temp_queue: Vec<(usize, usize)> = Vec::new();
    let mut i: usize = line.len() - 1;

    loop {
        // Is it a digit 1-9?
        if line_chars[i].is_digit(10) {
            return line_chars[i].to_digit(10).unwrap() as u32;
        }

        // Is it spelled out in letters?
        for (digit_index, streak_len) in queue.iter() {
            let digit_chars: Vec<char> = DIGITS[*digit_index].chars().rev().collect();

            // Is the current line char the correct one? Then push it to the queue.
            if digit_chars[*streak_len] == line_chars[i] {
                if *streak_len == digit_chars.len() - 1 {
                    return *digit_index as u32 + 1;
                }

                temp_queue.push((digit_index.clone(), streak_len + 1));
            }
        }

        // Add digits to queue
        for (digit_index, digit) in DIGITS.iter().enumerate() {
            let digit_chars: Vec<char> = digit.chars().rev().collect();
            if digit_chars[0] == line_chars[i] {
                temp_queue.push((digit_index, 1));
            }
        }

        queue.append(&mut temp_queue);

        if i == 0 {
            break;
        }

        i -= 1;
    }

    0
}
