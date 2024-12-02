mod year2024day2 {
    use crate::read_lines;
    
    fn parse_report(line: &str) -> Vec<usize> {
        line.split_whitespace()
            .map(|s| s.parse::<usize>().unwrap() )
            .collect()
    }

    fn is_safe(report: &Vec<usize>) -> bool {
        let increasing = report[1] > report[0];
        
        increasing && report.windows(2).all(|w| w[1] > w[0] && w[1]-w[0] >= 1 && w[1]-w[0] <= 3 ) ||
            report.windows(2).all(|w| w[0] > w[1] && w[0]-w[1] >= 1 && w[0]-w[1] <= 3)
    }
    
    fn part1(filename: &str) -> usize {
        let mut result = 0;
        for line in read_lines(filename) {
            if is_safe(&parse_report(line.unwrap().as_str())) { result += 1 }
        }
        result
    }
    
    fn part2(filename: &str) -> usize {
        let mut result = 0;
        for line in read_lines(filename) {
            let report = parse_report(line.unwrap().as_str());
            if is_safe(&report) {
                result += 1
            } else {
                for i in 0..report.len() {
                    let mut option = report.clone();
                    option.remove(i);
                    if is_safe(&option) {
                        result += 1;
                        break;
                    }
                }
            }
        }
        result
    }
    
    #[cfg(test)]
    mod tests {
        mod part1 {
            use crate::year2024day2::year2024day2::part1;

            #[test]
            fn example() {
                assert_eq!(2, part1("input/2024-02-e1.txt"));
            }
            
            #[test]
            fn solution() {
                assert_eq!(379, part1("input/2024-02-input.txt"));
            }
        }
        
        mod part2 {
            use crate::year2024day2::year2024day2::part2;

            #[test]
            fn example() {
                assert_eq!(4, part2("input/2024-02-e1.txt"));
            }

            #[test]
            fn solution() {
                assert_eq!(430, part2("input/2024-02-input.txt"));
            }
        }
    }
}