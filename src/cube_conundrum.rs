use std::error::Error;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

enum Color {
    Red,
    Green,
    Blue,
}

struct CubeSet {
    cubes: Vec<(usize, Color)>
}

impl CubeSet {
    fn new() -> CubeSet {
        CubeSet { cubes: Vec::new() }
    }

    fn from_string(input: &str) -> Result<CubeSet, Box<dyn std::error::Error>> {
        let mut cube_set = CubeSet::new();
        for cube_string in input.split(",") {
            let mut cube_info = cube_string.split_whitespace();
            let count = cube_info.next().unwrap().parse::<usize>().unwrap();
            let color = match cube_info.next().unwrap() {
                "red" => Color::Red,
                "green" => Color::Green,
                "blue" => Color::Blue,
                _ => panic!("Invalid color"),
            };
            cube_set.cubes.push((count, color));
        }
        Ok(cube_set)
    }
    
}

struct CubeGame {
    game_index: usize,
    cube_sets: Vec<CubeSet>,
}

impl CubeGame {
    fn new(game_index: usize) -> CubeGame {
        CubeGame {
            game_index,
            cube_sets: Vec::new(),
        }
    }
    
    fn from_string(input: &str) -> Result<CubeGame, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = input.split(":").collect();
        let game_index = parts[0][5..].trim().parse().unwrap();
        let mut game = CubeGame::new(game_index);
        for set_string in parts[1].split(";") {
            let cube_set = CubeSet::from_string(set_string)?;
            game.cube_sets.push(cube_set);
        }
        Ok(game)
    }

    fn is_valid(&self, max_red: usize, max_green: usize, max_blue: usize) -> bool {
        for cube_set in &self.cube_sets {
            let mut reds = 0;
            let mut greens = 0;
            let mut blues = 0;
            for (count, color) in &cube_set.cubes {
                match color {
                    Color::Red => reds += count,
                    Color::Green => greens += count,
                    Color::Blue => blues += count,
                }
            }
            if reds > max_red || greens > max_green || blues > max_blue {
                return false;
            }
        }
        return true;
    }

    fn cube_power(&self) -> usize {        
        let mut min_reds = 0;
        let mut min_greens = 0;
        let mut min_blues = 0;
        for cube_set in &self.cube_sets {
            for (count, color) in &cube_set.cubes {
                match color {
                    Color::Red =>  min_reds = min_reds.max(*count),
                    Color::Green => min_greens = min_greens.max(*count),
                    Color::Blue => min_blues = min_blues.max(*count),
                }
            }
        }        
        min_reds * min_greens * min_blues
    }
}

pub fn run_day_2<P>(path: P) -> Result<(), Box<dyn Error>> 
where P: AsRef<Path> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut games: Vec<CubeGame> = Vec::new();
    while let Some(line) = lines.next() {
        let line = line?;
        games.push(CubeGame::from_string(&line)?);
    }
    let mut sum_valid_indexes = 0;
    for game in &games {
        if game.is_valid(12, 13, 14) {
            sum_valid_indexes += game.game_index;
        }
    }
    println!("Sum of valid indexes: {}", sum_valid_indexes);
    let mut sum_cube_power = 0;
    for game in &games {
        sum_cube_power += game.cube_power();
    }
    println!("Sum of cube power: {}", sum_cube_power);
    Ok(())
}