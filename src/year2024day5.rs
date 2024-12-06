mod year2024day5 {
    use crate::read_lines;

    type Order = Vec<usize>;
    struct Input {
        rules: Vec<Rule>,
        orders: Vec<Order>,
    }

    impl Input {
        fn new(filename: &str) -> Self {
            let mut lines = read_lines(filename);
            let mut rules = Vec::new();
            let mut orders = Vec::new();
            while let Some(Ok(line)) = lines.next() {
                if line.contains('|') {
                    rules.push(Rule::new(line.as_str()));
                }
                if line.contains(',') {
                    orders.push(line.split(',').map(|s| s.parse().unwrap()).collect());
                }
            }

            Input { rules, orders }
        }
    }

    struct Rule {
        left: usize,
        right: usize,
    }

    impl Rule {
        fn new(line: &str) -> Self {
            let split: Vec<&str> = line.split('|').collect();
            Rule {
                left: split[0].parse().unwrap(),
                right: split[1].parse().unwrap(),
            }
        }

        fn test(&self, order: &Order) -> bool {
            let l = order.iter().position(|&x| x == self.left);
            let r = order.iter().position(|&x| x == self.right);
            l.is_none() || r.is_none() || l.unwrap() < r.unwrap()
        }
    }

    fn part1(filename: &str) -> usize {
        let input = Input::new(filename);
        input.orders
            .iter()
            .filter(|&o| input.rules.iter().all(|r| r.test(o)))
            .map(|o| o[o.len() / 2])
            .sum()
    }
    
    fn part2(filename: &str) -> usize {
        let input = Input::new(filename);
        let fix_and_middle = |order: &Order| -> usize {
            let mut source = order.clone();
            let mut dest = Vec::new();
            while !source.is_empty() {
                let i = source.iter().position(|&p| {
                    !input.rules.iter().any(|r| r.left == p && source.contains(&r.right))
                }).unwrap();
                dest.push(source.remove(i));
            }
            dest[dest.len() / 2]
        };
        
        input.orders
            .iter()
            .filter(|o| input.rules.iter().any(|r| !r.test(o)))
            .map(|o| fix_and_middle(o))
            .sum()
    }

    #[cfg(test)]
    mod tests {
        mod parse {
            use crate::year2024day5::year2024day5::{Input, Rule};

            #[test]
            fn example() {
                let input = Input::new("input/2024-05-e1.txt");
                assert_eq!(input.rules.len(), 21);
                assert_eq!(input.orders.len(), 6);
            }

            #[test]
            fn rule1() {
                let rule = Rule::new("75|53");
                assert!(rule.test(&vec![75, 47, 61, 53, 29]));
                assert!(!rule.test(&vec![53, 75, 47, 61, 29]));
                assert!(rule.test(&vec![47, 61, 53, 29]));
                assert!(rule.test(&vec![75, 47, 61, 29]));
            }
        }
        
        mod part1 {
            use crate::year2024day5::year2024day5::part1;

            #[test]
            fn example() {
                let actual = part1("input/2024-05-e1.txt");
                assert_eq!(actual, 143);
            }
            
            #[test]
            fn solution() {
                let actual = part1("input/2024-05-input.txt");
                assert_eq!(actual, 4135);
            }
        }
        
        mod part2 {
            use crate::year2024day5::year2024day5::part2;

            #[test]
            fn example() {
                let actual = part2("input/2024-05-e1.txt");
                assert_eq!(actual, 123);
            }

            #[test]
            fn solution() {
                let actual = part2("input/2024-05-input.txt");
                assert_eq!(actual, 5285);
            }
        }
    }
}
