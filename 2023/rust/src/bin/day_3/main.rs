use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    // Part 1

    let grid = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut num_index_start: Option<usize> = None;
    let mut num_index_end: Option<usize> = None;

    let mut sum = 0;

    for (i, line) in grid.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if !ch.is_numeric() {
                if num_index_start.is_some() && num_index_end.is_none() {
                    num_index_end = num_index_start;
                }
                if let (Some(start), Some(end)) = (num_index_start, num_index_end) {
                    if let Some(part_num) = part_num_at(i, start, end) {
                        sum += part_num;
                    }
                }

                num_index_start = None;
                num_index_end = None;
                continue;
            }

            if num_index_start.is_none() {
                num_index_start = Some(j);
            } else {
                num_index_end = Some(j);
            }
        }

        if num_index_start.is_some() && num_index_end.is_none() {
            num_index_end = num_index_start;
        }
        if let (Some(start), Some(end)) = (num_index_start, num_index_end) {
            if let Some(part_num) = part_num_at(i, start, end) {
                sum += part_num;
            }
        }

        num_index_start = None;
        num_index_end = None;
    }

    println!("Part 1: The answer is {sum}.");

    // Part 2 aka "I took the Don't-Repeat-Yourself principle out back and shot it"

    let mut gear_map = HashMap::<(usize, usize), Vec<usize>>::new();

    for (i, line) in grid.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if !ch.is_numeric() {
                if num_index_start.is_some() && num_index_end.is_none() {
                    num_index_end = num_index_start;
                }
                if let (Some(start), Some(end)) = (num_index_start, num_index_end) {
                    gear_part_at(i, start, end, &mut gear_map);
                }

                num_index_start = None;
                num_index_end = None;
                continue;
            }

            if num_index_start.is_none() {
                num_index_start = Some(j);
            } else {
                num_index_end = Some(j);
            }
        }

        if num_index_start.is_some() && num_index_end.is_none() {
            num_index_end = num_index_start;
        }
        if let (Some(start), Some(end)) = (num_index_start, num_index_end) {
            gear_part_at(i, start, end, &mut gear_map);
        }

        num_index_start = None;
        num_index_end = None;
    }

    let result = gear_map
        .values()
        .filter(|vec| vec.len() == 2)
        .map(|vec| vec[0] * vec[1])
        .sum::<usize>();

    println!("Part 2: The answer is {result}.");
}

fn part_num_at(line_num: usize, start: usize, end: usize) -> Option<usize> {
    let top_left_line = line_num.saturating_sub(1);
    let top_left_char = start.saturating_sub(1);
    let bottom_right_line = usize::min(line_num + 1, INPUT.lines().count() - 1);
    let bottom_right_char = usize::min(end + 1, INPUT.lines().next().unwrap().len() - 1);

    let mut is_part = false;

    'outer: for i in top_left_line..=bottom_right_line {
        for j in top_left_char..=bottom_right_char {
            let ch = INPUT.lines().nth(i).unwrap().chars().nth(j).unwrap();
            if !ch.is_numeric() && ch != '.' {
                is_part = true;
                break 'outer;
            }
        }
    }

    if is_part {
        let num = INPUT.lines().nth(line_num).unwrap()[start..=end]
            .parse::<usize>()
            .unwrap();

        Some(num)
    } else {
        None
    }
}

// They say your code should be DRY.
//
// Well this code is wet af lemme tell ya that
fn gear_part_at(
    line_num: usize,
    start: usize,
    end: usize,
    gear_map: &mut HashMap<(usize, usize), Vec<usize>>,
) {
    let top_left_line = line_num.saturating_sub(1);
    let top_left_char = start.saturating_sub(1);
    let bottom_right_line = usize::min(line_num + 1, INPUT.lines().count() - 1);
    let bottom_right_char = usize::min(end + 1, INPUT.lines().next().unwrap().len() - 1);

    let mut is_gear_part = false;

    let mut gear_line = 0;
    let mut gear_char = 0;

    'outer: for i in top_left_line..=bottom_right_line {
        for j in top_left_char..=bottom_right_char {
            let ch = INPUT.lines().nth(i).unwrap().chars().nth(j).unwrap();
            if ch == '*' {
                is_gear_part = true;
                gear_line = i;
                gear_char = j;
                break 'outer;
            }
        }
    }

    if is_gear_part {
        let num = INPUT.lines().nth(line_num).unwrap()[start..=end]
            .parse::<usize>()
            .unwrap();

        gear_map
            .entry((gear_line, gear_char))
            .or_insert(Vec::new())
            .push(num);
    }
}
