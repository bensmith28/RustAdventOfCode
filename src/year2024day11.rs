mod year2024day11 {

    fn blink_into(source: &Vec<usize>, destination: &mut Vec<usize>) {
        destination.clear();
        for &stone in source {
            if stone == 0 {
                destination.push(1);
            } else if stone.to_string().len() % 2 == 0 {
                let s = stone.to_string();
                let a = s[0..s.len() / 2].parse::<usize>().unwrap();
                let b = s[s.len() / 2..s.len()].parse::<usize>().unwrap();
                destination.push(a);
                destination.push(b);
            } else {
                destination.push(stone * 2024);
            }
        }
    }
    
    fn do_blinks(filename: &str, n: usize) -> usize {
        let input = std::fs::read_to_string(filename).unwrap()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut a = input;
        let mut b = Vec::new();
        let mut lastest_is_a = true;

        for i in 0..n {
            if i % 2 == 0 {
                blink_into(&a, &mut b);
                lastest_is_a = false;
            } else {
                blink_into(&b, &mut a);
                lastest_is_a = true;
            }
        }

        if lastest_is_a {
            a.len()
        } else {
            b.len()
        }
    }

    #[cfg(test)]
    mod tests {
        mod part1 {
            use crate::year2024day11::year2024day11::do_blinks;

            #[test]
            fn example1() {
                assert_eq!(55312, do_blinks("input/2024-11-e1.txt", 25));
            }

            #[test]
            fn solution() {
                assert_eq!(186996, do_blinks("input/2024-11-input.txt", 25));
            }
        }
        mod part2 {
            use crate::year2024day11::year2024day11::do_blinks;

            #[test]
            fn solution() {
                assert_eq!(186996, do_blinks("input/2024-11-input.txt", 75));
            }
        }
    }
}