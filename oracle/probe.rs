/// An ISBN type
#[derive(PartialEq, Eq)]
enum IsbnType {
    Isbn10,
    Isbn13,
}

/// Checks if an 'X' is valid at the given position for the given ISBN type
#[allow(non_snake_case)]
fn is_X_valid(position: &usize, isbn_type: &IsbnType) -> bool {
    (isbn_type == &IsbnType::Isbn10 && position == &9)
        || (isbn_type == &IsbnType::Isbn13 && position == &12)
}

/// Checks if a '-' is valid at the given position for the given ISBN type
fn is_dash_valid(position: &usize, isbn_type: &IsbnType) -> bool {
    isbn_type == &IsbnType::Isbn13 && (position == &1 || position == &5 || position == &11)
}

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn_type = match isbn.len() {
        10 => IsbnType::Isbn10,
        13 => IsbnType::Isbn13,
        _ => return false,
    };

    let mut checksum = 0;
    let mut coefficient = 10;
    for (position, c) in isbn.char_indices() {
        let digit_value = match c {
            '0'..='9' => c.to_digit(10).unwrap(),
            'X' if is_X_valid(&position, &isbn_type) => 10,
            '-' if is_dash_valid(&position, &isbn_type) => continue,
            _ => return false,
        };

        checksum += coefficient * digit_value;
        coefficient -= 1;
    }

    checksum % 11 == 0
}


fn main() {
    let inputs: Vec<&str> = vec!["3-598-21508-8", "3-598-21508-9", "3-598-21507-X", "3-598-21507-A", "3-598-P1581-X", "3-598-2X507-9", "3-598-21508-96", "3598215088", "359821507X", "359821507", "00", "3-598-21507", "3-598-21515-X", "134456729", "3132P34035", "3598P215088"];
    let mut out: Vec<String> = Vec::new();
    for &x in inputs.iter() {
        out.push(if is_valid_isbn(x) { "true".to_string() } else { "false".to_string() });
    }
    println!("{{\"out\": [{}]}}", out.join(","));
}
