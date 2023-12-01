// Can't wait to look back on this and call my former self stupid
// Future me you suck btw just wanted to let you know that in anticipation

use std::cmp::Ordering;

fn main() {
    // Part 1

    let input = include_str!("input.txt");

    let result: usize = input
        .lines()
        .map(|line| {
            let first_num = line
                .chars()
                .find(|ch| ch.is_numeric())
                .expect("line didn't have a number");

            let second_num = line
                .chars()
                .rev()
                .find(|ch| ch.is_numeric())
                .expect("line didn't have a number");

            format!("{first_num}{second_num}")
                .parse::<usize>()
                .expect("parse to usize failed")
        })
        .sum();

    println!("Part 1: The answer is {result}.");

    // Part 2

    let num_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let result: usize = input
        .lines()
        .map(|line| {
            let first_word_and_index = num_words
                .iter()
                .enumerate()
                .map(|(num, num_word)| (num + 1, line.find(*num_word)))
                .min_by(|(_, index1), (_, index2)| match (index1, index2) {
                    (None, None) => Ordering::Equal,
                    // The `Ord` derive on Option<T> gives me the reverse of the two below
                    (Some(_), None) => Ordering::Less,
                    (None, Some(_)) => Ordering::Greater,
                    (Some(num1), Some(num2)) => num1.cmp(num2),
                });

            let last_word_and_index = num_words
                .iter()
                .enumerate()
                .map(|(num, num_word)| (num + 1, line.rfind(*num_word)))
                .max_by(|(_, index1), (_, index2)| index1.cmp(index2));

            let (first_num_index, first_num) = line
                .chars()
                .enumerate()
                .find(|(_, ch)| ch.is_numeric())
                .map(|(index, ch)| {
                    (
                        index,
                        ch.to_string()
                            .parse::<usize>()
                            .expect("failed to parse to usize"),
                    )
                })
                .expect("line didn't have a number");

            let (last_num_index, last_num) = line
                .chars()
                .rev()
                .collect::<String>()
                .chars()
                .enumerate()
                .find(|(_, ch)| ch.is_numeric())
                .map(|(index, ch)| {
                    (
                        line.len() - index - 1,
                        ch.to_string()
                            .parse::<usize>()
                            .expect("failed to parse to usize"),
                    )
                })
                .expect("line didn't have a number");

            let first_num = if let Some((num, Some(index))) = first_word_and_index {
                if index < first_num_index {
                    num
                } else {
                    first_num
                }
            } else {
                first_num
            };

            let last_num = if let Some((num, Some(index))) = last_word_and_index {
                if index > last_num_index {
                    num
                } else {
                    last_num
                }
            } else {
                last_num
            };

            format!("{first_num}{last_num}")
                .parse::<usize>()
                .expect("parse to usize failed")
        })
        .sum();

    println!("Part 2: The answer is {result}.");
}
