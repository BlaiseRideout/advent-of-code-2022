use std::io;

enum Instruction {
    Add { count: i32 },
    Noop,
}

fn parse_commands(lines: &Vec<String>) -> Vec<Instruction> {
    lines
        .iter()
        .filter_map(|line| {
            let mut words = line.split_whitespace();
            match words.next().expect("Couldn't get first word") {
                "addx" => Some(Instruction::Add {
                    count: words
                        .next()
                        .expect("Couldn't get argument")
                        .parse::<i32>()
                        .expect("Couldn't parse argument"),
                }),
                "noop" => Some(Instruction::Noop),
                _ => None,
            }
        })
        .collect()
}

fn run_program(commands: &Vec<Instruction>) -> Vec<i32> {
    commands
        .iter()
        .fold(vec![1, 1], |x_over_t, command| -> Vec<i32> {
            let x = *x_over_t.last().expect("Couldn't get current x val");
            match command {
                Instruction::Add { count } => x_over_t.into_iter().chain([x, x + count]).collect(),
                Instruction::Noop => x_over_t.into_iter().chain([x]).collect(),
            }
        })
}

fn part1(registers_over_t: &Vec<i32>) -> i32 {
    [20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|ind| registers_over_t[ind] * ind as i32)
        .sum()
}

fn part2(registers_over_t: &Vec<i32>) {
    static HEIGHT: usize = 6;
    static WIDTH: usize = 40;

    println!(
        "{}",
        (0..HEIGHT)
            .map(|y| (0..WIDTH)
                .map(|x| {
                    let cycle_index = y * WIDTH + x + 1;
                    let x_min: i32 = x as i32 - 1;
                    let x_max: i32 = x as i32 + 1;
                    if (x_min..=x_max).contains(&registers_over_t[cycle_index]) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>())
            .fold(String::new(), |a, s| a + &s + "\n")
    );
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();

    let commands = parse_commands(&lines);
    let registers_over_t = run_program(&commands);

    println!("Part 1: {:?}", part1(&registers_over_t));
    println!("Part 2:");
    part2(&registers_over_t);
}
