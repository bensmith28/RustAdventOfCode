mod year2023day9 {
    use crate::read_lines;

    struct Sequence {
        levels: Vec<Vec<isize>>
    }
    
    impl Sequence {
        fn new(line: &str) -> Self {
            let mut levels: Vec<Vec<isize>> = Vec::new();

            levels.push(line.split_whitespace().map(|s| s.parse().unwrap()).collect());
            
            loop {
                let last_level = levels.last().unwrap();
                if last_level.iter().all ( |&x| x == 0 ) {
                    break;
                }
                
                let mut next_level = Vec::new();
                for w in last_level.windows(2) {
                    next_level.push(w[1] - w[0]);
                }
                levels.push(next_level);
            }
            
            Sequence { levels }
        }
        
        fn predict(&self) -> isize {
            let mut prediction = 0;
            for l in self.levels.iter().rev() {
                prediction = prediction + l.last().unwrap();
            }
            
            prediction
        }
        
        fn predict_left(&self) -> isize {
            let mut prediction = 0;
            for l in self.levels.iter().rev() {
                prediction = l.first().unwrap() - prediction;
            }

            prediction
        }
    }
    
    fn part1(filename: &str) -> isize {
        let mut lines = read_lines(filename);
        let mut sum = 0;
        while let Some(Ok(line)) = lines.next() {
            sum += Sequence::new(line.as_str()).predict();
        }
        
        sum
    }

    fn part2(filename: &str) -> isize {
        let mut lines = read_lines(filename);
        let mut sum = 0;
        while let Some(Ok(line)) = lines.next() {
            sum += Sequence::new(line.as_str()).predict_left();
        }

        sum
    }
    
    #[cfg(test)]
    mod tests {
        mod part1 {
            use crate::year2023day9::year2023day9::{part1, Sequence};

            #[test]
            fn parse_example1() {
                let actual = Sequence::new("0 3 6 9 12 15").levels.len();
                assert_eq!(actual, 3);
            }
            
            #[test]
            fn predict_example1() {
                assert_eq!(18, Sequence::new("0 3 6 9 12 15").predict());
                assert_eq!(28, Sequence::new("1 3 6 10 15 21").predict());
                assert_eq!(68, Sequence::new("10 13 16 21 30 45").predict());
            }
            
            #[test]
            fn example1() {
                assert_eq!(114, part1("input/2023-09-e1.txt"));
            }

            #[test]
            fn solution() {
                assert_eq!(1584748274, part1("input/2023-09-input.txt"));
            }
        }
        
        mod part2 {
            use crate::year2023day9::year2023day9::{part2, Sequence};

            #[test]
            fn example_line() {
                let actual = Sequence::new("10  13  16  21  30  45");
                assert_eq!(5, actual.predict_left());
            }
            
            #[test]
            fn example() {
                let actual = part2("input/2023-09-e1.txt");
                assert_eq!(2, actual);
            }

            #[test]
            fn solution() {
                let actual = part2("input/2023-09-input.txt");
                assert_eq!(1026, actual);
            }
        }
    }
}