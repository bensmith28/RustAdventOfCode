mod year2024day1 {
    use crate::read_lines;

    fn parse_lists(filename: &str) -> (Vec<usize>, Vec<usize>) {
        read_lines(filename)
            .map(|r| {
                let line = r.unwrap();
                let mut split = line.split_whitespace();
                (split.next().unwrap().parse::<usize>().unwrap(), 
                 split.next().unwrap().parse::<usize>().unwrap())
            }).unzip()
    }
    
    fn part1(filename: &str) -> usize {
        let (mut list1, mut list2) = parse_lists(filename);
        list1.sort();
        list2.sort();
        
        list1.iter().zip(list2.iter())
            .map(|(&a, &b)| a.abs_diff(b))
            .sum()
    }

    fn part2(filename: &str) -> usize {
        let (list1, list2) = parse_lists(filename);
        
        list1.iter().fold(0, |acc, &e1| {
            acc + e1 * list2.iter().filter(|&&e2| e1 == e2).count()
        })
    }
    
    #[cfg(test)]
    mod tests {
        mod part1 {
            use crate::year2024day1::year2024day1::part1;

            #[test]
            fn example() {
                assert_eq!(11, part1("input/2024-01-e1.txt"));
            }

            #[test]
            fn solution() {
                assert_eq!(2378066, part1("input/2024-01-input.txt"));
            }
        }
        
        mod part2 {
            use crate::year2024day1::year2024day1::part2;

            #[test]
            fn example() {
                assert_eq!(31, part2("input/2024-01-e1.txt"));
            }

            #[test]
            fn solution() {
                assert_eq!(18934359, part2("input/2024-01-input.txt"));
            }
        }
    }
}