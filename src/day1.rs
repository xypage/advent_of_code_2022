pub fn part_one(input: &String) -> u32 {
    let lines = input.lines();

    let mut current_calories: u32 = 0;
    let mut max_calories: u32 = 0;

    for line in lines {
        if let Ok(x) = line.parse::<u32>() {
            current_calories += x;
        } else {
            // Empty line means new elf, so start a new sum, overwriting max if new val is larger
            if current_calories > max_calories {
                max_calories = current_calories;
            }
            current_calories = 0;
        }
    }
    if current_calories > max_calories {
        max_calories = current_calories;
    }

    return max_calories;
}
pub fn part_two(input: &String) -> u32 {
    let lines = input.lines();

    let mut current_calories: u32 = 0;
    let mut calories_list: Vec<u32> = Vec::new();

    for line in lines {
        if let Ok(x) = line.parse::<u32>() {
            current_calories += x;
        } else {
            // Empty line means new elf, so start a new sum, overwriting max if new val is larger
            calories_list.push(current_calories);
            current_calories = 0;
        }
    }
    calories_list.push(current_calories);

    calories_list.sort_by(|a, b| b.cmp(a));
    return  calories_list[..3].iter().fold(0, |a, b| a + b);
}
