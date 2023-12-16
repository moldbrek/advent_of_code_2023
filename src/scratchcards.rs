use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Clone)]
pub struct Card {
    _id: String,
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

impl Card {
    fn new(id: &str, winning_numbers: Vec<u32>, card_numbers: Vec<u32>) -> Card {
        Card {
            _id: id.to_string(),
            winning_numbers,
            card_numbers,
        }
    }

    fn from_string(input: &str) -> Result<Card, Box<dyn std::error::Error>> {
        let parts: Vec::<&str> = input.split(":").collect();
        let (id, numbers_string) = (parts[0], parts[1]);
        let parts: Vec::<&str> = numbers_string.split("|").collect();
        let winning_numbers = parts[0]
            .split_whitespace()
            .map(|x| x.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()?;
        let card_numbers = parts[1]
            .split_whitespace()
            .map(|x| x.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Card::new(id, winning_numbers, card_numbers))
    }

    fn get_num_matches(&self) -> u32 {
        let mut matches = 0;
        for number in &self.winning_numbers {
            if self.card_numbers.contains(number) {
                matches += 1;
            }
        }
        return matches;
    }

    fn get_score(&self) -> u32 {
        let matches = self.get_num_matches();
        if matches > 0
        {
            return 2u32.pow(matches - 1);
        }
        return 0;
    }
}

pub fn get_num_copies(
    cards: &Vec::<Card>, 
    index: usize, cache: 
    &mut HashMap<usize, u32>
) -> u32 {
    if let Some(&num_copies) = cache.get(&index) {
        num_copies
    } else {
        let num_matches = cards[index].get_num_matches();
        let mut num_copies = 1;
        for i in 1..(num_matches + 1) {
            num_copies += get_num_copies(cards, index + i as usize, cache);
        }
        cache.insert(index, num_copies);
        num_copies
    }
}

pub fn run_day_4<P>(path: P) -> Result<(), Box<dyn Error>> 
where P: AsRef<Path> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let cards = reader
        .lines()
        .map(|line| -> Result<Card, Box<dyn Error>>{
            let line = line?;
            let card = Card::from_string(&line)?;
            Ok(card)
        })
        .collect::<Result<Vec<Card>, Box<dyn Error>>>()?;
    
    let total = cards
        .iter()
        .map(|card| card.get_score())
        .sum::<u32>();
    
    println!("Total score: {}", total);

    let mut num_copies = 0;
    let mut cache: HashMap<usize, u32> = HashMap::new();
    for i in 0..cards.len() {
        num_copies += get_num_copies(&cards, i, &mut cache);
    }
    println!("Total copies: {}", num_copies);
    Ok(())
}