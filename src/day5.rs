use regex::Regex;

pub fn part_one(input: &String) -> String {
    // The rows showing the stacks are formatted as "[A] [B]   ", or "    [C] [D]" when nothing is in front
    // in other words, even when empty the width is fixed, and each column takes up 4 spaces, except the last one
    // thus to find how many columns there are just add 1 to the width and then divide by 4
    // Since I don't want to mess with my .lines() iterator, I can just find the index of the first new line
    // to know its width, then get col_size from that
    let col_count = (input.find('\n').unwrap() + 1) / 4;
    // Make a vector with col_count empty character vectors inside it
    let mut stacks = vec![vec![' '; 0]; col_count];

    let mut lines = input.lines();
    // Doing this with bytes just to easily address it
    let mut curr_line = lines.next().unwrap().as_bytes();
    // Below the stacks it looks like " 1   2   3 . . ." so the 2nd character will be '1' for sure,
    // we can check for that to know when we've reached the next "section" of the input
    while curr_line[1] != b'1' {
        // then we iterate through the columns
        for i in 0..col_count {
            // again, each one is 4 wide, with the actual character label in the middle so + 1
            let index = 1 + i * 4;
            // get the character
            let char = curr_line[index];
            // if it's ' ' then nothing needs to be added there
            if char != b' ' {
                // otherwise, throw it onto that stack, this'll make them upside down for now but we can just reverse after
                stacks[i].push(char::from(char));
            }
        }
        // get the next line
        curr_line = lines.next().unwrap().as_bytes();
    }

    // Now reverse all the vectors
    for v in stacks.iter_mut() {
        v.reverse();
    }

    // consume the empty line
    lines.next();

    // now we've got lines that look like "move n from x to y" where n is a count, and x/y are stack indices
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in lines {
        // get the captures
        let caps = re.captures(line).unwrap();
        // n is as is
        let n = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        // x and y are indices so we need to -1 to make it go from 1 indexed to 0 indexed
        let x = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let y = caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

        // then iterate n times
        for _ in 0..n {
            // grab the top off of x and move it to y
            let val = stacks[x].pop().unwrap();
            stacks[y].push(val);
        }
    }

    // then just build a string containing the top elements of every stack and return that
    let mut top = String::new();
    for stack in stacks {
        top.push(stack[stack.len() - 1]);
    }

    return top;
}

pub fn part_two(input: &String) -> String {
    // same as part_one till the iterator moving them from stack x to stack y
    // where the next comment is
    let col_count = (input.find('\n').unwrap() + 1) / 4;
    let mut stacks = vec![vec![' '; 0]; col_count];

    let mut lines = input.lines();
    let mut curr_line = lines.next().unwrap().as_bytes();
    while curr_line[1] != b'1' {
        for i in 0..col_count {
            let index = 1 + i * 4;
            let char = curr_line[index];
            if char != b' ' {
                stacks[i].push(char::from(char));
            }
        }
        curr_line = lines.next().unwrap().as_bytes();
    }

    for v in stacks.iter_mut() {
        v.reverse();
    }

    lines.next();

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in lines {
        let caps = re.captures(line).unwrap();
        let n = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let x = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let y = caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

        // Here we don't want to reverse the order, so instead of popping them off the bottom we want to
        // push them onto y in the same order that they're on x, still iterating the same number of times
        // but we should calculate the index outside, since we'll get the same index every time as it'll
        // shift the rest over to fill the spot where it used to be

        // we're grabbing n, so if we've got n = 2 and [1, 2, 3, 4, 5, 6] then we want 5 and 6, the elements
        // that'll be at index 4, in an array of length 6, in other words, we want len-n
        let index = stacks[x].len() - n;
        for _ in 0..n {
            // So now we'll use .remove which removes a specific index from the vec and returns whatever was there
            let val = stacks[x].remove(index);
            stacks[y].push(val);
        }
    }

    let mut top = String::new();
    for stack in stacks {
        top.push(stack[stack.len() - 1]);
    }

    return top;
}
