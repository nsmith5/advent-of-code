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

// For part 1, count up the total score of all cards
fn total_score(cards: &Vec<Card>) -> usize {
    cards.iter().map(|card| -> usize { card.score() }).sum()
}

// For part 2, count up the total cards won
fn total_cards(cards: &Vec<Card>) -> usize {
    let mut cards_won = Vec::new();
    cards_won.resize(cards.len(), 0);

    for (i, card) in cards.iter().enumerate() {
        let multiplier = cards_won[i] + 1;
        let matches = card.matches();
        for j in 0..matches {
            cards_won[i + j + 1] += multiplier
        }
    }

    // Need to return the total cards won + the orignals
    let won: usize = cards_won.iter().sum();
    let original: usize = cards.len();

    won + original
}

fn main() {
    let cards = parse_cards("input.txt").unwrap();

    // 21959
    println!("Part 1: {}", total_score(&cards));
    println!("Part 2: {}", total_cards(&cards));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_score() {
        let tests: Vec<(Card, usize)> = vec![
            (
                Card {
                    winners: HashSet::from([1, 2, 3]),
                    numbers: HashSet::from([1, 2, 3, 5, 6]),
                },
                4,
            ),
            (
                Card {
                    winners: HashSet::from([1, 2, 3]),
                    numbers: HashSet::from([4, 5, 4, 5, 6]),
                },
                0,
            ),
        ];

        tests.iter().for_each(|test| {
            let (card, expect) = test;
            assert_eq!(&card.score(), expect);
        });
    }

    #[test]
    fn test_card_matches() {
        let tests: Vec<(Card, usize)> = vec![
            (
                Card {
                    winners: HashSet::from([1, 2, 3]),
                    numbers: HashSet::from([1, 2, 3, 5, 6]),
                },
                3,
            ),
            (
                Card {
                    winners: HashSet::from([1, 2, 3]),
                    numbers: HashSet::from([4, 5, 4, 5, 6]),
                },
                0,
            ),
        ];

        tests.iter().for_each(|test| {
            let (card, expect) = test;
            assert_eq!(&card.matches(), expect);
        });
    }

    #[test]
    fn test_total_cards() {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        let cards = vec![
            Card {
                winners: [41, 48, 83, 86, 17].into(),
                numbers: [83, 86, 6, 31, 17, 9, 48, 53].into(),
            },
            Card {
                winners: [13, 32, 20, 16, 61].into(),
                numbers: [61, 30, 68, 82, 17, 32, 24, 19].into(),
            },
            Card {
                winners: [1, 21, 53, 59, 44].into(),
                numbers: [69, 82, 63, 72, 16, 21, 14, 1].into(),
            },
            Card {
                winners: [41, 92, 73, 84, 69].into(),
                numbers: [59, 84, 76, 51, 58, 5, 54, 83].into(),
            },
            Card {
                winners: [87, 83, 26, 28, 32].into(),
                numbers: [88, 30, 70, 12, 93, 22, 82, 36].into(),
            },
            Card {
                winners: [31, 18, 13, 56, 72].into(),
                numbers: [74, 77, 10, 23, 35, 67, 36, 11].into(),
            },
        ];

        let total = total_cards(&cards);
        assert_eq!(total, 30)
    }
}
