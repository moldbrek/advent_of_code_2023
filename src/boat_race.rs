use std::error::Error;
use std::result::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::path::Path;

fn parse_line_values(line: &str) -> Result<Vec<u64>, Box<dyn Error>> {
    line.split(":")
        .nth(1)
        .ok_or("Expected a ':' in the line")?
        .split_whitespace()
        .map(|s| s.parse::<u64>())
        .collect::<Result<Vec<u64>, ParseIntError>>()
        .map_err(|e| e.into())
}

fn parse_line_into_number(line: &str) -> Result<u64, Box<dyn Error>> {
    let number_str = line.split(":")
        .nth(1)
        .ok_or("Expected a ':' in the line")?
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    number_str.parse::<u64>().map_err(|e| e.into())
}

fn record_hold_interval(total_time: u64, record_distance: u64) -> Result<(u64, u64), Box<dyn Error>> {
    let t = total_time as f64;
    let d = record_distance as f64;
    let h_min = (t - (t * t - 4.0 * d).sqrt()) / 2.0;
    let h_max = (t + (t * t - 4.0 * d).sqrt()) / 2.0;
    if h_min.is_finite() && h_max.is_finite() {
        Ok((h_min.floor() as u64, h_max.ceil() as u64))
    } else {
        Err(From::from(format!("No solution for total_time: {}, record_distance: {}",
        total_time, record_distance)))
    }
} 

pub fn run_day_6<P>(path: P) -> Result<(), Box<dyn Error>> 
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let time_line = lines.next().ok_or("Expected a line")??;
    let times = parse_line_values(&time_line)?;

    let distance_line = lines.next().ok_or("Expected a line")??;
    let distances = parse_line_values(&distance_line)?;

    println!("Times: {:?}", times);
    println!("Distances: {:?}", distances);

    let num_win_hold_times: Result<Vec<u64>, _> = times.iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| {
            record_hold_interval(time, distance)
                .map(|(min, max)| max - min - 1)
        })
        .collect();

    match num_win_hold_times {
        Ok(values) => {
            let product = values.iter().product::<u64>();
            println!("Number of ways to beat record: {}", product);
        },
        Err(e) => {
            println!("Error: {}", e);
        }   
    }

    let time = parse_line_into_number(&time_line)?;
    let distance = parse_line_into_number(&distance_line)?;

    let (min, max) = record_hold_interval(time, distance)?;
    println!("Minimum number of holds to beat long record: {}", max - min - 1);

    Ok(())
}