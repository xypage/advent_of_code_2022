pub fn part_one(input: &String) -> u32 {
    let lines = input.lines();
    let mut count = 0;
    for line in lines {
        let mut elves = line.split(",");
        let (first_start, first_stop) = {
            let mut split = elves.next().unwrap().split("-");
            (
                split.next().unwrap().parse::<u32>().unwrap(),
                split.next().unwrap().parse::<u32>().unwrap(),
            )
        };

        let (second_start, second_stop) = {
            let mut split = elves.next().unwrap().split("-");
            (
                split.next().unwrap().parse::<u32>().unwrap(),
                split.next().unwrap().parse::<u32>().unwrap(),
            )
        };

        if (first_start <= second_start && first_stop >= second_stop)
            || (second_start <= first_start && second_stop >= first_stop)
        {
            count += 1;
        }
    }
    return count;
}

pub fn part_two(input: &String) -> u32 {
    let lines = input.lines();
    let mut count = 0;
    for line in lines {
        let mut elves = line.split(",");
        let (first_start, first_stop) = {
            let mut split = elves.next().unwrap().split("-");
            (
                split.next().unwrap().parse::<u32>().unwrap(),
                split.next().unwrap().parse::<u32>().unwrap(),
            )
        };

        let (second_start, second_stop) = {
            let mut split = elves.next().unwrap().split("-");
            (
                split.next().unwrap().parse::<u32>().unwrap(),
                split.next().unwrap().parse::<u32>().unwrap(),
            )
        };

        if (first_start <= second_start && first_stop >= second_start)
            || (second_start <= first_start && second_stop >= first_start)
        {
            count += 1;
        }
    }
    return count;
}