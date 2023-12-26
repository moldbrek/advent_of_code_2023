use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
enum Instruction {
    Left,
    Right
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn run_day_8<P>(path: P) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let instructions = lines
        .next().ok_or("Expected line")??
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Invalid instruction"),
        })
        .collect::<Vec<Instruction>>();

    let nodes: HashMap<String, (String, String)> = lines
        .filter_map(|line| line.ok())
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let key = line[0..3].to_string();
            let left = line[7..10].to_string();
            let right = line[12..15].to_string();
            (key, (left, right))
    }).collect();

    let start = nodes.get("AAA");
    let end = nodes.get("ZZZ");
    
    if let (Some(start), Some(end)) = (start, end) {
        let mut current = start;
        let mut count = 0;
    
        while current != end {
            let (left, right) = current;
            let next = match instructions[count % instructions.len()] {
                Instruction::Left => left,
                Instruction::Right => right
            };
            current = nodes.get(next).ok_or("Invalid node")?;
            count += 1;
        }
    
        println!("Count for AAA to ZZZ: {}", count);
    } else {
        println!("'AAA' or 'ZZZ' not found in nodes");
    }

    let start_nodes: HashSet<&str> = nodes
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(String::as_str)
        .collect();

    let end_nodes : HashSet<&str> = nodes
        .keys()
        .filter(|key| key.ends_with("Z"))
        .map(String::as_str)
        .collect();

    println!("Start nodes: {:?}", start_nodes);
    println!("End nodes: {:?}", end_nodes);

    let mut current_nodes: Vec<&str> = start_nodes.iter().cloned().collect();
    let mut counts: Vec<usize> = vec![0; start_nodes.len()];
    let mut end_counts: Vec<Option<usize>> = vec![None; start_nodes.len()];
    
    while end_counts.iter().any(|&count| count.is_none()) {
        let mut next_nodes: Vec<&str> = Vec::with_capacity(current_nodes.len());
        for ((i, node), count) in current_nodes.iter().enumerate().zip(&mut counts) {
            if let Some((left, right)) = nodes.get(*node) {
                let next = match instructions[*count % instructions.len()] {
                    Instruction::Left => left,
                    Instruction::Right => right
                };
                next_nodes.push(next);
                *count += 1;
                if end_nodes.contains(next.as_str()) && end_counts[i].is_none() {
                    end_counts[i] = Some(*count);
                }
            }
        }
        current_nodes = next_nodes;
    }
    
    let lcm = end_counts
        .iter()
        .filter_map(|&count| count)
        .reduce(lcm)
        .unwrap();
    
    println!("Count for all XXA to XXZ: {}", lcm);

    Ok(())
}