// Purpose: Main entry point for the Advent of Code 2023 Rust solutions.
use std::error::Error;
pub mod trebuchet;
pub mod cube_conundrum;
pub mod gear_ratios;
pub mod scratchcards;
pub mod food_production;
pub mod boat_race;
pub mod camel_cards;
pub mod haunted_wasteland;

pub(crate) fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let day = args[1].parse().unwrap();
    let path = &args[2];
    match day {
        1 => return trebuchet::run_day_1(path),
        2 => return cube_conundrum::run_day_2(path),
        3 => return gear_ratios::run_day_3(path),
        4 => return scratchcards::run_day_4(path),
        5 => return food_production::run_day_5(path),
        6 => return boat_race::run_day_6(path),
        7 => return camel_cards::run_day_7(path),
        8 => return haunted_wasteland::run_day_8(path), 
        _ => return Err(From::from(format!("Day {} not implemented", day))),
    };
}

