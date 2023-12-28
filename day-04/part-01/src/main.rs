use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("./files/input-4.txt")?;

    let lines: Vec<_> = io::BufReader::new(file).lines().collect::<Result<_, _>>()?;

    let mut sum = 0;

    for line in lines.iter() {
        sum += calculate_points(line);
    }

    println!("{}", sum);

    Ok(())
}

fn calculate_points(line: &str) -> usize {
    let delimiters = [':', '|'];
    let mut parts: Vec<&str> = vec![line];

    for &delimiter in &delimiters {
        parts = parts.iter().flat_map(|&part| part.split(delimiter)).collect();
    }

    let winning_cards: Vec<i32> = parts[1]
    .split(" ")
    .into_iter()
    .flat_map(|s| s.split_whitespace())
    .filter_map(|s| s.trim().parse().ok())
    .collect();

    let my_cards: Vec<i32> = parts[2]
    .split(" ")
    .into_iter()
    .flat_map(|s| s.split_whitespace())
    .filter_map(|s| s.trim().parse().ok())
    .collect();

    let cards_point: Vec<i32> = winning_cards
    .iter()
    .filter(|&card| my_cards.contains(card))
    .cloned()
    .collect();

    if cards_point.len() < 1 {
        return 0 
    }

    let two: usize = 2;

    two.pow((cards_point.len() - 1) as u32)
}