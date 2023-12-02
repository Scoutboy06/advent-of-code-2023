use std::fs;

fn main() {
    let input = fs::read_to_string("src/in/01.in").expect("No indata file (01.in)");
    let mut sum: u32 = 0;

    for line in input.split('\n') {
        let chars: Vec<char> = line.chars().collect();
        let mut n = String::with_capacity(2);

        let mut i: usize = 0;
        while i < line.len() {
            if chars[i].is_digit(10) {
                break;
            }
            i += 1;
        }

        let mut j: usize = line.len() - 1;
        loop {
            if chars[j].is_digit(10) || j == 0 {
                break;
            }
            j -= 1;
        }

        n.push(chars[i]);
        n.push(chars[j]);

        sum += n.parse::<u32>().unwrap();
    }

    println!("{}", sum);
}
