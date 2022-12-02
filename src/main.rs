#![allow(dead_code)]

use std::fs;

mod day1;


fn main() {
    // println!("Hello, advent of code!");
    day1();
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
}

fn day1() {
    let contents = fs::read_to_string("./input/day1")
        .expect("Should have been able to read the file");
    println!("{}", day1::part_one(&contents));
    println!("{}", day1::part_two(&contents));
}
