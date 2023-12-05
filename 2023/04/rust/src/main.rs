use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Card {
    winners: HashSet<usize>,
    numbers: HashSet<usize>,
}

impl Card {
    fn score(&self) -> usize {
        let mut sum: usize = 0;
        for num in self.numbers.iter() {
            if self.winners.contains(&num) {
                if sum == 0 {
                    sum = 1;
                } else {
                    sum *= 2;
                }
            }
        }

        sum
    }

    fn matches(&self) -> usize {
        self.numbers
            .iter()
            .map(|d| -> usize {
                if self.winners.contains(&d) {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
}

fn parse_cards(path: &str) -> Result<Vec<Card>, Box<dyn Error>> {
    let mut cards = Vec::new();

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let suffix = line.split(":").last().unwrap();
        let parts: Vec<&str> = suffix.split("|").collect();

        let winners: HashSet<usize> = parts[0]
            .trim()
            .split_whitespace()
            .map(|s| -> usize { s.parse().unwrap() })
            .collect();

        let numbers: HashSet<usize> = parts[1]
            .trim()
            .split_whitespace()
            .map(|s| -> usize { s.parse().unwrap() })
            .collect();

        cards.push(Card { winners, numbers });
    }
    Ok(cards)
}

fn main() {
    let cards = parse_cards("input.txt").unwrap();

    let sum: usize = cards.iter().map(|card| -> usize { card.score() }).sum();

    // 21959
    println!("Part 1: {}", sum);

    let mut cards_won = Vec::new();
    cards_won.resize(cards.len(), 0);

    for (i, card) in cards.iter().enumerate() {
        let multiplier = cards_won[i] + 1;
        let matches = card.matches();
        for j in 0..matches {
            cards_won[i + j + 1] += multiplier
        }
    }

    let foo: Vec<usize> = (1..1).collect();

    println!("foo: {:?}", foo);

    let total: usize = cards_won.iter().sum();

    println!("Part 2: {}", total);
}
