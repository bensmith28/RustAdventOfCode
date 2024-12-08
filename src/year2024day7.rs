mod year2024day7 {
    use std::sync::mpsc;
    use std::thread;
    use regex::Regex;
    use crate::read_lines;
    use crate::year2024day7::year2024day7::Operator::*;

    #[derive(Debug, PartialEq, Eq)]
    struct Equation {
        test_value: usize,
        operands: Vec<usize>,
    }

    #[derive(Clone, PartialEq, Eq)]
    enum Operator {
        PLUS, TIMES, CONCATENATE
    }

    impl Equation {
        fn parse(line: &str) -> Self {
            let captures = Regex::new(r"(\d+): ([\d ]+)").unwrap()
                .captures(line).unwrap();
            let test_value = captures[1].parse().unwrap();
            let operands = captures[2]
                .split_whitespace()
                .map(|val| val.parse().unwrap())
                .collect();
            Self {
                test_value,
                operands,
            }
        }

        fn is_valid(&self, how_many_operators: usize) -> bool {
            let operators_from_index = |index: usize| -> Vec<Operator> {
                let mut operators = Vec::new();
                for i in 0..self.operands.len() - 1 {
                    match (index / how_many_operators.pow(i as u32)) % how_many_operators {
                        0 => operators.push(PLUS),
                        1 => operators.push(TIMES),
                        2 => operators.push(CONCATENATE),
                        _ => unreachable!()
                    }
                }
                
                operators
            };
            for index in 0..how_many_operators.pow(self.operands.len() as u32 - 1) {
                let operators = operators_from_index(index);
                let mut value = *self.operands.first().unwrap();
                for i in 0..operators.len() {
                    match operators[i] {
                        PLUS => {
                            value += self.operands[i+1];
                        }
                        TIMES => {
                            value *= self.operands[i+1];
                        }
                        CONCATENATE => {
                            value = format!("{}{}", value, self.operands[i+1]).parse().unwrap();
                        }
                    }
                }
                if value == self.test_value { return true;}
            }

            false
        }
    }
    
    fn solve(filename: &str, how_many_operators: usize) -> usize {
        let mut result = 0;
        let mut lines = read_lines(filename);
        let (tx, rx) = mpsc::channel();
        while let Some(Ok(line)) = lines.next() {
            let tx1 = tx.clone();
            let _ = thread::spawn(move || {
                let eq = Equation::parse(line.as_str());
                if eq.is_valid(how_many_operators) {
                    tx1.send(eq.test_value).unwrap();
                }
            });
        }
        drop(tx);
        while let Ok(r) = rx.recv() {
            result += r;
        }
        result
    }

    #[cfg(test)]
    mod tests {
        mod parse {
            use crate::year2024day7::year2024day7::Equation;

            #[test]
            fn parse_line() {
                let equation = Equation::parse("190: 10 19");
                assert_eq!(
                    Equation {
                        test_value: 190,
                        operands: vec![10, 19]
                    },
                    equation
                );
            }
        }
        
        mod part1 {
            use crate::year2024day7::year2024day7::{solve, Equation};

            #[test]
            fn single() {
                assert!(Equation::parse("190: 10 19").is_valid(2));
                assert!(Equation::parse("3267: 81 40 27").is_valid(2));
                assert!(Equation::parse("292: 11 6 16 20").is_valid(2));
                assert!(!Equation::parse("83: 17 5").is_valid(2));
            }
            
            #[test]
            fn example() {
                assert_eq!(3749, solve("input/2024-07-e1.txt", 2));
            }
            
            #[test]
            fn solution() {
                assert_eq!(3245122495150, solve("input/2024-07-input.txt", 2));
            }
        }
        
        mod part2 {
            use crate::year2024day7::year2024day7::{solve, Equation};
            
            #[test]
            fn singles() {
                assert!(Equation::parse("156: 15 6").is_valid(3));
                assert!(Equation::parse("7290: 6 8 6 15").is_valid(3));
                assert!(Equation::parse("192: 17 8 14").is_valid(3));
            }

            #[test]
            fn example() {
                assert_eq!(11387, solve("input/2024-07-e1.txt", 3));
            }

            #[test]
            fn solution() {
                assert_eq!(105517128211543, solve("input/2024-07-input.txt", 3));
            }
        }
    }
}
