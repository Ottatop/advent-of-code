const INPUT: &str = include_str!("input.txt");

fn main() {
    // Part 1

    let histories = parse_input();

    let answer = histories
        .iter()
        .map(|history| {
            let mut sequences = vec![history.clone()];
            while !sequences.last().unwrap().iter().all(|&num| num == 0) {
                let last_seq = sequences.last().unwrap();
                let next_seq = last_seq
                    .windows(2)
                    .map(|nums| nums[1] - nums[0])
                    .collect::<Vec<_>>();
                sequences.push(next_seq);
            }

            sequences.reverse();

            sequences
                .into_iter()
                .skip(1)
                .fold(0, |acc, history| history.last().unwrap() + acc)
        })
        .sum::<isize>();

    println!("Part 1: The answer is {answer}.");

    // Part 2

    let answer = histories
        .iter()
        .map(|history| {
            let mut sequences = vec![history.clone()];
            while !sequences.last().unwrap().iter().all(|&num| num == 0) {
                let last_seq = sequences.last().unwrap();
                let next_seq = last_seq
                    .windows(2)
                    .map(|nums| nums[1] - nums[0])
                    .collect::<Vec<_>>();
                sequences.push(next_seq);
            }

            sequences.reverse();

            sequences
                .into_iter()
                .skip(1)
                .fold(0, |acc, history| history.first().unwrap() - acc)
        })
        .sum::<isize>();

    println!("Part 2: The answer is {answer}.");
}

fn parse_input() -> Vec<Vec<isize>> {
    INPUT
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<isize>().unwrap())
                .collect()
        })
        .collect()
}
