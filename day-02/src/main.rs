use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./files/input-2.txt")?;

    let reader = BufReader::new(file);

    let mut sum_power_games_cubes = 0;

    for line in reader.lines() {
        match line {
            Ok(line_content) => {
                sum_power_games_cubes += check_valid_games(&line_content);
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }

    println!("{}", sum_power_games_cubes);
    Ok(())
}

fn check_valid_games(line: &str) -> u32 {
    let mut min_cubes_red = 0;
    let mut min_cubes_blue = 0;
    let mut min_cubes_green = 0;

    if let Some(game_data) = line.split(":").nth(1) {
        for part in game_data.split(";") {
            for cubes in part.split(",") {
                let number_cubes = extract_number_cubes(cubes).unwrap();
                match cubes {
                    s if s.contains("red") && number_cubes > min_cubes_red => min_cubes_red = number_cubes,
                    s if s.contains("green") && number_cubes > min_cubes_green => min_cubes_green = number_cubes,
                    s if s.contains("blue") && number_cubes > min_cubes_blue => min_cubes_blue = number_cubes,
                    _ => {}
                }
            }
        }
    }

    min_cubes_red * min_cubes_blue * min_cubes_green
}

fn extract_number_cubes(input: &str) -> Option<u32> {
    let input_trim = input.trim();

    let cube_number_str: String = input_trim.chars().take_while(|&c| c.is_digit(10)).collect();

    if let Ok(cube_number) = cube_number_str.parse() {
        return Some(cube_number);
    }

    None
}
