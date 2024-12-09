mod year2023day12 {
    use std::sync::mpsc;
    use std::thread;
    use regex::Regex;
    use crate::read_lines;

    struct Row {
        notes: String,
        rules: Vec<usize>
    }

    impl Row {
        fn parse(line: &str) -> Self {
            let pattern = Regex::new(r"([.#?]+) ([\d,]+)").unwrap();
            let captures = pattern.captures(line).unwrap();
            let notes = captures[1].to_string();
            let rules = captures[2]
                .split(",")
                .map(|r| r.parse().unwrap())
                .collect();

            Self { notes, rules }
        }

        fn count_valid_arrangements(&self) -> usize {
            let mut count = 0;
            let count_unknown = self.notes.chars()
                .filter(|c| *c == '?' )
                .count();

            let conditions_for_index = |index: usize| -> Vec<String> {
                let mut conditions = Vec::new();
                for i in 0..count_unknown {
                    let c = match (index / 2usize.pow(i as u32)) % 2 {
                        0 => ".".to_string(),
                        1 => "#".to_string(),
                        _ => unreachable!()
                    };
                    conditions.push(c);
                }
                conditions
            };

            let mut pattern_string = r"^\.*".to_string();
            for (i, r) in self.rules.iter().enumerate() {
                pattern_string.push_str(format!("#{{{}}}\\.", r).as_str());
                if i < self.rules.len() - 1 {
                    pattern_string.push_str("+");
                } else {
                    pattern_string.push_str("*$");
                }
            }
            let pattern = Regex::new(&pattern_string).unwrap();

            for i in 0..2usize.pow(count_unknown as u32) {
                let add_ins = conditions_for_index(i);
                let mut notes = self.notes.clone();
                for j in 0..count_unknown {
                    notes = notes.replacen('?', add_ins[j].as_str(), 1);
                }
                if pattern.is_match(&notes) {
                    count += 1;
                }
            }

            count
        }
    }
    
    fn part1(filename: &str) -> usize {
        let mut lines = read_lines(filename);
        let mut count = 0;
        let (tx, rx) = mpsc::channel();
        
        while let Some(Ok(line)) = lines.next() {
            let tx1 = tx.clone();
            let _ = thread::spawn(move || {
                let row = Row::parse(line.as_str());
                tx1.send(row.count_valid_arrangements())
            });
        }
        drop(tx);
        
        while let Ok(v) = rx.recv() {
            count += v;
        }
        
        count
    }

    #[cfg(test)]
    mod tests {
        mod part1 {
            use crate::year2023day12::year2023day12::{part1, Row};

            #[test]
            fn single() {
                assert_eq!(1, Row::parse("???.### 1,1,3").count_valid_arrangements());
                assert_eq!(4, Row::parse(".??..??...?##. 1,1,3").count_valid_arrangements());
                assert_eq!(1, Row::parse("?#?#?#?#?#?#?#? 1,3,1,6").count_valid_arrangements());
                assert_eq!(1, Row::parse("????.#...#... 4,1,1").count_valid_arrangements());
                assert_eq!(4, Row::parse("????.######..#####. 1,6,5").count_valid_arrangements());
                assert_eq!(10, Row::parse("?###???????? 3,2,1").count_valid_arrangements());
            }
            
            #[test]
            fn example() {
                assert_eq!(21, part1("input/2023-12-e1.txt"));
            }

            #[test]
            fn solution() {
                assert_eq!(21, part1("input/2023-12-input.txt"));
            }
        }
    }
}