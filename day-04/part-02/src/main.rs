use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("./files/input-4.txt")?;

    let lines: Vec<_> = io::BufReader::new(file).lines().collect::<Result<_, _>>()?;

    let mut instances_played = 0;

    let mut instances_to_play: Vec<i32> = lines
    .iter()
    .enumerate()
    .map(|(i, _)| (i) as i32)
    .collect();



    loop {
        if instances_played >= instances_to_play.len() {
            // Exit the loop when all instances are played
            break;
        }
    
        let i = instances_to_play[instances_played];
    
        let next_instances_to_play = calculate_next_games_to_play(&lines[i as usize], i as usize);
        instances_to_play.extend(next_instances_to_play);
    
        println!("{}", instances_played);
        instances_played += 1;
    }

    println!("{}", instances_played);

    Ok(())
}

fn calculate_next_games_to_play(line: &str, line_index: usize) -> Vec<i32> {
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

    let next_instances_to_play: Vec<i32> = cards_point
    .iter()
    .enumerate()
    .map(|(i, _)| (line_index + i + 1) as i32)
    .collect();

    next_instances_to_play
}