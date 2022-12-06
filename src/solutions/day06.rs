use std::string::ToString;
use std::collections::HashMap;

use crate::solutions::Solution;

pub struct Day06;
impl Solution for Day06 {
    fn part1(&self, input: &String) -> String {
        // This is the original code I used, however the solution for part2 works for part1 if I just modify 
        // the tracker size so I've changed it to just using that as well
        // let iter = input.as_bytes().windows(4);
        // for (i, window) in iter.enumerate() {
        //     if window[0] != window[1] && window [0] != window [2] && window[0] != window[3] &&
        //         window[1] != window[2] && window[1] != window[3] &&
        //         window[2] != window[3] {
        //             // wants the index of the last character in the window so + 3, then + 1 to 1-index it
        //             return (i + 4).to_string();
        //         }
        // }

        // // Random string to return if nothing else
        // String::from("No match, didn't work")
        
        // Here's the only difference from part2, I set the size to 4
        let mut t = Tracker::new(4);
        for char in input.as_bytes() {
            t.insert(*char);
            if t.check() {
                return t.counter.to_string();
            }
        }
        String::from("No match, didn't work")
    }

    fn part2(&self, input: &String) -> String {
        // Make a blank tracker struct
        let mut t = Tracker::new(14);
        // turn it to bytes to iterate and store more easily, then go through all of them
        for char in input.as_bytes() {
            // push the current character
            t.insert(*char);
            // and checking till we get a positive, not technically necessary for the first 14 but also very easy
            if t.check() {
                // if it does tell us that we have a positive, we can use the tracker's counter to know what index that character is
                return t.counter.to_string();
            }
        }
        // random string to return if nothing else
        String::from("No match, didn't work")
    }
}

struct Tracker {
    size: usize,
    list: Vec<u8>,
    map: HashMap<u8, u32>,
    counter: u32
}

impl Tracker {
    fn new(size: usize) -> Tracker {
        Tracker {
            size,
            list: Vec::new(),
            // not strictly necessary but since we know at the start might as well
            map: HashMap::new(),
            counter: 0
        }
    }

    fn insert(&mut self, val: u8) {
        // Make sure we increment the counter
        self.counter += 1;

        // Add the tip
        // add it to the tip of the vector
        self.list.push(val);
        // if it's in the hashmap, increment the count by 1, otherwise start a new count
        self.map.entry(val).and_modify(|num| *num += 1).or_insert(1);

        // Remove the last one if the current length is > size, since we have one added right now and
        // obviously haven't removed one yet so we want to remove one to get us to size, not remove one when we're at size
        if self.list.len() > self.size {
            let last = self.list.remove(0);
            // Check if the dropped value was a duplicate in the hash, if not then remove it, if so just decrement
            let count = self.map.get(&last).unwrap();
            if *count == 1 {
                self.map.remove(&last);
            } else {
                self.map.insert(last, count - 1);
            }
        }
    }

    fn check(&self) -> bool{
        // length of the map is the number of elements which is going to be the number of elements in the 
        // current window so when that's the same as the size, we know each element is unique otherwise
        // we'd have fewer keys since some would have a value of 2
        return self.map.len() == self.size;
    }
}