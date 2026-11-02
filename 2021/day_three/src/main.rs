use std::fs;
use std::path::PathBuf;

fn read_input(day: &str) -> String {
    let folder = fs::read_to_string("../../config.txt").expect("Something went wrong");
    let path = PathBuf::from(folder).join("2021").join(day);

    //println!("Looking for: {}", path.display());
    fs::read_to_string(&path).expect("could not read input")
}

fn main() {
    println!("2021 - Day Three");

    let input = read_input("day_three.txt");
    let instructions_str: Vec<&str> = input.split_whitespace().collect();

    let result_one = part_one(&instructions_str);
    println!("Day Three, Part One: {}", result_one);

    let result_two = part_two(&instructions_str);
    println!("Day Three, Part Two: {}", result_two);
}

fn part_one(instructions: &[&str]) -> i64 {
    let mut gamma: String = "".to_string();
    let mut epsilon: String = "".to_string();

    for index in 0..instructions[0].len() {
        let (ones, zeroes) = get_amount_bits(instructions, index);

        if ones > zeroes {
            gamma.push('1');
            epsilon.push('0');
        } else if zeroes > ones {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    let gamma_nr = i64::from_str_radix(gamma.as_str(), 2).unwrap();
    let epsilon_nr = i64::from_str_radix(epsilon.as_str(), 2).unwrap();
    gamma_nr * epsilon_nr
}

fn get_amount_bits(instructions: &[&str], index: usize) -> (i32, i32) {
    let mut ones = 0;
    let mut zeroes = 0;

    for item in instructions {
        match item.chars().nth(index).unwrap() {
            '1' => ones += 1,
            '0' => zeroes += 1,
            _ => println!("Panic {}", item),
        }
    }
    (ones, zeroes)
}

fn part_two(instructions: &[&str]) -> i64 {
    let mut possible_oxygen = Vec::new();
    let mut index: usize = 0;
    let mut ones;
    let mut zeroes;

    while possible_oxygen.len() != 1 {
        if index == 0 {
            (ones, zeroes) = get_amount_bits(instructions, index);
        } else {
            (ones, zeroes) = get_amount_bits(&possible_oxygen, index);
        }
        let mut current = '1';
        if ones < zeroes {
            current = '0';
        }
        if index == 0 {
            for item in instructions {
                if item.chars().nth(index).unwrap() == current {
                    possible_oxygen.push(item);
                }
            }
        } else {
            let mut temp = Vec::new();
            for item in possible_oxygen {
                if item.chars().nth(index).unwrap() == current {
                    temp.push(item);
                }
            }
            possible_oxygen = temp;
        }
        index += 1;
    }

    let mut possible_scrubber = Vec::new();
    index = 0;

    while possible_scrubber.len() != 1 {
        if index == 0 {
            (ones, zeroes) = get_amount_bits(instructions, index);
        } else {
            (ones, zeroes) = get_amount_bits(&possible_scrubber, index);
        }
        let mut current = '0';
        if ones < zeroes {
            current = '1';
        }
        if index == 0 {
            for item in instructions {
                if item.chars().nth(index).unwrap() == current {
                    possible_scrubber.push(item);
                }
            }
        } else {
            let mut temp = Vec::new();
            for item in possible_scrubber {
                if item.chars().nth(index).unwrap() == current {
                    temp.push(item);
                }
            }
            possible_scrubber = temp;
        }
        index += 1;
    }
    let oxygen_nr = i64::from_str_radix(possible_oxygen[0], 2).unwrap();
    let scrubber_nr = i64::from_str_radix(possible_scrubber[0], 2).unwrap();
    oxygen_nr * scrubber_nr
}
