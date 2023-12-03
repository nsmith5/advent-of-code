use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum ParsedChar {
    Digit(usize),
    Symbol,
    None,
}

fn parse_char(c: char) -> ParsedChar {
    match c {
        '.' => ParsedChar::None,
        '0' => ParsedChar::Digit(0),
        '1' => ParsedChar::Digit(1),
        '2' => ParsedChar::Digit(2),
        '3' => ParsedChar::Digit(3),
        '4' => ParsedChar::Digit(4),
        '5' => ParsedChar::Digit(5),
        '6' => ParsedChar::Digit(6),
        '7' => ParsedChar::Digit(7),
        '8' => ParsedChar::Digit(8),
        '9' => ParsedChar::Digit(9),
        _ => ParsedChar::Symbol,
    }
}

fn parse_game(schema_path: &str) -> Result<Vec<Vec<ParsedChar>>, Box<dyn Error>> {
    let mut g: Vec<Vec<ParsedChar>> = Vec::new();

    let file = File::open(schema_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        g.push(line?.chars().map(parse_char).collect())
    }

    Ok(g)
}

fn neighbours(i: usize, leni: usize, j: usize, lenj: usize) -> Vec<(usize, usize)> {
    let mut n: Vec<(usize, usize)> = Vec::new();

    // Corners first
    // Top left
    if i == 0 && j == 0 {
        n.push((i, j + 1));
        n.push((i + 1, j));
        n.push((i + 1, j + 1));
        return n;
    }

    // Top right
    if i == 0 && j == lenj - 1 {
        n.push((i, j - 1));
        n.push((i + 1, j));
        n.push((i + 1, j - 1));
        return n;
    }

    // Bottom left
    if i == leni - 1 && j == 0 {
        n.push((i, j + 1));
        n.push((i - 1, j));
        n.push((i - 1, j + 1));
        return n;
    }

    // Bottom right
    if i == leni - 1 && j == lenj - 1 {
        n.push((i, j - 1));
        n.push((i - 1, j - 1));
        n.push((i - 1, j));
        return n;
    }

    // Sides now
    // Top row
    if i == 0 {
        n.push((i, j - 1));
        n.push((i, j + 1));
        n.push((i + 1, j - 1));
        n.push((i + 1, j));
        n.push((i + 1, j + 1));
        return n;
    }

    // Bottom row
    if i == leni - 1 {
        n.push((i, j - 1));
        n.push((i, j + 1));
        n.push((i - 1, j - 1));
        n.push((i - 1, j));
        n.push((i - 1, j + 1));
        return n;
    }

    // Left column
    if j == 0 {
        n.push((i - 1, j));
        n.push((i + 1, j));
        n.push((i - 1, j + 1));
        n.push((i, j + 1));
        n.push((i + 1, j + 1));
        return n;
    }

    // Right column
    if j == lenj - 1 {
        n.push((i - 1, j));
        n.push((i + 1, j));
        n.push((i - 1, j - 1));
        n.push((i, j - 1));
        n.push((i + 1, j - 1));
        return n;
    }

    // Only remaining is out in the middle somewhere
    n.push((i - 1, j - 1));
    n.push((i - 1, j));
    n.push((i - 1, j + 1));
    n.push((i, j - 1));
    n.push((i, j + 1));
    n.push((i + 1, j - 1));
    n.push((i + 1, j));
    n.push((i + 1, j + 1));

    n
}

fn partsum(schema: Vec<Vec<ParsedChar>>) -> usize {
    let mut sum: usize = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for (i, line) in schema.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if let ParsedChar::Symbol = char {
                // Track visited neighbours
                for (n, m) in neighbours(i, schema.len(), j, line.len()) {
                    if let ParsedChar::Digit(d) = schema[n][m] {
                        if !visited.contains(&(n, m)) {
                            visited.insert((n, m));
                            let part_number = {
                                let mut deq: VecDeque<usize> = VecDeque::from([d]);
                                let mut pos = m;
                                // walk left
                                loop {
                                    if pos == 0 {
                                        break;
                                    } else {
                                        pos -= 1;
                                    }
                                    if let ParsedChar::Digit(d) = schema[n][pos] {
                                        deq.push_front(d);
                                        visited.insert((n, pos));
                                    } else {
                                        break;
                                    }
                                }

                                // Walk right
                                pos = m + 1;
                                while pos <= line.len() - 1 {
                                    if let ParsedChar::Digit(d) = schema[n][pos] {
                                        deq.push_back(d);
                                        visited.insert((n, pos));
                                        pos += 1;
                                    } else {
                                        break;
                                    }
                                }

                                let mut part_number: usize = 0;
                                for d in deq.iter() {
                                    part_number *= 10;
                                    part_number += d;
                                }
                                part_number
                            };

                            sum += part_number;
                        }
                    }
                }
            }
        }
    }

    sum
}
fn main() {
    let schema: Vec<Vec<ParsedChar>> = parse_game("input.txt").unwrap();

    // 556057
    println!("{}", partsum(schema))
}
