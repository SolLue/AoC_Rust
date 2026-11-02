use std::fs;
use std::path::PathBuf;

fn read_input(day: &str) -> String {
    let folder = fs::read_to_string("../../config.txt").expect("Something went wrong");
    let path = PathBuf::from(folder).join("2021").join(day);

    //println!("Looking for: {}", path.display());
    fs::read_to_string(&path).expect("could not read input")
}

fn main() {
    println!("2021 - Day One");

    let input = read_input("day_one.txt");
    let sweep_str: Vec<&str> = input.split_whitespace().collect();
    let sweep_nr: Vec<i32> = sweep_str.iter().map(|s| s.parse().unwrap()).collect();

    //println!("{:?}", sweep_nr);

    let amount_part_one = depth_increase_amount(&sweep_nr);
    println!("Day One, Part One: {}", amount_part_one);

    let amount_part_two = depth_increase_amount_three(&sweep_nr);
    println!("Day One, Part Two: {}", amount_part_two);
}

fn depth_increase_amount(vec: &[i32]) -> i16 {
    let mut previous = vec[0];
    let mut amount = 0;
    for sweep in vec {
        if previous < *sweep {
            amount += 1;
        }
        previous = *sweep;
    }
    amount
}

fn depth_increase_amount_three(vec: &[i32]) -> i16 {
    let mut previous = vec[0] + vec[1] + vec[2];
    let mut amount = 0;
    for i in 1..vec.len() - 2 {
        let next = vec[i] + vec[i + 1] + vec[i + 2];
        if previous < next {
            amount += 1;
        }
        previous = next;
    }
    amount
}
