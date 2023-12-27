use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./files/input-1.txt")?;

    let reader = BufReader::new(file);

    let mut input_numbers: Vec<i32> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line_content) => {
                let (first, last) = find_first_last_number(&line_content);
                input_numbers.push(join_numbers(first, last));
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }

    let sum: i32 = input_numbers.iter().sum();
    println!("{}", sum);
    Ok(())
}

fn string_to_i32(s: &str) -> Option<i32> {
    let lowercase_s = s.to_lowercase();

    match &lowercase_s {
        s if s.contains("one") => Some(1),
        s if s.contains("two") => Some(2),
        s if s.contains("three") => Some(3),
        s if s.contains("four") => Some(4),
        s if s.contains("five") => Some(5),
        s if s.contains("six") => Some(6),
        s if s.contains("seven") => Some(7),
        s if s.contains("eight") => Some(8),
        s if s.contains("nine") => Some(9),
        _ => None,
    }
}

fn find_first_last_number(line: &str) -> (i32, i32) {
    let mut first_number = None;
    let mut last_number = None;

    let mut start_index = 0;
    let mut end_index = line.len() - 1;

    while start_index <= end_index {
        if first_number.is_none() {
            if let Some(c) = line.chars().nth(start_index) {
                if c.is_digit(10) {
                    first_number = Some(c.to_digit(10).unwrap() as i32);
                } else {
                    let word = line[..=start_index].split_whitespace().next().unwrap_or("");
                    if let Some(value) = string_to_i32(word) {
                        first_number = Some(value);
                    }
                }
                start_index += 1;
            }
        }

        if last_number.is_none() {
            if let Some(c) = line.chars().nth(end_index) {
                if c.is_digit(10) {
                    last_number = Some(c.to_digit(10).unwrap() as i32);
                } else {
                    let word = line[end_index..].split_whitespace().last().unwrap_or("");
                    if let Some(value) = string_to_i32(word) {
                        last_number = Some(value);
                    }
                }
                end_index -= 1;
            }
        }

        if last_number.is_some() && first_number.is_some() {
            break;
        }
    }

    let mut first_number = first_number.unwrap_or_default();
    let mut last_number = last_number.unwrap_or_default();

    if first_number == 0 {
        first_number = last_number;
    }

    if last_number == 0 {
        last_number = first_number;
    }

    (first_number, last_number)
}

fn join_numbers(first_n: i32, last_n: i32) -> i32 {
    let first_str = first_n.to_string();
    let last_str = last_n.to_string();

    let concatenated_str = format!("{}{}", first_str, last_str);

    match concatenated_str.parse::<i32>() {
        Ok(result) => result,
        Err(_) => {
            println!("Error: Unable to parse concatenated string");
            0
        }
    }
}