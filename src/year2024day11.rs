use std::collections::HashMap;

#[derive(Clone)]
struct Stone {
    value: usize,
    count: usize,
}

impl Stone {
    fn new(value: usize) -> Self {
        Stone { value, count: 1 }
    }

    fn with_value(&self, value: usize) -> Self {
        let mut clone = self.clone();
        clone.value = value;
        clone
    }
}

fn do_blinks(filename: &str, n: usize) -> usize {
    let input = std::fs::read_to_string(filename)
        .unwrap()
        .split_whitespace()
        .map(|s| {
            let value = s.parse::<usize>().unwrap();
            Stone::new(value)
        })
        .collect::<Vec<Stone>>();

    let mut stones = HashMap::new();
    for stone in input {
        stones.insert(stone.value, stone);
    }

    for _ in 0..n {
        let mut temp = Vec::new();
        for stone in stones.values() {
            if stone.value == 0 {
                temp.push(stone.with_value(1));
            } else if stone.value.to_string().len() % 2 == 0 {
                let s = stone.value.to_string();
                let a = s[..s.len() / 2].parse::<usize>().unwrap();
                let b = s[s.len() / 2..].parse::<usize>().unwrap();
                temp.push(stone.with_value(a));
                temp.push(stone.with_value(b));
            } else {
                temp.push(stone.with_value(stone.value * 2024));
            }
        }
        stones.clear();
        for stone in temp {
            stones
                .entry(stone.value)
                .and_modify(|ex| ex.count += stone.count)
                .or_insert(stone);
        }
    }

    stones.values().map(|s| s.count).sum()
}

#[cfg(test)]
mod tests {
    mod part1 {
        use crate::year2024day11::do_blinks;

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
        use crate::year2024day11::do_blinks;

        #[test]
        fn example() {
            assert_eq!(19025, do_blinks("input/2024-11-t1.txt", 25));
        }

        #[test]
        fn solution() {
            assert_eq!(186996, do_blinks("input/2024-11-input.txt", 75));
        }
    }
}
