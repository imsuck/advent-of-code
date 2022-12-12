fn main() {
    let input = std::fs::read_to_string(yr2022::get_input()).expect("Can't read input D:");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let mut total_score = 0;

    for turn in input.lines() {
        total_score += get_score_part_one(turn);
    }

    println!("Part one: {}", total_score);
}

fn part_two(input: &str) {
    let mut total_score = 0;

    for turn in input.lines() {
        total_score += get_score_part_two(turn);
    }

    println!("Part two: {}", total_score);
}

fn get_score_part_one(turn: &str) -> u32 {
    let (opponent, us) = turn.split_once(' ').unwrap();
    let mut score;

    match (us, opponent) {
        ("X", "B") | ("Y", "C") | ("Z", "A") => score = 0,
        ("X", "A") | ("Y", "B") | ("Z", "C") => score = 3,
        ("X", "C") | ("Y", "A") | ("Z", "B") => score = 6,
        _ => score = 69420,
    }

    score += match us {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };

    score
}

fn get_score_part_two(turn: &str) -> u32 {
    let (opponent, result) = turn.split_once(' ').unwrap();
    let mut score;

    match result {
        "X" => score = 0,
        "Y" => score = 3,
        "Z" => score = 6,
        _ => score = 69420,
    }

    score += match (result, opponent) {
        ("X", "B") | ("Y", "A") | ("Z", "C") => 1,
        ("X", "C") | ("Y", "B") | ("Z", "A") => 2,
        ("X", "A") | ("Y", "C") | ("Z", "B") => 3,
        _ => 0,
    };

    score
}
