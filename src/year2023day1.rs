use std::fs::File;
use std::io::{BufRead, BufReader};

fn step1(file_name: String) -> std::io::Result<usize> {
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
                if first == None {
                    first = Some(n);
                }
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

fn parse_value(s: &String) -> Result<usize, ParseError> {
    if s.is_empty() {
        return Err(ParseError::Empty)
    }
    if s.chars().next().unwrap().is_ascii_digit() {
        return Ok(s.chars().next().unwrap().to_digit(10).unwrap() as usize);
    }
    if s.starts_with("one") {
        return Ok(1);
    }
    if s.starts_with("two") {
        return Ok(2);
    }
    if s.starts_with("three") {
        return Ok(3);
    }
    if s.starts_with("four") {
        return Ok(4);
    }
    if s.starts_with("five") {
        return Ok(5);
    }
    if s.starts_with("six") {
        return Ok(6);
    }
    if s.starts_with("seven") {
        return Ok(7);
    }
    if s.starts_with("eight") {
        return Ok(8);
    }
    if s.starts_with("nine") {
        return Ok(9);
    }
    if s.starts_with("zero") { 
        return Ok(0);
    }
    Err(ParseError::NoMatch)
}

fn step2(file_name: String) -> std::io::Result<usize> {
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
        let result = step1("input/2023-01-e1.txt".to_string()).unwrap();
        assert_eq!(result, 142);
    }

    #[test]
    fn test_step1() {
        let result = step1("input/2023-01-i1.txt".to_string()).unwrap();
        assert_eq!(result, 53651);
    }
    
    #[test]
    fn test_parse_value_digit() {
        let result = parse_value(&"1other".to_string()).unwrap();
        assert_eq!(result, 1usize)
    }

    #[test]
    fn test_parse_value_word() {
        let result = parse_value(&"oneother".to_string()).unwrap();
        assert_eq!(result, 1usize)
    }

    #[test]
    fn test_step2_example() {
        let result = step2("input/2023-01-e2.txt".to_string()).unwrap();
        assert_eq!(result, 281);
    }

    #[test]
    fn test_step2() {
        let result = step2("input/2023-01-i1.txt".to_string()).unwrap();
        assert_eq!(result, 53894);
    }
}