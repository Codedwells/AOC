/// --- AOC Day 1 :: Part 2 ---
pub fn run(is_example: bool) {
    //Read input file
    let input = if is_example {
        include_str!("./example2.txt").trim().lines()
    } else {
        include_str!("./input2.txt").trim().lines()
    };

    let parsed_input: u32 = input.map(process_line).sum::<u32>();

    println!("Part 2 : {:?}", parsed_input);
}

fn process_line(line: &str) -> u32 {

    let mut iter = (0..line.len()).filter_map(|index| {

        let reduced_line = &line[index..];

        let result = if reduced_line.starts_with("one") {
            Some(1)
        } else if reduced_line.starts_with("two") {
            Some(2)
        } else if reduced_line.starts_with("three") {
            Some(3)
        } else if reduced_line.starts_with("four") {
            Some(4)
        } else if reduced_line.starts_with("five") {
            Some(5)
        } else if reduced_line.starts_with("six") {
            Some(6)
        } else if reduced_line.starts_with("seven") {
            Some(7)
        } else if reduced_line.starts_with("eight") {
            Some(8)
        } else if reduced_line.starts_with("nine") {
            Some(9)
        } else {
            reduced_line
                .chars()
                .next()
                .unwrap()
                .to_digit(10)
        };

        result
    });

    let first = iter.next().expect("should be a number");

    // If there is a number after the first one, use it, otherwise use the first one
    match iter.last() {
        Some(num) => first * 10 + num,
        None => first * 10 + first,
    }
}
