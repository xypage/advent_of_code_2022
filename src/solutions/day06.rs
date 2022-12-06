use std::collections::HashMap;
use std::string::ToString;

use crate::solutions::Solution;

pub struct Day06;
impl Solution for Day06 {
    fn part1(&self, input: &String) -> String {
        // This is the original code I used, however my improved solutions work here as well if
        // I just shift the numbers so I'm doing that here too
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

        // Copied my second improved solution from part 2
        const PACKET_SIZE: i32 = 4;

        let mut last_seen: [usize; 26] = [0; 26];
        let mut countdown: i32 = 0;
        for (index, char) in input.as_bytes().iter().enumerate() {
            let dist: i32 = (index - last_seen[(char - b'a') as usize]) as i32;
            last_seen[(char - b'a') as usize] = index;
            countdown -= 1;

            if dist >= PACKET_SIZE && countdown < 1 {
                return (index + 1).to_string();
            }

            if PACKET_SIZE - dist > countdown {
                countdown = PACKET_SIZE - dist;
            }
        }

        String::from("No match")
    }

    fn part2(&self, input: &String) -> String {
        // // Make a blank tracker struct
        // let mut t = Tracker::new(14);
        // // turn it to bytes to iterate and store more easily, then go through all of them
        // for char in input.as_bytes() {
        //     // push the current character
        //     t.insert(*char);
        //     // and checking till we get a positive, not technically necessary for the first 14 but also very easy
        //     if t.check() {
        //         // if it does tell us that we have a positive, we can use the tracker's counter to know what index that character is
        //         return t.counter.to_string();
        //     }
        // }
        // // random string to return if nothing else
        // String::from("No match, didn't work")

        // The size of a packet, makes this a lot easier to use for part 1 or any other theoretical size that's similar
        const PACKET_SIZE: i32 = 14;

        // an array of the indices where a given character last showed up
        let mut last_seen: [usize; 26] = [0; 26];
        // A countdown for when the current nearest repeat drops out
        //  For example, with a window of 4, if we had x(yzza)_bba then when you were at the cursor, you have a countdown of 2
        //  since you need to move 2 for that old z to drop out, then you move forward and have xy(zzab)_ba and have
        //  a countdown of 1 because you move 1 and if nothing else comes in you can send it, giving xyz(zabb)_a and now your
        //  countdown is 3 since you need to move the b out, etc.
        let mut countdown: i32 = 0;
        for (index, char) in input.as_bytes().iter().enumerate() {
            // Since we iterated forwards 1 we decrement the counter
            countdown -= 1;

            // get the distance between the index and the index of the last time you saw that character
            let dist: i32 = (index - last_seen[(char - b'a') as usize]) as i32;
            // since that's stored, we can write the current index to the "last seen" time for that character
            last_seen[(char - b'a') as usize] = index;

            // if the distance is larger than the packet size, and the countdown is < 1 (so we don't have any more to wait)
            if dist >= PACKET_SIZE && countdown < 1 {
                // then we got it, return that + 1 to 1-index it
                return (index + 1).to_string();
            }

            // otherwise just update countdown if the new "time to wait till the duplicate drops off"
            // is greater than the current one is
            if PACKET_SIZE - dist > countdown {
                countdown = PACKET_SIZE - dist;
            }
        }

        String::from("No match")
    }
}

struct Tracker {
    size: usize,
    list: Vec<u8>,
    map: HashMap<u8, u32>,
    counter: u32,
}

impl Tracker {
    fn new(size: usize) -> Tracker {
        Tracker {
            size,
            list: Vec::new(),
            // not strictly necessary but since we know at the start might as well
            map: HashMap::new(),
            counter: 0,
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

    fn check(&self) -> bool {
        // length of the map is the number of elements which is going to be the number of elements in the
        // current window so when that's the same as the size, we know each element is unique otherwise
        // we'd have fewer keys since some would have a value of 2
        return self.map.len() == self.size;
    }
}
