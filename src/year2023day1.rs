use std::fs::File;
use std::io::{BufRead, BufReader};

fn step1(file_name: &str) -> std::io::Result<usize> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut sum = 0usize;

    // Create an iterator over the lines
    for line in reader.lines() {
        let mut first: Option<usize> = None;
        let mut last: Option<usize> = None;
        for c in line.unwrap().chars() {
            if c.is_ascii_digit() {
                let n = c.to_digit(10).unwrap() as usize;
                first.get_or_insert(n);
                last = Some(n);
            }
        }
        sum += first.unwrap() * 10 + last.unwrap();
    }

    Ok(sum)
}

#[derive(Debug)]
enum ParseError {
    Empty,
    NoMatch
}

const NUMBER_WORDS: [(&str, usize); 10] = [
    ("zero", 0), ("one", 1), ("two", 2), ("three", 3), ("four", 4),
    ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
];

fn parse_value(s: &str) -> Result<usize, ParseError> {
    if s.is_empty() {
        return Err(ParseError::Empty);
    }

    let first_char = s.chars().next().unwrap();
    if first_char.is_ascii_digit() {
        return Ok(first_char.to_digit(10).unwrap() as usize);
    }

    for (word, value) in NUMBER_WORDS.iter() {
        if s.starts_with(word) {
            return Ok(*value);
        }
    }

    Err(ParseError::NoMatch)
}

fn step2(file_name: &str) -> std::io::Result<usize> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut sum = 0usize;

    // Create an iterator over the lines
    for line in reader.lines() {
        let mut first: Option<usize> = None;
        let mut last: Option<usize> = None;
        let mut s = line.unwrap();
        while !s.is_empty() {
            let v = parse_value(&s);
            match v {
                Ok(n) => {
                    if first == None {
                        first = Some(n);
                    }
                    last = Some(n);
                }
                Err(_) => {}
            }
            s.drain(..1);
        }
        sum += first.unwrap() * 10 + last.unwrap();
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;  // Brings step1 into scope

    #[test]
    fn test_step1_example() {
        let result = step1("input/2023-01-e1.txt").unwrap();
        assert_eq!(result, 142);
    }

    #[test]
    fn test_step1() {
        let result = step1("input/2023-01-i1.txt").unwrap();
        assert_eq!(result, 53651);
    }
    
    #[test]
    fn test_parse_value_digit() {
        let result = parse_value(&"1other").unwrap();
        assert_eq!(result, 1usize)
    }

    #[test]
    fn test_parse_value_word() {
        let result = parse_value(&"oneother").unwrap();
        assert_eq!(result, 1usize)
    }

    #[test]
    fn test_step2_example() {
        let result = step2("input/2023-01-e2.txt").unwrap();
        assert_eq!(result, 281);
    }

    #[test]
    fn test_step2() {
        let result = step2("input/2023-01-i1.txt").unwrap();
        assert_eq!(result, 53894);
    }
}