fn main() {
    let input = std::fs::read_to_string(yr2022::get_input()).expect("Can't read input D:");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let mut count = 0u32;

    for line in input.lines() {
        let (elf1, elf2) = line.split_once(',').unwrap();
        let (elf1_start, elf1_end) = get_range(elf1);
        let (elf2_start, elf2_end) = get_range(elf2);

        if elf1_start <= elf2_start && elf2_end <= elf1_end
            || elf2_start <= elf1_start && elf1_end <= elf2_end
        {
            count += 1;
        }
    }

    println!("Part one: {count}");
}

fn part_two(input: &str) {
    let mut count = 0u32;

    for line in input.lines() {
        let (elf1, elf2) = line.split_once(',').unwrap();
        let (elf1_start, elf1_end) = get_range(elf1);
        let (elf2_start, elf2_end) = get_range(elf2);

        if elf2_start <= elf1_end && elf1_start <= elf2_end {
            count += 1;
        }
    }

    println!("Part two: {count}");
}

fn get_range(str: &str) -> (u32, u32) {
    str.split_once('-')
        .map(|v| (v.0.parse::<u32>().unwrap(), v.1.parse::<u32>().unwrap()))
        .unwrap()
}
