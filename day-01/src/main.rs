use std::fs;

const INPUT_FILE: &str = "input/data.txt";

fn main() {
    let contents = fs::read_to_string(INPUT_FILE).unwrap();
    let input: Vec<_> = contents
        .lines()
        .map(|line| line.parse::<i32>().unwrap_or(0))
        .enumerate()
        .collect();

    part_one(input.clone());
    part_two(input.clone());
}

fn part_one(input: Vec<(usize, i32)>) {
    for (i, i_value) in &input {
        for (j, j_value) in &input {
            if i == j {
                continue;
            }

            if i_value + j_value == 2020 {
                let answer = i_value * j_value;
                println!("Answer 1: {}", answer);
                return ();
            }
        }
    }

    println!("Answer 1 not found!");
}

fn part_two(input: Vec<(usize, i32)>) {
    for (i, i_value) in &input {
        for (j, j_value) in &input {
            for (k, k_value) in &input {
                if any_are_the_same(i, j, k) {
                    continue;
                }

                if i_value + j_value + k_value == 2020 {
                    let answer = i_value * j_value * k_value;
                    println!("Answer 2: {}", answer);
                    return ();
                }
            }
        }
    }

    println!("Answer 2 not found!");
}

fn any_are_the_same(i: &usize, j: &usize, k: &usize) -> bool {
    i == j || i == k || j == k
}
