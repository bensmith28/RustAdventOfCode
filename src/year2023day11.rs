mod year2023day11 {
    use crate::read_lines;

    type Coord = (usize, usize);
    struct GalaxyMap {
        galaxies: Vec<Coord>,
    }

    impl GalaxyMap {
        fn new(filename: &str, distance_multiplier: usize) -> Self {
            #[derive(Eq, PartialEq)]
            enum Space {
                Empty, Galaxy
            }

            fn char_to_space(c: char) -> Space {
                match c {
                    '.' => Space::Empty,
                    '#' => Space::Galaxy,
                    c => panic!("Invalid char: {}", c)
                }
            }

            let map: Vec<Vec<Space>> = read_lines(filename)
                .map(|line| {
                    let line_string = line.unwrap();
                    line_string.chars().map(|c| char_to_space(c)).collect()
                }).collect();
            let empty_rows: Vec<usize> = map.iter().enumerate().filter_map(|(i, row)| {
                if row.iter().all(|s| *s == Space::Empty) {
                    Some(i)
                } else {
                    None
                }
            }).collect();
            let mut empty_cols: Vec<usize> = Vec::new();
            for i in 0..map.first().unwrap().len() {
                if map.iter().all(|row| row[i] == Space::Empty) {
                    empty_cols.push(i);
                }
            }

            let mut galaxies = Vec::new();
            for (r, row) in map.iter().enumerate() {
                for(c, space) in row.iter().enumerate() {
                    if *space == Space::Empty {
                        continue;
                    }
                    let c_offset = empty_cols.iter().filter(|&&e| e < c).count()
                        * distance_multiplier;
                    let r_offset = empty_rows.iter().filter(|&&e| e < r).count()
                        * distance_multiplier;
                    galaxies.push((r + r_offset, c + c_offset));
                }
            }

            GalaxyMap { galaxies }
        }
        
        fn sum_distances(&self) -> usize {
            let mut sum = 0;
            
            for a in 0..self.galaxies.len() {
                for b in a+1..self.galaxies.len() {
                    let ga = self.galaxies[a];
                    let gb = self.galaxies[b];
                    sum += ga.1.abs_diff(gb.1) + ga.0.abs_diff(gb.0);
                }
            }
            
            sum
        }
    }

    #[cfg(test)]
    mod tests {
        mod parse {
            use crate::year2023day11::year2023day11::GalaxyMap;

            #[test]
            fn sample() {
                let map = GalaxyMap::new("input/2023-11-e1.txt", 1);
                assert_eq!(9, map.galaxies.len());
                assert_eq!(vec![
                    (0,4),
                    (1,9),
                    (2,0),
                    (5,8),
                    (6,1),
                    (7,12),
                    (10,9),
                    (11,0),
                    (11,5)
                ], map.galaxies)
            }
        }
        
        mod part1 {
            use crate::year2023day11::year2023day11::GalaxyMap;

            #[test]
            fn example() {
                let map = GalaxyMap::new("input/2023-11-e1.txt", 1);
                assert_eq!(374, map.sum_distances());
            }

            #[test]
            fn solution() {
                let map = GalaxyMap::new("input/2023-11-input.txt", 1);
                assert_eq!(10494813, map.sum_distances());
            }
        }
        
        mod part2 {
            use crate::year2023day11::year2023day11::GalaxyMap;

            #[test]
            fn example1() {
                let map = GalaxyMap::new("input/2023-11-e1.txt", 9);
                assert_eq!(1030, map.sum_distances());
            }

            #[test]
            fn example2() {
                let map = GalaxyMap::new("input/2023-11-e1.txt", 99);
                assert_eq!(8410, map.sum_distances());
            }

            #[test]
            fn solution() {
                let map = GalaxyMap::new("input/2023-11-input.txt", 999999);
                assert_eq!(840988812853, map.sum_distances());
            }
        }
    }
}