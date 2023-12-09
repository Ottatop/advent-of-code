const INPUT: &str = include_str!("input.txt");

fn main() {
    // Part 1

    let races = parse_input_part_1();

    let result = races
        .iter()
        .map(|race| win_count(*race))
        .reduce(|acc, next| acc * next)
        .unwrap();

    println!("Part 1: The answer is {result}.");

    // Part 2

    let answer = win_count(parse_input_part_2());

    println!("Part 2: The answer is {answer}.");
}

#[derive(Clone, Copy)]
struct Race {
    time: usize,
    distance: usize,
}

fn win_count(race: Race) -> usize {
    let Race { time, distance } = race;

    (0..=time)
        .map(|charge_time| (time - charge_time) * charge_time)
        .filter(|&dist| dist > distance)
        .count()
}

fn parse_input_part_1() -> Vec<Race> {
    let mut lines = INPUT.lines();
    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|time| time.parse::<usize>().unwrap());
    let distances = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|dist| dist.parse::<usize>().unwrap());

    times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn parse_input_part_2() -> Race {
    let mut lines = INPUT.lines();
    let time = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    Race { time, distance }
}
