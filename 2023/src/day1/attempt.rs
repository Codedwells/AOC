pub fn run_day1(is_example: bool) {
    let input = if is_example {
        include_str!("./example.txt")
            .trim()
            .lines()
            .collect::<Vec<&str>>()
    } else {
        include_str!("./input.txt")
            .trim()
            .lines()
            .collect::<Vec<&str>>()
    };

    for line in &input {
        let num = line
            .split("")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x| x.parse::<u32>().expect("Not a number"));

        println!("{:?}", num.collect::<Vec<u32>>());
    }

    println!("Day 1 of Advent of Code!");
    println!("{:?}", input);
}
