#![allow(dead_code)]
// #![allow(unused_variables)]

use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    day5();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day1_part_one() {
        let example: String =
            String::from("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
        assert_eq!(24000, day1::part_one(&example));
    }
    #[test]
    fn day1_part_two() {
        let example: String =
            String::from("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
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

    #[test]
    fn day3_part_one() {
        let example: String = String::from("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw");
        assert_eq!(157, day3::part_one(&example));
    }
    #[test]
    fn day3_part_two() {
        let example: String = String::from("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw");
        assert_eq!(70, day3::part_two(&example));
    }

    #[test]
    fn day4_part_one() {
        let example: String = String::from("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8");
        assert_eq!(2, day4::part_one(&example));
    }
    #[test]
    fn day4_part_two() {
        let example: String = String::from("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8");
        assert_eq!(4, day4::part_two(&example));
    }

    #[test]
    fn day5_part_one() {
        let example: String = String::from(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        );
        assert_eq!("CMZ", day5::part_one(&example));
    }
    #[test]
    fn day5_part_two() {
        let example: String = String::from(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        );
        assert_eq!("MCD", day5::part_two(&example));
    }
}

fn day1() {
    let contents =
        fs::read_to_string("./input/day1").expect("Should have been able to read the file");
    println!("Part one: {}", day1::part_one(&contents));
    println!("Part two: {}", day1::part_two(&contents));
}

fn day2() {
    let contents =
        fs::read_to_string("./input/day2").expect("Should have been able to read the file");
    println!("Part one: {}", day2::part_one(&contents));
    println!("Part two: {}", day2::part_two(&contents));
}

fn day3() {
    let contents =
        fs::read_to_string("./input/day3").expect("Should have been able to read the file");
    println!("Part one: {}", day3::part_one(&contents));
    println!("Part two: {}", day3::part_two(&contents));
}

fn day4() {
    let contents =
        fs::read_to_string("./input/day4").expect("Should have been able to read the file");
    println!("Part one: {}", day4::part_one(&contents));
    println!("Part two: {}", day4::part_two(&contents));
}

fn day5() {
    let contents =
        fs::read_to_string("./input/day5").expect("Should have been able to read the file");
    println!("Part one: {}", day5::part_one(&contents));
    println!("Part two: {}", day5::part_two(&contents));
}
