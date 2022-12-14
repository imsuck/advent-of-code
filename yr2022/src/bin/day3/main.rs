use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string(yr2022::get_input()).expect("Can't read input D:");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let mut priorities = Vec::new();

    for rucksack in input.lines() {
        bad_item_type(rucksack).iter().for_each(|char| {
            priorities.push(*char);
        });
    }

    println!(
        "Part one: {}",
        priorities.iter().map(char_to_priority).sum::<u32>()
    );
}

fn part_two(input: &str) {
    let mut priorities = Vec::new();

    for group in input.lines().collect::<Vec<&str>>().chunks(3) {
        let bag1 = item_types(group[0]);
        let bag2 = item_types(group[1]);
        let bag3 = item_types(group[2]);

        let temp = bag1.intersection(&bag2).copied().collect::<HashSet<char>>();
        let badge = temp.intersection(&bag3);

        // TODO: - take list of all item types
        //       - find intersection of all of them
        //       - do that vvvv

        badge.for_each(|char| {
            priorities.push(*char);
        });
    }

    println!(
        "Part two: {}",
        priorities.iter().map(char_to_priority).sum::<u32>()
    );
}

fn char_to_priority(char: &char) -> u32 {
    (match *char {
        char if ('a'..='z').contains(&char) => char as u8 - 96,
        char if ('A'..='Z').contains(&char) => char as u8 - 38,
        _ => 0,
    }) as u32
}

fn item_types(list: &str) -> HashSet<char> {
    let mut set = HashSet::new();

    for char in list.chars() {
        set.insert(char);
    }

    set
}

fn bad_item_type(rucksack: &str) -> HashSet<char> {
    let (comp1, comp2) = rucksack.split_at(rucksack.len() / 2);
    let comp1_set = item_types(comp1);
    let comp2_set = item_types(comp2);

    comp1_set
        .intersection(&comp2_set)
        .copied()
        .collect::<HashSet<char>>()
}
