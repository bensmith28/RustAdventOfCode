mod year2023day3 {
    use std::ops::Range;
    use crate::read_lines;

    #[derive(Debug)]
    #[derive(PartialEq)]
    struct Location {
        x: Range<usize>,
        y: usize
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    enum Tile {
        Number(usize, Location),
        Symbol(char, Location)
    }

    impl Tile {
        fn is_adjacent(&self, other: &Tile) -> bool {
            let self_location = match self {
                Tile::Number(_, l) => l,
                Tile::Symbol(_, l) => l,
            };
            let other_location = match other {
                Tile::Number(_, l) => l,
                Tile::Symbol(_, l) => l,
            };
            self_location.is_adjacent(other_location)
        }
    }

    impl Location {
        fn is_adjacent(&self, other: &Location) -> bool {
            self.y.abs_diff(other.y) <= 1 &&
                self.x.start <= other.x.end && other.x.start <= self.x.end
        }
    }

    fn filter_for_part1(tiles: &[Tile]) -> Vec<&Tile> {
        let symbols: Vec<_> = tiles.iter()
            .filter(|t| matches!(t, Tile::Symbol(_, _)))
            .collect();
        tiles.iter()
            .filter(|t| match t {
                Tile::Symbol(_, _) => false,
                Tile::Number(_,_) => symbols.iter().any(|s| s.is_adjacent(t))
            })
            .collect()
    }
    
    fn filter_for_part2(tiles: &[Tile]) -> Vec<usize> {
        let gears: Vec<_> = tiles.iter()
            .filter(|t| matches!(t, Tile::Symbol('*', _)))
            .collect();
        let numbers: Vec<_> = tiles.iter()
            .filter(|t| matches!(t, Tile::Number(_, _)))
            .collect();
        let mut result = Vec::new();
        for g in gears {
            let adj: Vec<_> = numbers.iter()
                .filter(|n| n.is_adjacent(g))
                .collect();
            if adj.len() == 2 {
                let ratio = adj.iter()
                    .map(|n| match n {
                        Tile::Number(v, _) => v,
                        _ => panic!("Should not have any non-number tile: {:?}", n),
                    }).fold(1, |acc, n| acc * n);
                result.push(ratio);
            }
        }
        result
    }

    fn parse_schematic(file_name: &str) -> Vec<Tile> {
        read_lines(file_name).into_iter().enumerate().fold(vec![], |mut acc, (y, line)| {
            for tile in parse_line(&line.unwrap(), y) {
                acc.push(tile);
            }
            acc
        })
    }

    fn parse_line(line: &str, y: usize) -> Vec<Tile> {
        let mut result = Vec::new();
        let mut v = 0;
        let mut x_start = None;
        for (idx, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if x_start.is_none() {
                    x_start = Some(idx);
                }
                v = v * 10 + c.to_digit(10).unwrap();
            } else {
                if x_start.is_some() {
                    result.push(Tile::Number(v as usize, Location {
                        x: x_start.unwrap()..idx,
                        y: y
                    }));
                    v = 0;
                    x_start = None;
                }
                if c != '.' {
                    result.push(Tile::Symbol(c, Location {
                        x: idx..idx+1,
                        y: y
                    }))
                }
            }
        }
        if x_start.is_some() {
            result.push(Tile::Number(v as usize, Location {
                x: x_start.unwrap()..line.len(),
                y: y
            }));
        }
        result
    }

    #[cfg(test)]
    mod tests {
        mod parse {
            use crate::year2023day3::year2023day3::{parse_line, parse_schematic, Location, Tile};

            #[test]
            fn handles_line_two_numbers() {
                let line = "467..114..";
                let result = parse_line(line, 0);
                let expected = vec![
                    Tile::Number(467, Location { x: 0..3, y: 0}),
                    Tile::Number(114, Location { x: 5..8, y: 0}),
                ];
                assert_eq!(result, expected);
            }

            #[test]
            fn handles_line_two_symbols() {
                let line = "...$.*....";
                let result = parse_line(line, 0);
                let expected = vec![
                    Tile::Symbol('$', Location { x: 3..4, y: 0}),
                    Tile::Symbol('*', Location { x: 5..6, y: 0}),
                ];
                assert_eq!(result, expected);
            }

            #[test]
            fn handles_line_mixed() {
                let line = ".....+.58.";
                let result = parse_line(line, 0);
                let expected = vec![
                    Tile::Symbol('+', Location { x: 5..6, y: 0}),
                    Tile::Number(58, Location { x: 7..9, y: 0}),
                ];
                assert_eq!(result, expected);
            }

            #[test]
            fn handles_line_no_trailing_period() {
                let line = ".......581";
                let result = parse_line(line, 0);
                let expected = vec![
                    Tile::Number(581, Location { x: 7..10, y: 0}),
                ];
                assert_eq!(result, expected);
            }

            #[test]
            fn handles_schematic_not_empty() {
                let result = parse_schematic("input/2023-03-e1.txt");
                assert_eq!(result.len(), 16)
            }
        }

        mod adjacent {
            use crate::year2023day3::year2023day3::Location;

            #[test]
            fn handles_location_adjacent() {
                assert!(Location { x: 1..2, y: 0 }
                    .is_adjacent(&Location { x: 2..3, y: 0 }));
                assert!(Location { x: 2..3, y: 0 }
                    .is_adjacent(&Location { x: 1..2, y: 0 }));
            }
        }
        
        mod part1 {
            use crate::year2023day3::year2023day3::{filter_for_part1, parse_schematic, Tile};

            #[test]
            fn example() {
                let tiles = parse_schematic("input/2023-03-e1.txt");
                let result: usize = filter_for_part1(&tiles)
                    .iter().map(|t| match t {
                    &Tile::Number(v, _) => *v,
                    _ => 0usize,
                }).sum();
                assert_eq!(result, 4361)
            }

            #[test]
            fn solution() {
                let tiles = parse_schematic("input/2023-03-input.txt");
                let result: usize = filter_for_part1(&tiles)
                    .iter().map(|t| match t {
                    &Tile::Number(v, _) => *v,
                    _ => 0usize,
                }).sum();
                assert_eq!(result, 533784)
            }
        }
        
        mod part2 {
            use crate::year2023day3::year2023day3::{filter_for_part2, parse_schematic};

            #[test]
            fn example() {
                let tiles = parse_schematic("input/2023-03-e1.txt");
                let result: usize = filter_for_part2(&tiles)
                    .iter().sum();
                assert_eq!(result, 467835)
            }

            #[test]
            fn solution() {
                let tiles = parse_schematic("input/2023-03-input.txt");
                let result: usize = filter_for_part2(&tiles)
                    .iter().sum();
                assert_eq!(result, 78826761)
            }
        }
    }
}