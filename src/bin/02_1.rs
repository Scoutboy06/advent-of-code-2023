use std::fs;

fn main() {
    let input = fs::read_to_string("src/in/02.in").expect("No input file found (02.in)");
    let mut sum: u32 = 0;

    // Example: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    'outer: for (i, line) in input.split('\n').enumerate() {
        // Example: "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        let rounds = line.split_once(": ").unwrap().1.split("; ");

        // Example: "3 blue, 4 red"
        for round in rounds {
            let mut red: u32 = 0;
            let mut green: u32 = 0;
            let mut blue: u32 = 0;

            // Example: "3 blue"
            for cubes in round.split(", ") {
                let split: Vec<&str> = cubes.split(' ').collect();
                let count: u32 = split[0].parse().unwrap();
                let color = split[1];

                match color {
                    "red" => red += count,
                    "green" => green += count,
                    "blue" => blue += count,
                    _ => unreachable!(),
                }
            }

            if red > 12 || green > 13 || blue > 14 {
                continue 'outer;
            }
        }

        sum += i as u32 + 1;
    }

    println!("Answer: {}", sum);
}
