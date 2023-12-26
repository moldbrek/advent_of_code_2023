use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub trait CardValue {
    fn card_value(self) -> u8;
}

impl CardValue for char {
    fn card_value(self) -> u8 {
        match self {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Invalid card label"),
        }
    }
}

pub trait CardValueJoker {
    fn card_value_joker(self) -> u8;
}

impl CardValueJoker for char {
    fn card_value_joker(self) -> u8 {
        match self {
            'J' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'Q' => 11,
            'K' => 12,
            'A' => 13,
            _ => panic!("Invalid card label"),
        }
    }
}

#[derive(Debug)]
pub enum HandRank {
    FiveOfAKind(char),
    FourOfAKind(char),
    FullHouse(char, char),
    ThreeOfAKind(char),
    TwoPairs(char, char),
    OnePair(char),
    HighCard,
}

impl PartialOrd for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandRank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let rank = |hand_rank: &HandRank| {
            match hand_rank {
                HandRank::FiveOfAKind(_) => 7,
                HandRank::FourOfAKind(_) => 6,
                HandRank::FullHouse(_, _) => 5,
                HandRank::ThreeOfAKind(_) => 4,
                HandRank::TwoPairs(_, _) => 3,
                HandRank::OnePair(_) => 2,
                HandRank::HighCard => 1,
            }
        };

        rank(self).cmp(&rank(other))
    }
}

impl PartialEq for HandRank {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (HandRank::FiveOfAKind(_), HandRank::FiveOfAKind(_)) => true,
            (HandRank::FourOfAKind(_), HandRank::FourOfAKind(_)) => true,
            (HandRank::FullHouse(_, _), HandRank::FullHouse(_, _)) => true,
            (HandRank::ThreeOfAKind(_), HandRank::ThreeOfAKind(_)) => true,
            (HandRank::TwoPairs(_, _), HandRank::TwoPairs(_, _)) => true,
            (HandRank::OnePair(_), HandRank::OnePair(_)) => true,
            (HandRank::HighCard, HandRank::HighCard) => true,
            _ => false,
        }
    }
}

impl Eq for HandRank {}

pub struct Hand {
    cards: [char; 5],
    bid: u64,
    rank: HandRank
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.rank.cmp(&other.rank) {
            std::cmp::Ordering::Equal => {
                let self_card_values: Vec<u8> = self.cards.iter().map(|&card| card.card_value()).collect();
                let other_card_values: Vec<u8> = other.cards.iter().map(|&card| card.card_value()).collect();
                self_card_values.cmp(&other_card_values)
            },
            ordering => ordering,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.cards == other.cards
    }
}

impl Eq for Hand {}

impl Hand {
    pub fn from_string(hand_str: &str) -> Result<Hand, Box<dyn Error>> {
        let parts: Vec<&str> = hand_str.split_whitespace().collect();
        if parts.len() != 2 {
            return Err("Input string must have exactly two parts".into());
        }
        if parts[0].len() != 5 {
            return Err("Cards string must have exactly 5 characters".into());
        }
        let cards: [char; 5] = parts[0].chars().collect::<Vec<_>>()[..5].try_into()?;
        let bid: u64 = parts[1].parse()?;

        let mut card_counts = HashMap::new();
        for &card in &cards {
            *card_counts.entry(card).or_insert(0) += 1;
        }
        let rank = if let (true, Some(card)) = Hand::is_five_of_a_kind(&card_counts) {
            HandRank::FiveOfAKind(card)
        } else if let (true, Some(card)) = Hand::is_four_of_a_kind(&card_counts) {
            HandRank::FourOfAKind(card)
        } else if let (true, Some(three_of_a_kind), Some(two_of_a_kind)) = Hand::is_full_house(&card_counts) {
            HandRank::FullHouse(three_of_a_kind, two_of_a_kind)
        } else if let (true, Some(card)) = Hand::is_three_of_a_kind(&card_counts) {
            HandRank::ThreeOfAKind(card)
        } else if let (true, Some(pair1), Some(pair2)) = Hand::is_two_pairs(&card_counts) {
            HandRank::TwoPairs(pair1, pair2)
        } else if let (true, Some(card)) = Hand::is_one_pair(&card_counts) {
            HandRank::OnePair(card)
        } else {
            HandRank::HighCard
        };

        Ok(Hand { cards, bid, rank })
    }

    pub fn get_rank(&self) -> &HandRank {
        &self.rank
    }

