use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    // Part 1

    let ParsedInput { instrs, nodes } = parse_input();

    let instrs = instrs.chars().cycle().enumerate();

    let mut current_node = "AAA".to_string();

    for (i, instr) in instrs {
        match instr {
            'L' => {
                current_node = nodes.get(&current_node).unwrap().0.clone();
            }
            'R' => {
                current_node = nodes.get(&current_node).unwrap().1.clone();
            }
            _ => unreachable!(),
        }

        if current_node == "ZZZ" {
            println!("Part 1: The answer is {}", i + 1);
            break;
        }
    }

    // Part 2

    let ParsedInput { instrs, nodes } = parse_input();
    let instrs = instrs.chars().cycle();
    let mut current_nodes = nodes
        .keys()
        .filter(|key| key.ends_with('A'))
        .cloned()
        .zip(std::iter::repeat(0))
        .collect::<Vec<(_, usize)>>();

    for instr in instrs {
        match instr {
            'L' => {
                for (node, path_len) in current_nodes
                    .iter_mut()
                    .filter(|(node, _)| !node.ends_with('Z'))
                {
                    *node = nodes.get(node).unwrap().0.clone();
                    *path_len += 1;
                }
            }
            'R' => {
                for (node, path_len) in current_nodes
                    .iter_mut()
                    .filter(|(node, _)| !node.ends_with('Z'))
                {
                    *node = nodes.get(node).unwrap().1.clone();
                    *path_len += 1;
                }
            }
            _ => unreachable!(),
        }

        if current_nodes.iter().all(|(node, _)| node.ends_with('Z')) {
            break;
        }
    }

    let path_lens = current_nodes
        .into_iter()
        .map(|(_, path_len)| path_len)
        .collect::<Vec<_>>();

    let answer = lcm(path_lens.as_slice());

    println!("Part 2: The answer is {answer}.");
}

struct ParsedInput {
    instrs: String,
    nodes: HashMap<String, (String, String)>,
}

fn parse_input() -> ParsedInput {
    let mut lines = INPUT.lines();

    let instrs = lines.next().unwrap().to_string();
    lines.next(); // empty line

    let mut nodes = HashMap::<String, (String, String)>::new();

    for node in lines {
        let mut split = node.split_whitespace();
        let key = split.next().unwrap().to_string();
        split.next(); // =
        let left = split.next().unwrap()[1..4].to_string();
        let right = split.next().unwrap()[0..3].to_string();

        nodes.insert(key, (left, right));
    }

    ParsedInput { instrs, nodes }
}

// Shamelesssly stolen from https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
