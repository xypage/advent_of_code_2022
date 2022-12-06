pub trait Solution {
    fn part1(&self, input: &String) -> String;
    fn part2(&self, input: &String) -> String;
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

const DAYS: [&'static dyn Solution; 5] = [
    &day01::Day01,
    &day02::Day02,
    &day03::Day03,
    &day04::Day04,
    &day05::Day05,
];

pub fn get_day(day: usize) -> &'static dyn Solution {
    return DAYS[day];
}
