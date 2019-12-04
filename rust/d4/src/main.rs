use std::collections::HashMap;
use std::ops::RangeInclusive;

fn main() {
    let input = std::fs::read_to_string("../inputs/4.txt").unwrap();
    let range: RangeInclusive<u32> = {
        let mut numbers = input
            .split("-")
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .into_iter();
        numbers.next().unwrap()..=numbers.next().unwrap()
    };

    let mut part_one: Vec<u32> = Vec::new();
    let mut part_two: Vec<u32> = Vec::new();

    for num in range {
        if check_digit(num, false) {
            part_one.push(num);
        } 
        
        if check_digit(num, true) {
            part_two.push(num);
        }
    }

    println!("[Part One] Legal passwords: {}", part_one.len());
    println!("[Part Two] Legal passwords: {}", part_two.len());
}

fn check_digit(num: u32, part_two: bool) -> bool {
    let digits: Vec<u8> = num
        .to_string()
        .chars()
        .map(|n| n.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();

    let mut last_digit = 0u8;
    let mut nums: HashMap<u8, u8> = HashMap::new();

    for d in digits {
        if d < last_digit {
            return false;
        } else {
            last_digit = d
        }

        if nums.contains_key(&d) {
            if let Some(count) = nums.get_mut(&d) {
                *count += 1
            }
        } else {
            nums.insert(d, 1);
        }
    }

    return if part_two {
        nums.values().any(|n| *n == 2u8)
    } else {
        nums.values().any(|n| *n >= 2u8)
    };
}
