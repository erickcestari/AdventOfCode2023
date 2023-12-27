use std::fs::File;
use std::io::{self, BufRead, BufReader};

//12 red cubes, 13 green cubes, and 14 blue cubes
const REDCUBES: u32 = 12;
const GREENCUBES: u32 = 13;
const BLUECUBES: u32 = 14;

fn main() -> io::Result<()> {
    let file = File::open("./files/input-2.txt")?;

    let reader = BufReader::new(file);

    let mut sum_valid_game_ids = 0;

    for line in reader.lines() {
        match line {
            Ok(line_content) => {
                sum_valid_game_ids += check_valid_games(&line_content);
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }

    println!("{}", sum_valid_game_ids);
    Ok(())
}

fn check_valid_games(line: &str) -> u32 {
    let game_id = extract_game_id(line).unwrap();

    if let Some(game_data) = line.split(":").nth(1) {
        for part in game_data.split(";") {
            for cubes in part.split(",") {
                let max_cubes = get_number_of_cube_color(cubes);
                let number_cubes = extract_number_cubes(cubes).unwrap();
                println!("Cubes: {}", number_cubes);
                if max_cubes < number_cubes {
                    println!("Cubes: {}", cubes.trim());
                    return 0;
                }
            }
        }
    }

    game_id
}

fn get_number_of_cube_color(s: &str) -> u32 {
    match s {
        s if s.contains("red") => REDCUBES,
        s if s.contains("green") => GREENCUBES,
        s if s.contains("blue") => BLUECUBES,
        _ => 0,
    }
}

fn extract_game_id(input: &str) -> Option<u32> {
    if let Some(start_idx) = input.find("Game ") {
        if let Some(end_idx) = input[start_idx..].find(':') {
            let game_id_str = &input[start_idx + 5..start_idx + end_idx];
            if let Ok(game_id) = game_id_str.parse() {
                return Some(game_id);
            }
        }
    }

    None
}

fn extract_number_cubes(input: &str) -> Option<u32> {
    let input_trim = input.trim();

    let cube_number_str: String = input_trim.chars().take_while(|&c| c.is_digit(10)).collect();

    if let Ok(cube_number) = cube_number_str.parse() {
        return Some(cube_number);
    }

    None
}
