use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Err: Could not read file.");
    let mut numbers: Vec<Vec<u32>> = vec![Vec::new(); input.lines().next().unwrap().len()];
    let mut gamma_rate = String::new();

    for line in input.lines() {
        for (i, char) in line.chars().enumerate() {
            numbers[i].push(char.to_digit(2).unwrap());
        }
    }

    for a in numbers {
        gamma_rate.push_str(
            if a.clone().iter().sum::<u32>() as f64 / a.clone().len() as f64 > 0.5 {
                "1"
            } else {
                "0"
            },
        );
    }

    let gamma_rate = u32::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = gamma_rate ^ 0b111111111111;

    println!("{epsilon_rate} {gamma_rate}");

    println!("{}", gamma_rate * epsilon_rate);
}