    fn is_n_of_a_kind(card_counts: &HashMap<char, u32>, n: u32) -> (bool, Option<char>) {
        card_counts.iter()
            .find(|&(_, &count)| count >= n)
            .map(|(&card, _)| (true, Some(card)))
            .unwrap_or((false, None))
    }
    
    fn is_five_of_a_kind(card_counts: &HashMap<char, u32>) -> (bool, Option<char>) {
        Self::is_n_of_a_kind(card_counts, 5)
    }
    
    fn is_four_of_a_kind(card_counts: &HashMap<char, u32>) -> (bool, Option<char>) {
        Self::is_n_of_a_kind(card_counts, 4)
    }
    
    fn is_three_of_a_kind(card_counts: &HashMap<char, u32>) -> (bool, Option<char>) {
        Self::is_n_of_a_kind(card_counts, 3)
    }
    
    fn is_one_pair(card_counts: &HashMap<char, u32>) -> (bool, Option<char>) {
        Self::is_n_of_a_kind(card_counts, 2)
    }

    fn is_full_house(card_counts: &HashMap<char, u32>) -> (bool, Option<char>, Option<char>) {
        let mut three_of_a_kind = None;
        let mut two_of_a_kind = None;
        for (&card, &count) in card_counts.iter() {
            if count == 3 {
                three_of_a_kind = Some(card);
            } else if count == 2 {
                two_of_a_kind = Some(card);
            }
        }
        (three_of_a_kind.is_some() && two_of_a_kind.is_some(), three_of_a_kind, two_of_a_kind)
    }

    fn is_two_pairs(card_counts: &HashMap<char, u32>) -> (bool, Option<char>, Option<char>) {
        let mut pairs = card_counts.iter()
            .filter(|&(_, &count)| count == 2)
            .map(|(&card, _)| card)
            .collect::<Vec<_>>();
        pairs.sort_unstable();
        pairs.reverse();
        if pairs.len() >= 2 {
            (true, Some(pairs[0]), Some(pairs[1]))
        } else {
            (false, None, None)
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.cards.iter().collect::<String>(), self.bid)
    }
}

pub struct HandJoker {
    cards: [char; 5],
    bid: u64,
    rank: HandRank
}

impl PartialOrd for HandJoker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandJoker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.rank.cmp(&other.rank) {
            std::cmp::Ordering::Equal => {
                let self_card_values: Vec<u8> = self.cards.iter().map(|&card| card.card_value_joker()).collect();
                let other_card_values: Vec<u8> = other.cards.iter().map(|&card| card.card_value_joker()).collect();
                self_card_values.cmp(&other_card_values)
            },
            ordering => ordering,
        }
    }
}

impl PartialEq for HandJoker {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.cards == other.cards
    }
}

impl Eq for HandJoker {}

impl HandJoker {
    pub fn from_string(hand_str: &str) -> Result<HandJoker, Box<dyn Error>> {
        let parts: Vec<&str> = hand_str.split_whitespace().collect();
        if parts.len() != 2 {
            return Err("Input string must have exactly two parts".into());
        }
        if parts[0].len() != 5 {
            return Err("Cards string must have exactly 5 characters".into());
        }
        let cards: [char; 5] = parts[0].chars().collect::<Vec<_>>()[..5].try_into()?;
        let bid: u64 = parts[1].parse()?;

        let mut card_counts = HashMap::new();
        for &card in &cards {
            *card_counts.entry(card).or_insert(0) += 1;
        }
        let rank = if let (true, Some(card)) = HandJoker::is_five_of_a_kind(&card_counts) {
            HandRank::FiveOfAKind(card)
        } else if let (true, Some(card)) = HandJoker::is_four_of_a_kind(&card_counts) {
            HandRank::FourOfAKind(card)
        } else if let (true, Some(three_of_a_kind), Some(two_of_a_kind)) = HandJoker::is_full_house(&card_counts) {
            HandRank::FullHouse(three_of_a_kind, two_of_a_kind)
        } else if let (true, Some(card)) = HandJoker::is_three_of_a_kind(&card_counts) {
            HandRank::ThreeOfAKind(card)
        } else if let (true, Some(pair1), Some(pair2)) = HandJoker::is_two_pairs(&card_counts) {
            HandRank::TwoPairs(pair1, pair2)
        } else if let (true, Some(card)) = HandJoker::is_one_pair(&card_counts) {
            HandRank::OnePair(card)
        } else {
            HandRank::HighCard
        };

        Ok(HandJoker { cards, bid, rank })
    }

