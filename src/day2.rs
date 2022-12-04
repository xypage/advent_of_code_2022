pub fn part_one(input: &String) -> i32 {
    let lines = input.lines();
    let mut score: i32 = 0;
    for line in lines {
        let chars = line.as_bytes();

        // 65 = A in ascii
        let theirs: i32 = chars[0] as i32 - 65;
        // 88 = X in ascii
        let yours: i32 = chars[2] as i32 - 88;

        // Score gets 1 for rock, 2 for paper, 3 for scissors, or 1 for A, 2 for B, 3 for C
        score += yours + 1;

        if yours == theirs {
            // Draw
            score += 3;
        }
        if (theirs + 1) % 3 == yours {
            // choice + 1 beats it, so if yours is theirs + 1 (with modulo to wrap around) you win
            score += 6;
        }
        // Else, lost, no score added
    }

    return score;
}

pub fn part_two(input: &String) -> i32 {
    let lines = input.lines();
    let mut score: i32 = 0;
    for line in lines {
        let chars = line.as_bytes();

        // 65 = A in ascii
        let theirs: i32 = chars[0] as i32 - 65;
        // 88 = X in ascii
        let outcome: i32 = chars[2] as i32 - 88;

        // Score gets 1 for rock, 2 for paper, 3 for scissors, or 1 for A, 2 for B, 3 for C
        // But we need to choose our piece based on whether we should win, draw, or lose
        // lose means we need theirs - 1, draw means same, win means theirs + 1

        // So we can subtract 1 from outcome giving us -1 for losing, 0 for draw, and 1 for winning
        // Add that + 3 to theirs, then modulo, that way you can't get -1 if they have rock
        // then we add 1 because rock is the 0th index but 1 point, same for the rest

        // Can combine the -1 to shift it and the + 3 for modulo fixing to get + 2
        score += (theirs + outcome + 2) % 3 + 1;

        // outcome is 0 for lost, 1 for draw, and 2 for win, so we can just multiply it by 3 to get
        // the score for that result
        score += outcome * 3;
    }

    return score;
}
