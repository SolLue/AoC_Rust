use std::fs;
use std::path::PathBuf;

fn read_input(day: &str) -> String {
    let folder = fs::read_to_string("../../config.txt").expect("Something went wrong");
    let path = PathBuf::from(folder).join("2021").join(day);

    //println!("Looking for: {}", path.display());
    fs::read_to_string(&path).expect("could not read input")
}

fn main() {
    println!("2021 - Day Two");

    let input = read_input("day_two.txt");
    let instructions_str: Vec<&str> = input.split_whitespace().collect();

    let (depth_part_one, horizontal_part_one) = move_submarine_part_one(&instructions_str);
    let part_one = depth_part_one * horizontal_part_one;
    println!("Day Two, Part One: {}", part_one);

    let (depth_part_two, horizontal_part_two) = move_submarine_part_two(&instructions_str);
    let part_two = depth_part_two * horizontal_part_two;
    println!("Day Two, Part Two: {}", part_two);
}

fn move_submarine_part_one(instructions: &[&str]) -> (i32, i32) {
    let mut horizontal = 0;
    let mut depth = 0;
    for i in (0..instructions.len() - 1).step_by(2) {
        let nr = instructions[i + 1].parse::<i32>().unwrap();
        match instructions[i] {
            "forward" => horizontal += nr,
            "down" => depth += nr,
            "up" => depth -= nr,
            _ => println!("How did you get here"),
        }
    }
    (depth, horizontal)
}

fn move_submarine_part_two(instructions: &[&str]) -> (i32, i32) {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;
    for i in (0..instructions.len() - 1).step_by(2) {
        let nr = instructions[i + 1].parse::<i32>().unwrap();
        match instructions[i] {
            "forward" => {
                horizontal += nr;
                if aim != 0 {
                    depth += nr * aim;
                }
            }
            "down" => {
                aim += nr;
            }
            "up" => {
                aim -= nr;
            }
            _ => println!("How did you get here"),
        }
    }
    (depth, horizontal)
}