    pub fn get_rank(&self) -> &HandRank {
        &self.rank
    }

    fn prepare_counts(card_counts: &HashMap<char, u32>) -> (u32, Vec<(&char, &u32)>) {
        let joker_count = *card_counts.get(&'J').unwrap_or(&0);
        let mut counts: Vec<_> = card_counts.iter().collect();
        counts.sort_by(|a, b| b.1.cmp(a.1));
        (joker_count, counts)
    }

    fn is_n_of_a_kind(card_counts: &HashMap<char, u32>, n: u32) -> (bool, Option<char>) {
        let (remaining_jokers, counts) = Self::prepare_counts(card_counts);
        let remaining = n.saturating_sub(remaining_jokers);
    
        if remaining == 0 {
            return (true, Some('J'));
        }
    
        counts.iter()
            .filter(|&(&card, _)| card != 'J')
            .find(|&(_, &count)| count >= remaining)
            .map(|(&card, _)| (true, Some(card)))
            .unwrap_or((false, None))
    }

    fn is_five_of_a_kind(card_counts: &HashMap<char, u32>) -> (bool, Option<char>) {
        Self::is_n_of_a_kind(card_counts, 5)
    }

    fn is_four_of_a_kind(card_counts: &HashMap<char, u32>) -> (bool, Option<char>) {
        Self::is_n_of_a_kind(card_counts, 4)
    }

    fn is_three_of_a_kind(card_counts: &HashMap<char, u32>) -> (bool, Option<char>) {
        Self::is_n_of_a_kind(card_counts, 3)
    }

    fn is_one_pair(card_counts: &HashMap<char, u32>) -> (bool, Option<char>) {
        Self::is_n_of_a_kind(card_counts, 2)
    }

    fn is_full_house(card_counts: &HashMap<char, u32>) -> (bool, Option<char>, Option<char>) {
        let (mut remaining_jokers, counts) = Self::prepare_counts(card_counts);
    
        let mut three_of_a_kind = None;
        let mut two_of_a_kind = None;
    
        for (&card, &count) in counts.iter().filter(|&(&card, _)| card != 'J') {
            if three_of_a_kind.is_none() && count + remaining_jokers >= 3 {
                three_of_a_kind = Some(card);
                remaining_jokers = (count + remaining_jokers).saturating_sub(3);
            } else if two_of_a_kind.is_none() && count + remaining_jokers >= 2 {
                two_of_a_kind = Some(card);
                remaining_jokers = (count + remaining_jokers).saturating_sub(2);
            }
        }
    
        if three_of_a_kind.is_some() && two_of_a_kind.is_some() {
            (true, three_of_a_kind, two_of_a_kind)
        } else {
            (false, None, None)
        }
    }

    fn is_two_pairs(card_counts: &HashMap<char, u32>) -> (bool, Option<char>, Option<char>) {
        let (mut remaining_jokers, counts) = Self::prepare_counts(card_counts);
    
        let mut pairs = Vec::new();
    
        for (&card, &count) in counts.iter().filter(|&(&card, _)| card != 'J') {
            if count + remaining_jokers >= 2 {
                pairs.push(card);
                remaining_jokers = (count + remaining_jokers).saturating_sub(2);
            }
        }
    
        if pairs.len() >= 2 {
            (true, Some(pairs[0]), Some(pairs[1]))
        } else if pairs.len() == 1 && remaining_jokers >= 2 {
            (true, Some(pairs[0]), Some('J'))
        } else {
            (false, None, None)
        }
    }
}

pub fn run_day_7<P>(path: P) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut hands = lines
        .map(|line| Hand::from_string(&line.unwrap()))
        .collect::<Result<Vec<Hand>, Box<dyn Error>>>()?;

    hands.sort_unstable();

    let total_winnings: u64 = hands.iter()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index + 1) as u64)
        .sum();

    println!("Total winnings: {}", total_winnings);

    let mut hands_joker = hands.iter()
        .map(|hand| HandJoker::from_string(&hand.to_string()))
        .collect::<Result<Vec<HandJoker>, Box<dyn Error>>>()?;

    hands_joker.sort_unstable();

    let total_winnings_joker: u64 = hands_joker.iter()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index + 1) as u64)
        .sum();

    println!("Total winnings with joker: {}", total_winnings_joker);

    Ok(())
}