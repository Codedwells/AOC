pub fn run_day1(is_example: bool) {

    let input = if is_example {
        include_str!("./example.txt").trim().lines().collect::<Vec<&str>>()
    } else {
        include_str!("./input.txt").trim().lines().collect::<Vec<&str>>()
    };

    println!("Day 1 of Advent of Code!");
    println!("{:?}", input);

}
