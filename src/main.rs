use std::io;

fn main() {
    println!("\nWelcome to the Min-Max Normalization in Rust!\n");

    println!("Please enter a list of numbers separated by spaces:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    if numbers.is_empty() {
        println!("Error: No numbers entered");
        return;
    }

    let number_min = numbers.iter().min().unwrap();
    let number_max = numbers.iter().max().unwrap();

    let normalized_numbers: Vec<i32> = numbers
        .iter()
        .map(|x| (*x - number_min) / (number_max - number_min))
        .collect();

    println!("Numbers that your input: {:?}", numbers);
    println!("Numbers that are normalized: {:?}", normalized_numbers);
}
