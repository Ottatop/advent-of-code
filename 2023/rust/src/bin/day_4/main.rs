use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn main() {
    // Part 1

    let result = INPUT
        .lines()
        .map(|game| match win_count(game) {
            0 => 0,
            1 => 1,
            count => 2usize.pow((count - 1) as u32),
        })
        .sum::<usize>();

    println!("Part 1: The answer is {result}.");

    // Part 2

    let mut instances = vec![1usize; INPUT.lines().count()];

    for game_num in 0..instances.len() {
        let win_count = win_count(INPUT.lines().nth(game_num).unwrap());

        let current_instance_count = *instances.get(game_num).unwrap();

        for i in (game_num + 1)..(game_num + 1 + win_count) {
            if let Some(instance_count) = instances.get_mut(i) {
                *instance_count += current_instance_count;
            }
        }
    }

    let result = instances.into_iter().sum::<usize>();

    println!("Part 2: The answer is {result}.");
}

fn win_count(game: &str) -> usize {
    let nums = game.split(": ").last().unwrap();
    let mut split = nums.split('|');

    let winning_nums = split.next().unwrap();
    let numbers_you_have = split.next().unwrap();

    let winning_nums = winning_nums
        .split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<HashSet<_>>();

    let numbers_you_have = numbers_you_have
        .split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<HashSet<_>>();

    winning_nums.intersection(&numbers_you_have).count()
}
