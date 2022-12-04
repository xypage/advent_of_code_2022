use std::str::FromStr;

pub struct Compartment {
    // Using this just for the binary
    pub priorities: u64,
}

impl FromStr for Compartment {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut priorities: u64 = 0;
        let s = s.as_bytes();
        for char in s {
            if *char < 97 {
                // if the letter is capital

                // A is supposed to be worth 27 points, it's ascii value is 65, so 65-38 gives us that 27
                // since we start with 1 though, we only need to shift it 27-1 times, so we do 65-39
                priorities |= 1 << (char - 39);
            } else {
                // if it's lower case

                // a is worth 1, it's ascii value is 97, we start with a 1 so we don't want to shift at all for that
                priorities |= 1 << (char - 97);
            }
        }
        Ok(Compartment { priorities })
    }
}

pub fn part_one(input: &String) -> u32 {
    let lines = input.lines();
    let mut sum: u32 = 0;

    for line in lines {
        let len = line.len();

        let left_compartment = Compartment::from_str(&line[..len / 2]).unwrap();
        let right_compartment = Compartment::from_str(&line[len / 2..]).unwrap();

        let matching = left_compartment.priorities & right_compartment.priorities;

        let priority = matching.ilog2() + 1;

        sum += priority;
    }

    return sum;
}

pub fn part_two(input: &String) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    let chunks = lines.chunks(3);
    let mut sum: u32 = 0;

    for triplet in chunks {
        let value = triplet.into_iter().fold(u64::MAX, |acc, sack| {
            acc & Compartment::from_str(sack).unwrap().priorities
        });
        sum += value.ilog2() + 1;
        // let len = line.len();

        // let left_compartment = Compartment::from_str(&line[..len / 2]).unwrap();
        // let right_compartment = Compartment::from_str(&line[len / 2..]).unwrap();

        // let matching = left_compartment.priorities & right_compartment.priorities;

        // let priority = matching.ilog2() + 1;

        // sum += priority;
    }

    return sum;
}
