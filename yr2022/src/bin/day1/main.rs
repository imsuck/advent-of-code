fn main() {
    let input = std::fs::read_to_string(yr2022::get_input()).expect("Can't read input D:");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let mut max_count = 0;
    let mut count = 0;

    for line in input.lines() {
        if line.parse::<u32>().is_ok() {
            count += line.parse::<u32>().unwrap();
        } else {
            max_count = std::cmp::max(count, max_count);
            count = 0;
        }
    }

    println!("Part one: {}", max_count);
}

fn part_two(input: &str) {
    let mut elves = [0; 3];

    let mut count = 0;

    for line in input.lines() {
        if line.parse::<u32>().is_ok() {
            count += line.parse::<u32>().unwrap();
            continue;
        }

        if count > elves[0] {
            elves[2] = elves[1];
            elves[1] = elves[0];
            elves[0] = count;
        } else if count > elves[1] {
            elves[2] = elves[1];
            elves[1] = count;
        } else if count > elves[2] {
            elves[2] = count;
        }

        count = 0;
    }

    println!("Part two: {}", elves.iter().sum::<u32>());
}
