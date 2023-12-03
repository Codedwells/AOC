/// --- AOC Day 1 :: Part 1 ---
pub fn run(is_example: bool) {
    //Read input file
    let input = if is_example {
        include_str!("./example1.txt").trim().lines()
    } else {
        include_str!("./input1.txt").trim().lines()
    };

    let parsed_input = input.map(|line| {
        let mut iter = line.chars();

        let first_num = iter
            .find_map(|char| char.to_digit(10))
            .expect("Charater should be a number!!");

        let last_num = iter
            .rfind(|char| char.is_ascii_digit())
            .map(|x| x.to_digit(10).expect("Charater should be a number!!"))
            // Return the last number if it exists, otherwise return the first number
            .unwrap_or(first_num);

        first_num * 10 + last_num
    }).sum::<u32>();

    println!("Day 1 of Advent of Code!");
    println!("Part 1 : {:?}", parsed_input);
}
