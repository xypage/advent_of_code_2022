#![allow(dead_code)]

use std::fs;

mod day1;
mod day2;

fn main() {
    // println!("Hello, advent of code!");
    // day1();
    day2();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day1_part_one() {
        let example: String = String::from("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
        assert_eq!(24000, day1::part_one(&example));
    }
    #[test]
    fn day1_part_two() {
        let example: String = String::from("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
        assert_eq!(45000, day1::part_two(&example));
    }

    #[test]
    fn day2_part_one() {
        let example: String = String::from("A Y\nB X\nC Z");
        assert_eq!(15, day2::part_one(&example));
    }
    #[test]
    fn day2_part_two() {
        let example: String = String::from("A Y\nB X\nC Z");
        assert_eq!(12, day2::part_two(&example));
    }
}

fn day1() {
    let contents = fs::read_to_string("./input/day1")
        .expect("Should have been able to read the file");
    println!("{}", day1::part_one(&contents));
    println!("{}", day1::part_two(&contents));
}

fn day2() {
    let contents = fs::read_to_string("./input/day2")
        .expect("Should have been able to read the file");
    println!("{}", day2::part_one(&contents));
    println!("{}", day2::part_two(&contents));
}
