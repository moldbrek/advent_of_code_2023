use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::path::Path;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

pub struct MapEntry {
    destination_range: usize,
    source_range: usize,
    range_length: usize
}

impl MapEntry {
    pub fn new(destination_range: usize, source_range: usize, range_length: usize) -> MapEntry {
        MapEntry {
            destination_range,
            source_range,
            range_length
        }
    }

    pub fn filter_ranges(&self, ranges: &mut Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut transformed = Vec::new();
        let source_start = self.source_range;
        let source_end = self.source_range + self.range_length - 1;
        let range_diff = self.destination_range as i64 - self.source_range as i64;

        let mut untransformed = Vec::new();
        for element in ranges.drain(..) {
            let (start, length) = element;
            let end = start + length - 1;
            if start <= source_end && end >= source_start {
                let new_start = if start >= source_start {
                    start
                } else {
                    untransformed.push((start, source_start - start));
                    source_start
                };
                let new_end = if end <= source_end {
                    end
                } else {
                    untransformed.push((source_end + 1, end - source_end));
                    source_end
                };
                transformed.push(((new_start as i64 + range_diff) as usize, new_end - new_start + 1));
            }
            else {
                untransformed.push(element);
            }
        }
        ranges.append(&mut untransformed);
        transformed
    }
}

pub struct Map {
    destination_category: Category,
    entries: Vec<MapEntry>
}

impl Map {
    pub fn new(destination_category: Category) -> Map {
        Map {
            destination_category,
            entries: Vec::new()
        }
    }

    fn add_entry(&mut self, entry: MapEntry) {
        self.entries.push(entry);
    }

    fn lookup(&self, value: usize) -> usize {
        for entry in &self.entries {
            if value >= entry.source_range && value < entry.source_range + entry.range_length {
                return entry.destination_range + (value - entry.source_range);
            }
        }
        value
    }

    pub fn lookup_ranges(&self, ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
        let mut transformed = Vec::new();
        let mut untransformed: Vec::<(usize, usize)> = ranges.clone().to_vec();
        for entry in &self.entries {
            transformed.append(&mut entry.filter_ranges(&mut untransformed));
        }
        transformed.append(&mut untransformed);
        transformed
    }
}

pub struct FeedingAlmanac {
    production_maps: HashMap<Category, Map>
}

impl FeedingAlmanac {
    fn create_transitions() -> HashMap<&'static str, (Category, Category)> {
        [
            ("seed-to-soil map:", (Category::Seed, Category::Soil)),
            ("soil-to-fertilizer map:", (Category::Soil, Category::Fertilizer)),
            ("fertilizer-to-water map:", (Category::Fertilizer, Category::Water)),
            ("water-to-light map:", (Category::Water, Category::Light)),
            ("light-to-temperature map:", (Category::Light, Category::Temperature)),
            ("temperature-to-humidity map:", (Category::Temperature, Category::Humidity)),
            ("humidity-to-location map:", (Category::Humidity, Category::Location)),
        ].iter().cloned().collect()
    }

    fn parse_map_entry(line: &str) -> Result<MapEntry, ParseIntError> {
        let map_values = line
            .split_whitespace()
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<usize>, ParseIntError>>()?;
        let (destination_range, source_range, range_length) = 
            (map_values[0], map_values[1], map_values[2]);
        Ok(MapEntry::new(destination_range, source_range, range_length))
    }

    fn from_lines(lines: Vec<String>) -> Result<FeedingAlmanac, Box<dyn Error>> {
        let mut current_section: Option<&(Category, Category)> = None;
        let mut production_maps: HashMap<Category, Map> = HashMap::new();

        let transitions = Self::create_transitions();

        'lines: for line in lines {
            if line.is_empty() {
                current_section = None;
                continue 'lines;
            }

            if let Some(transition) = transitions.get(line.as_str()) {
                current_section = Some(transition);
                continue 'lines;
            }
            
            let map_entry = Self::parse_map_entry(&line)?;

            match production_maps.entry(current_section.ok_or("No current section")?.0.clone()) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().add_entry(map_entry);
                }
                Entry::Vacant(entry) => {
                    let mut map = Map::new(current_section.ok_or("No current section")?.1.clone());
                    map.add_entry(map_entry);
                    entry.insert(map);
                }
            }
        }

        Ok(FeedingAlmanac {
            production_maps
        })
    }

    fn get_location(&self, seed: usize) -> Result<usize, Box<dyn Error>> {
        let mut current_map = self
            .get_map(Category::Seed)
            .ok_or("No map for Seed")?;
        let mut value = current_map.lookup(seed);

        while current_map.destination_category != Category::Location {
            current_map = self
                .get_map(current_map.destination_category.clone())
                .ok_or(format!("No map for {:?}", current_map.destination_category))?;

            value = current_map.lookup(value);
        }
        Ok(value)
    }

    pub fn get_location_ranges(&self, seed_ranges: &[(usize, usize)]) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
        let mut current_map = self
            .get_map(Category::Seed)
            .ok_or("No map for Seed")?;
        let mut ranges = current_map.lookup_ranges(seed_ranges);

        while current_map.destination_category != Category::Location {
            current_map = self
                .get_map(current_map.destination_category.clone())
                .ok_or(format!("No map for {:?}", current_map.destination_category))?;

            ranges = current_map.lookup_ranges(&ranges);
        }
        Ok(ranges)
    }

    fn check_seeds(&self, seeds: &[usize]) -> Result<usize, Box<dyn Error>> {
        let mut location_min = usize::MAX;
        for seed in seeds {
            match self.get_location(*seed) {
                Ok(location) => {
                    location_min = if location < location_min {
                        location 
                    } else { 
                        location_min
                    };
                },
                Err(e) => {
                    println!("Failed to get location for seed {}: {}", seed, e);
                }
            }
        }
        Ok(location_min)
    }

    fn get_map(&self, category: Category) -> Option<&Map> {
        self.production_maps.get(&category)
    }

}

pub fn run_day_5<P>(path: P) -> Result<(), Box<dyn Error>> 
where P: AsRef<Path> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let seed_line = lines.next().ok_or("Expected a line")??;
    let seeds = seed_line
        .split(":")
        .nth(1)
        .ok_or("Expected a ':' in the line")?
        .split_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<usize>, ParseIntError>>()?;

    println!("seeds: {:?}", seeds);
    let almanac = FeedingAlmanac::from_lines(
        lines.map(|l| l.unwrap()).collect::<Vec<String>>())?;

    let location_min = almanac.check_seeds(&seeds)?;
    println!("minimum location for individual seeds: {}", location_min);

    let seed_ranges: Vec<(usize, usize)> = seeds
        .chunks(2)
        .filter_map(|chunk| {
            if chunk.len() == 2 {
                let start = chunk[0];
                let count = chunk[1];
                Some((start, count)) // Added closing parenthesis here
            } else {
                println!("Invalid chunk: {:?}", chunk);
                None
            }
        })
        .collect();

    let location_ranges = almanac.get_location_ranges(&seed_ranges)?;
    let first_items: Vec<usize> = location_ranges
        .iter()
        .map(|&(first, _)| first)
        .collect();

    let smallest_location = first_items.iter().min().unwrap();
    
    let seed_ranges_sum: usize = seed_ranges
        .iter()
        .map(|&(_, second)| second)
        .sum();

    let location_ranges_sum: usize = location_ranges
        .iter()
        .map(|&(_, second)| second)
        .sum();
    println!("seed_ranges_sum: {}, location_ranges_sum: {}", seed_ranges_sum, location_ranges_sum);

    println!("smallest location: {}", smallest_location);

    Ok(())
}
