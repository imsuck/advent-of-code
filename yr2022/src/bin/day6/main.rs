fn main() {
    let input = std::fs::read_to_string(yr2022::get_input()).expect("Can't read input D:");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    for (i, group) in input.chars().collect::<Vec<char>>().windows(4).enumerate() {
        if !(1..group.len()).any(|i| group[i..].contains(&group[i - 1])) {
            println!("Part one: {}", i + 4);

            return;
        }
    }
}

fn part_two(input: &str) {
    for (i, group) in input.chars().collect::<Vec<char>>().windows(14).enumerate() {
        if !(1..group.len()).any(|i| group[i..].contains(&group[i - 1])) {
            println!("Part two: {}", i + 14);

            return;
        }
    }
}
