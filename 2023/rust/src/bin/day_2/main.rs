fn main() {
    let input = include_str!("input.txt");

    // Part 1

    let result: usize = input
        .lines()
        .enumerate()
        .filter(|(_, game)| {
            let mut cubes = parse(game);

            cubes.all(|cubes| cubes.red <= 12 && cubes.green <= 13 && cubes.blue <= 14)
        })
        .map(|(i, _)| i + 1)
        .sum();

    println!("Part 1: The answer is {result}.");

    // Part 2

    let result: u32 = input
        .lines()
        .map(|game| {
            let cubes = parse(game);

            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;

            for cube in cubes {
                if cube.red > min_red {
                    min_red = cube.red;
                }

                if cube.green > min_green {
                    min_green = cube.green;
                }

                if cube.blue > min_blue {
                    min_blue = cube.blue;
                }
            }

            min_red * min_green * min_blue
        })
        .sum();

    println!("Part 2: The answer is {result}.");
}

fn parse(game: &str) -> impl Iterator<Item = Cubes> + '_ {
    let cubes = game.split(": ").last().unwrap();
    let plays = cubes.split("; ");

    plays.map(|play| {
        let colors = play.split(", ");

        let mut cubes = Cubes::default();

        for color in colors {
            let mut num_and_color = color.split(' ');
            let num = num_and_color.next().unwrap().parse::<u32>().unwrap();
            let color = num_and_color.next().unwrap();

            match color {
                "red" => cubes.red = num,
                "green" => cubes.green = num,
                "blue" => cubes.blue = num,
                _ => unreachable!(),
            }
        }

        cubes
    })
}

#[derive(Default)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}
