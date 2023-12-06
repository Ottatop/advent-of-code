use rayon::{iter::ParallelIterator, slice::ParallelSlice};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let seeds = INPUT
        .lines()
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(' ')
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let maps = parse_input();

    // Part 1

    let mut inputs = seeds.clone();

    for map in maps.iter() {
        inputs = inputs
            .into_iter()
            .map(|input| map_num(input, map))
            .collect();
    }

    let answer = inputs.into_iter().min().unwrap();

    println!("Part 1: The answer is {answer}.");

    // Part 2 aka why think when brute force do trick

    let answer = seeds
        .par_chunks(2)
        // .chunks(2)
        .flat_map(|chunk| {
            let start = chunk[0];
            let len = chunk[1];
            start..(start + len)
        })
        .map(|mut input| {
            for map in maps.iter() {
                input = map_num(input, map);
            }

            input
        })
        .min()
        .unwrap();

    println!("Part 2: The answer is {answer}.");
}

fn map_num(num: usize, maps: &[[usize; 3]]) -> usize {
    let mut result = num;

    for map in maps {
        let range_size = map[2];
        let source_start = map[1];
        let dest_start = map[0];

        if (source_start..(source_start + range_size)).contains(&num) {
            result = dest_start + (num - source_start);
        }
    }

    result
}

fn parse_input() -> Vec<Vec<[usize; 3]>> {
    let mut lines = INPUT.lines();
    lines.next();

    let mut maps: Vec<Vec<[usize; 3]>> = Vec::new();

    let mut map: Vec<[usize; 3]> = Vec::new();

    for line in lines {
        if line.is_empty() {
            if !map.is_empty() {
                maps.push(std::mem::take(&mut map));
            }
        } else if !line.chars().next().unwrap().is_numeric() {
            continue;
        } else {
            let mut nums = [0usize; 3];
            let nums_split = line.split(' ');
            for (i, num) in nums_split.enumerate() {
                nums[i] = num.parse().unwrap();
            }

            map.push(nums);
        }
    }

    maps.push(map);

    maps
}
