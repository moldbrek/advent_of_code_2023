use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

struct PartNumber {
    id: i32,
    position: Position,
    num_digits: i32,
}

impl PartNumber {
    fn new(id: i32, position: Position, num_digits: i32) -> PartNumber {
        PartNumber { id, position, num_digits }
    }

    fn is_near(&self, symbol: &Symbol) -> bool {
        let x_near = symbol.position.x >= self.position.x - 1 && 
            symbol.position.x <= self.position.x + self.num_digits;
        let y_near = (self.position.y - symbol.position.y).abs() <= 1;
        x_near && y_near
    }
}

struct Symbol {
    _value: char,
    position: Position,
}

impl Symbol {
    fn new(_value: char, position: Position) -> Symbol {
        Symbol { _value, position }
    }    
}

fn parse_number(number_pending: &mut Option<(i32, usize)>, char_index: usize, line_index: usize, part_numbers: &mut Vec<PartNumber>) {
    if let Some((number, num_digits)) = number_pending.take() {
        part_numbers.push(PartNumber::new(
            number,
            Position::new(char_index as i32 - num_digits as i32, line_index as i32),
            num_digits as i32));
    }
}

fn parse_symbol(c: char, char_index: usize, line_index: usize, symbols: &mut Vec<Symbol>) {
    if c != '.' {
        symbols.push(Symbol::new(
            c,
            Position::new(char_index as i32, line_index as i32)));
    }
}

fn parse_digit(digit: Option<u32>, number_pending: &mut Option<(i32, usize)>) {
    if let Some(digit) = digit {
        let (number, num_digits) = number_pending.get_or_insert((0, 0));
        *number = *number * 10 + digit as i32;
        *num_digits += 1;
    }
}

pub fn run_day_3<P>(path: P) -> Result<(), Box<dyn Error>> 
where P: AsRef<Path> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    for (line_index, line) in lines.enumerate() {
        let line = line?;
        let mut number_pending: Option<(i32, usize)> = None;
        for (char_index, c) in line.char_indices() {
            let digit = c.to_digit(10);
            parse_digit(digit, &mut number_pending);
            if digit.is_none() {
                parse_number(&mut number_pending, char_index, line_index, &mut part_numbers);
                parse_symbol(c, char_index, line_index, &mut symbols);
            }
        }
        parse_number(&mut number_pending, line.len(), line_index, &mut part_numbers);
    }
    let valid_part_number_sum: i32 = part_numbers.iter()
        .filter(|part_number| symbols.iter().any(|symbol| part_number.is_near(symbol)))
        .map(|part_number| part_number.id)
        .sum();

    println!("Sum of valid part numbers: {}", valid_part_number_sum);

    let gear_ratio_sum: i32 = symbols.iter()
        .filter_map(|symbol| {
            let near_parts: Vec<_> = part_numbers.iter()
                .filter(|part_number| part_number.is_near(symbol))
                .collect();
            if near_parts.len() == 2 {
                Some(near_parts.iter().map(|part_number| part_number.id).product::<i32>())
            } else {
                None
            }
        })
        .sum();

    println!("Sum of gear ratios: {}", gear_ratio_sum);
    Ok(())
}