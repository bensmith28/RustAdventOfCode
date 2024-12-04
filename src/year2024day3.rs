mod year2024day3 {
    use regex::Regex;

    fn part1(line: &str) -> usize {
        let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let mut result = 0;
        
        for m in pattern.find_iter(&line) {
            let captures = pattern.captures(&m.as_str()).unwrap();
            let a = captures[1].parse::<usize>().unwrap();
            let b = captures[2].parse::<usize>().unwrap();
            result += a * b;
        }
        
        result
    }
    
    fn part2(line: &str) -> usize {
        let pattern = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
        let mut result = 0;
        let mut enabled = true;
        for m in pattern.find_iter(&line) {
            match m.as_str() {
                "don't()" => {
                    enabled = false;
                }
                "do()" => {
                    enabled = true;
                }
                mul => {
                    if enabled {
                        let captures = pattern.captures(mul).unwrap();
                        let a = captures[1].parse::<usize>().unwrap();
                        let b = captures[2].parse::<usize>().unwrap();
                        result += a * b;
                    }
                    
                }
            }
        }
        
        result
    }
    
    #[cfg(test)]
    mod tests {
        mod part1 {
            use crate::read_string;
            use crate::year2024day3::year2024day3::part1;

            #[test]
            fn example() {
                let actual = part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
                assert_eq!(161, actual);
            }
            
            #[test]
            fn solution() {
                let input = read_string("input/2024-03-input.txt");
                let actual = part1(&input);
                assert_eq!(167650499, actual);
            }
        }
        
        mod part2 {
            use crate::read_string;
            use crate::year2024day3::year2024day3::part2;

            #[test]
            fn example() {
                let actual = part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
                assert_eq!(48, actual);    
            }

            #[test]
            fn solution() {
                let input = read_string("input/2024-03-input.txt");
                let actual = part2(&input);
                assert_eq!(95846796, actual);
            }
        }
    }
}