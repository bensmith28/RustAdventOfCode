mod year2024day1 {
    use crate::read_lines;

    fn part1(filename: &str) -> usize {
        let mut list1: Vec<usize> = Vec::new();
        let mut list2: Vec<usize> = Vec::new();
        
        let mut iter = read_lines(filename);
        while let Some(Ok(line)) = iter.next() {
            let mut split = line.split_whitespace();
            list1.push(split.next().unwrap().parse::<usize>().unwrap());
            list2.push(split.next().unwrap().parse::<usize>().unwrap());
        }
        
        list1.sort();
        list2.sort();
        
        let mut iter1 = list1.iter();
        let mut iter2 = list2.iter();
        let mut result = 0;
        while let (Some(e1), Some(e2)) = (iter1.next(), iter2.next()) {
            result += if e1 > e2 { e1 - e2 } else { e2 - e1 };
        }
        
        result
    }
    
    fn part2(filename: &str) -> usize {
        let mut list1: Vec<usize> = Vec::new();
        let mut list2: Vec<usize> = Vec::new();

        let mut iter = read_lines(filename);
        while let Some(Ok(line)) = iter.next() {
            let mut split = line.split_whitespace();
            list1.push(split.next().unwrap().parse::<usize>().unwrap());
            list2.push(split.next().unwrap().parse::<usize>().unwrap());
        }
        
        let mut score = 0;
        
        for e1 in list1 {
            score += e1 * list2.iter().filter(|&&e2| e2 == e1).count();
        }
        
        score
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