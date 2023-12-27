use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("./files/input-3.txt")?;

    let lines: Vec<_> = io::BufReader::new(file).lines().collect::<Result<_, _>>()?;

    let mut sum = 0;

    for (i, line) in lines.iter().enumerate() {
        sum += sum_numbers_adjacents_to_symbols(&lines, line, i);
    }

    println!("{}", sum);

    Ok(())
}

fn sum_numbers_adjacents_to_symbols(lines: &Vec<String>, line: &str, line_index: usize) -> u32 {
    let mut values = Vec::new();
    let mut current_number = String::new();
    let mut is_adjacent = false;

    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            current_number.push(c);
            if !is_adjacent {
                is_adjacent = have_adjacent_symbol(lines, line_index,i );
            }
        } else if !current_number.is_empty() {
            if let Ok(number) = current_number.parse::<u32>() {
                if is_adjacent {
                    values.push(number);
                    is_adjacent = false;
                }
            }

            current_number.clear();
        }
    }

    if let Ok(number) = current_number.parse::<u32>() {
        if is_adjacent {
            values.push(number);
        }
    }
    
    values.iter().sum()
}

fn have_adjacent_symbol(lines: &Vec<String>, line_index: usize, word_index: usize) -> bool {
    let i_line = line_index as i32;

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let adj_i = i_line + i;
            let adj_j = word_index as i32 + j;

            if (0..lines.len() as i32).contains(&adj_i) && (0..lines[line_index].len() as i32).contains(&adj_j) {
                let adjacent_symbol = lines[adj_i as usize].chars().nth(adj_j as usize);
                
                if let Some(s) = adjacent_symbol {
                    if !s.is_digit(10) && s != '.' {
                        return true;
                    }
                }
            }
        }
    }

    false
}