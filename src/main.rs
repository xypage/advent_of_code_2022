#![allow(dead_code)]
// #![allow(unused_variables)]

use std::env;
use std::fs;

mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Make sure there's 2-4 commands
    //  0: command to actually run it
    //  1: the day to run
    //  2: the part to run
    //  3: optional, the test to run (requires the part)
    assert!(args.len() > 1, "Need at least one argument to specify a day, one to specify part, and optionally one to say test.");
    assert!(
        args.len() < 5,
        "You can pass 3 arguments, the day, a part, and optionally to test, you passed {}",
        args.len()
    );

    // first arg specifies the day
    let day: usize = args[1].parse::<usize>().unwrap();
    // second arg specifies the part
    let part: usize = args[2].parse::<usize>().unwrap();

    let mut test_output: Option<String> = None;
    let input: String = {
        // less than 4 args means not testing
        if args.len() < 4 {
            let file = format!("./input/day{:02}", day);
            fs::read_to_string(&file).unwrap_or_else(|_| panic!("Error reading file {}", file))
        } else {
            // the test input file has the expected output as the first line, with the actual input following
            // so we split off the first line as the expected output
            let file = format!("./input/day{:02}-test", day);
            let test_input: String =
                fs::read_to_string(&file).unwrap_or_else(|_| panic!("Error reading file {}", file));
            // Split off that top line
            let result: Option<(&str, &str)> = test_input.split_once("\n");
            match result {
                Some((x, y)) => {
                    // Need to have String not &str because we drop test_input at the end of the closure here
                    // so there'd be nothing for the &str to reference

                    // We split on ___ which I'm using to separate my expected test results, that's arbitrary
                    // then we're setting the correct part's result to test_output
                    let (part_1_test, part_2_test) = x.split_once("___").unwrap();
                    test_output = Some(String::from({
                        if part == 1 {
                            part_1_test
                        } else {
                            part_2_test
                        }
                    }));
                    String::from(y)
                }
                None => {
                    panic!("Failed to split off top line of test input");
                }
            }
        }
    };

    // subtract 1 since day 1 is at index 0
    let solution = solutions::get_day(day - 1);

    match (test_output, part) {
        (Some(expected_out), 1) => {
            // Testing part 1
            assert_eq!(expected_out, solution.part1(&input), "Part 1 failed test");
            println!("Test passed for day {} part 1!", day)
        }
        (Some(expected_out), 2) => {
            // Testing part 2
            assert_eq!(expected_out, solution.part2(&input), "Part 2 failed test");
            println!("Test passed for day {} part 2!", day)
        }
        (None, 1) => {
            // Running part 1
            println!("Day {} part 1 returned: {}", day, solution.part1(&input));
        }
        (None, 2) => {
            // Running part 2
            println!("Day {} part 2 returned: {}", day, solution.part2(&input));
        }
        _ => {
            // Incorrect part number given
            panic!("You indicated part number {} which is invalid", part);
        }
    }
}
