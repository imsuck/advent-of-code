use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Err: Could not read input.txt");
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input.lines() {
        let line = line.split_whitespace().collect::<Vec<&str>>();
        let (direction, amount) = if let [direction, amount] = line.as_slice() {
            (direction, amount)
        } else {
            unreachable!();
        };
        let amount = amount
            .parse::<i32>()
            .expect("Expected number found something else.");

        match *direction {
            "forward" => {
                position += amount;
                depth += amount * aim;
            }
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => (),
        }
    }

    println!("{position} * {depth} = {}", position * depth);
}
