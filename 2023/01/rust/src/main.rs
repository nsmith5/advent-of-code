fn asdigit(c: char) -> Option<isize> {
    match c {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None,
    }
}

fn firstdigit(line: String) -> Option<isize> {
    for c in line.chars() {
        if c.is_digit(10) {
            return asdigit(c);
        }
    }
    None
}

fn main() {
    if let Some(digit) = firstdigit(String::from("foo123")) {
        println!("{}", digit);
    }
}
