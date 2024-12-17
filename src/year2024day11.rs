mod year2024day11 {
    use std::sync::mpsc;
    use std::thread;

    #[derive(Clone)]
    struct Stone {
        value: usize,
        blinks: usize
    }

    fn has_even_number_of_digits(value: usize) -> bool {
        ((value as f64).log10().floor() as usize + 1) % 2 == 0
    }
    
    impl Stone {
        fn new(value: usize) -> Self {
            Stone { value, blinks: 0 }
        }
        
        fn blink(&mut self) -> Option<Stone> {
            self.blinks += 1;
            if self.value == 0 {
                self.value = 1;
            } else if has_even_number_of_digits(self.value) {
                let power = (self.value as f64).log10().floor() as u32;
                let half_power = power / 2;
                let a = self.value / 10usize.pow(half_power + 1);
                let b = self.value % 10usize.pow(half_power + 1);
                self.value = a;
                let mut clone = self.clone();
                clone.value = b;
                return Some(clone)
            } else {
                self.value *= 2024;
            }
            None
        }
    }
    
    fn do_blinks(filename: &str, n: usize) -> usize {
        let input = std::fs::read_to_string(filename).unwrap()
            .split_whitespace()
            .map(|s| {
                let value = s.parse::<usize>().unwrap();
                Stone::new(value)
            })
            .collect::<Vec<Stone>>();
        let mut count = 0;
        
        let (tx, rx) = mpsc::channel();
        
        for stone in input {
            let mut stones = vec![stone];
            let tx1 = tx.clone();
            let _ = thread::spawn(move || {
                let mut count = 0;
                let mut others = Vec::new();
                while !stones.is_empty() {
                    let mut stone = stones.pop().unwrap();
                    for _ in stone.blinks..n {
                        if let Some(s) = stone.blink() {
                            others.push(s);
                        }
                    }
                    count += 1;
                    stones.append(&mut others);
                }
                tx1.send(count).unwrap();
            });
        }
        drop(tx);
        
        while let Ok(c) = rx.recv() {
            count += c;
        }
        
        count
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
            fn example() {
                assert_eq!(19025, do_blinks("input/2024-11-t1.txt", 25));
            }

            #[test]
            fn solution() {
                assert_eq!(186996, do_blinks("input/2024-11-input.txt", 75));
            }
        }
    }
}