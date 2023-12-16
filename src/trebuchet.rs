use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn calculate_sum_of_first_last_digits<P>(path: P) -> Result<u32, Box<dyn Error>>
where P: AsRef<Path> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut sum = 0;
    while let Some(line) = lines.next() {
        let line = line?;
        let digits: Vec<u32> = line.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        if let (Some(&first), Some(&last)) = (digits.first(), digits.last()) {
            sum += first * 10 + last;
        }
    }
    Ok(sum)
}

fn calculate_sum_of_first_and_last_numbers<P>(path: P) -> Result<u32, Box<dyn Error>>
where P: AsRef<Path> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let literals: HashMap<&str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ].iter().cloned().collect();

    let mut sum = 0;
    while let Some(line) = lines.next() {
        let line = line?;
        let mut numbers: Vec<(usize, u32)> = Vec::new();
        for (index, _) in line.char_indices() {
            for (&literal, &value) in literals.iter() {
                if line[index..].starts_with(literal) {
                    numbers.push((index, value));
                    break;
                }
            }
        }

        numbers.sort_unstable_by_key(|&(index, _)| index);
        
        let first_value = numbers.first().map(|&(_, value)| value).unwrap_or(0);
        let last_value = numbers.last().map(|&(_, value)| value).unwrap_or(0);

        sum += first_value * 10 + last_value;
    }
    Ok(sum)
}

pub fn run_day_1<P>(path: P) -> Result<(), Box<dyn Error>> 
where P: AsRef<Path> {
    let sum1 = calculate_sum_of_first_last_digits(&path)?;
    println!("Day 1, part 1: {}", sum1);
    let sum2 = calculate_sum_of_first_and_last_numbers(&path)?;
    println!("Day 1, part 2: {}", sum2);
    Ok(())
}