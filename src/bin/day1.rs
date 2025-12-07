use adv_code_2025::read_input_file_from;

fn main() {
    fn part1(input: &Vec<String>) -> i32 {
        let mut position = 50;
        let mut steps: Vec<i32> = Vec::new();
        let mut count = 0;

        for line in input {
            let (direction, amount) = line.split_at(1);
            let amount = amount.parse::<i32>().unwrap();

            match direction {
                "L" => position = (position - (amount + 100)) % 100,
                "R" => position = (position + amount) % 100,
                _ => eprintln!("direction should be L or R found: {direction}"),
            }
            if position == 0 {
                count += 1;
            }
            steps.push(position);
        }
        count
    }

    let input = read_input_file_from("./input/1.txt");
    let part1_solution = part1(&input);
    println!("part1 solution: {part1_solution}");

    fn part2(input: &Vec<String>) -> i32 {
        let mut position = 50;
        let mut steps: Vec<i32> = Vec::new();
        let mut count = 0;

        for line in input {
            let (direction, amount) = line.split_at(1);
            let amount = amount.parse::<i32>().unwrap();

            for _ in 0..amount {
                match direction {
                    "L" => position = ((position - 1) + 100) % 100,
                    "R" => position = (position + 1) % 100,
                    _ => eprintln!("direction should be L or R found: {direction}"),
                }
                if position == 0 {
                    count += 1;
                }
            }
            steps.push(position);
        }
        count
    }

    let example_input = Vec::from(
        [
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ]
        .map(String::from),
    );
    let part2_test = part2(&example_input);
    println!("example {part2_test}");
    let part2_solution = part2(&input);
    println!("part2 solution: {part2_solution}")
}
