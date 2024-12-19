use std::cmp::Ordering;
use crate::read_lines;

struct Garden {
    regions: Vec<Region>,
}

#[derive(PartialEq, Eq)]
struct Region {
    id: char,
    coords: Vec<Coord>
}

impl Region {
    fn is_adjacent_coord(&self, other: &Coord) -> bool {
        self.coords.contains(&other) || self.coords.iter().any(|me| {
            coords_are_adjacent(me, other)
        })
    }

    fn area(&self) -> usize {
        self.coords.len()
    }

    fn perimeter(&self) -> usize {
        self.coords.iter().map(|coord| {
            4 - self.coords.iter().filter(|other| {
                *other != coord && coords_are_adjacent(coord, other)
            }).count()
        }).sum()
    }
    
    fn side_count(&self) -> usize {
        let row_first_sort = |a: &&Coord, b: &&Coord| -> Ordering {
            let row = a.0.cmp(&b.0);
            if row != Ordering::Equal {
                row
            } else {
                a.1.cmp(&b.1)
            }
        };
        let col_first_sort = |a: &&Coord, b: &&Coord| -> Ordering {
            let col = a.1.cmp(&b.1);
            if col != Ordering::Equal {
                col
            } else {
                a.0.cmp(&b.0)
            }
        };
        let mut segments = 4;
        let mut ups = self.coords.iter().filter(|coord| {
            coord.0 == 0 || !self.coords.contains(&(coord.0 - 1, coord.1))
        }).collect::<Vec<_>>();
        ups.sort_by(row_first_sort);
        for window in ups.windows(2) {
            if window[0].0 != window[1].0 || window[0].1 + 1 != window[1].1 {
                segments += 1;
            }
        }
        
        let mut rights = self.coords.iter().filter(|coord| {
            !self.coords.contains(&(coord.0, coord.1 + 1))
        }).collect::<Vec<_>>(); 
        rights.sort_by(col_first_sort);
        for window in rights.windows(2) {
            if window[0].1 != window[1].1 || window[0].0 + 1 != window[1].0 {
                segments += 1;
            }
        }
        
        let mut downs = self.coords.iter().filter(|coord| {
            !self.coords.contains(&(coord.0 + 1, coord.1))
        }).collect::<Vec<_>>();
        downs.sort_by(row_first_sort);
        for window in downs.windows(2) {
            if window[0].0 != window[1].0 || window[0].1 + 1 != window[1].1 {
                segments += 1;
            }
        }
        
        let mut lefts = self.coords.iter().filter(|coord| {
            coord.1 == 0 || !self.coords.contains(&(coord.0, coord.1 - 1))
        }).collect::<Vec<_>>();
        lefts.sort_by(col_first_sort);
        for window in lefts.windows(2) {
            if window[0].1 != window[1].1 || window[0].0 + 1 != window[1].0 {
                segments += 1;
            }
        }
        segments
    }
}

type Coord = (usize, usize);

fn coords_are_adjacent(a: &Coord, b: &Coord) -> bool {
    (a.0.abs_diff(b.0) == 1 && a.1.abs_diff(b.1) == 0) ||
        (a.0.abs_diff(b.0) == 0 && a.1.abs_diff(b.1) == 1)
}

impl Garden {
    fn new(filename: &str) -> Garden {
        let mut regions: Vec<Region> = Vec::new();
        let mut lines = read_lines(filename);
        let mut row = 0;
        while let Some(Ok(line)) = lines.next() {
            for (col, c) in line.chars().enumerate() {
                let mut adjacents = regions.iter_mut().filter(|r| {
                    r.id == c && r.is_adjacent_coord(&(row, col))
                }).collect::<Vec<&mut Region>>();
                if adjacents.is_empty() {
                    regions.push(Region { id: c, coords: vec![(row, col)] });
                } else if adjacents.len() == 1 {
                    adjacents[0].coords.push((row, col));
                } else {
                    let mut coords = vec![(row, col)];
                    for r in adjacents {
                        coords.append(r.coords.as_mut());
                    }
                    regions.push(Region { id: c, coords });
                }
            }
            row += 1;
        }
        regions.retain(|r| !r.coords.is_empty());

        Garden {
            regions
        }
    }

    fn price(&self) -> usize {
        self.regions.iter().map(|r| r.area() * r.perimeter()).sum()
    }

    fn price_by_side(&self) -> usize {
        self.regions.iter().map(|r| r.area() * r.side_count()).sum()
    }
}

#[cfg(test)]
mod tests {
    mod part1 {
        use crate::year2024day12::Garden;

        #[test]
        fn example1() {
            let garden = Garden::new("input/2024-12-e1.txt");
            assert_eq!(140, garden.price());
        }

        #[test]
        fn example2() {
            let garden = Garden::new("input/2024-12-e2.txt");
            assert_eq!(772, garden.price());
        }

        #[test]
        fn example3() {
            let garden = Garden::new("input/2024-12-e3.txt");
            assert_eq!(1930, garden.price());
        }

        #[test]
        fn solution() {
            let garden = Garden::new("input/2024-12-input.txt");
            assert_eq!(1370258, garden.price());
        }
    }
    mod part2 {
        use crate::year2024day12::Garden;

        #[test]
        fn example1() {
            let garden = Garden::new("input/2024-12-e1.txt");
            assert_eq!(80, garden.price_by_side());
        }

        #[test]
        fn example3() {
            let garden = Garden::new("input/2024-12-e3.txt");
            assert_eq!(1206, garden.price_by_side());
        }

        #[test]
        fn example4() {
            let garden = Garden::new("input/2024-12-e4.txt");
            assert_eq!(236, garden.price_by_side());
        }

        #[test]
        fn example5() {
            let garden = Garden::new("input/2024-12-e5.txt");
            assert_eq!(368, garden.price_by_side());
        }

        #[test]
        fn solution() {
            let garden = Garden::new("input/2024-12-input.txt");
            assert_eq!(805814, garden.price_by_side());
        }
    }
}