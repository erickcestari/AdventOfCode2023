use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("./files/input-3.txt")?;

    let lines: Vec<_> = io::BufReader::new(file).lines().collect::<Result<_, _>>()?;

    let mut sum = 0;

    for (i, line) in lines.iter().enumerate() {
        sum += mult_numbers_adjacents_to_symbols(&lines, line, i);
    }

    println!("{}", sum);

    Ok(())
}

fn mult_numbers_adjacents_to_symbols(lines: &Vec<String>, line: &str, line_index: usize) -> u128{
    let mut value = 0;
    for (i, c) in line.chars().enumerate() {
        if c == '*' {
            value += mult_adjacent_numbers(lines, line_index, i);
        }
    }
    value
}

fn mult_adjacent_numbers(lines: &Vec<String>, line_index: usize, word_index: usize) -> u128 {
    let i_line = line_index as i32;

    let mut product: u128 = 1;

    let mut value_founds: Vec<u128> = Vec::new();

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let adj_i = i_line + i;
            let adj_j = word_index as i32 + j;

            if (0..lines.len() as i32).contains(&adj_i) && (0..lines[line_index].len() as i32).contains(&adj_j) {
                if let Some(c) = lines[adj_i as usize].chars().nth(adj_j as usize) {
                    if c.is_digit(10) {
                        let value = take_full_number(&lines[adj_i as usize], adj_j as usize) as u128;
                        
                        if !value_founds.contains(&value) {
                            product *= value;
                            value_founds.push(value);
                        }
                    }
                }
            }
        }
    }
    
    if value_founds.len() > 1 {
        return product
    }

    0
}

fn take_full_number(line: &String, i: usize) -> u32 {
    let mut number_str = String::new();

    number_str.push(line.chars().nth(i).unwrap());

    let mut front_index = i + 1;
    let mut back_index = i - 1;

    while front_index < line.len() && line[front_index..].chars().next().unwrap().is_digit(10) {
        let c = line.chars().nth(front_index).unwrap();
        number_str.push(c);
        front_index += 1;
    }

    while back_index > 0 && line[back_index..].chars().next().unwrap().is_digit(10) {
        let c = line.chars().nth(back_index).unwrap();
        number_str.insert(0, c);
        back_index -= 1;
    }

    if back_index == 0 && line.chars().nth(0).unwrap().is_digit(10) {
        let c = line.chars().nth(0).unwrap();
        number_str.insert(0, c);
    }


    if let Ok(parsed_number) = number_str.parse::<u32>() {
        parsed_number
    } else {
        0
    }
}