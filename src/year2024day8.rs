mod year2024day8 {
    use std::cmp::min;
    use crate::read_lines;
    use std::collections::HashSet;

    type Coord = (isize, isize);

    #[derive(Eq, PartialEq)]
    struct Antenna {
        frequency: char,
        location: Coord,
    }

    impl Antenna {
        fn antinodes_with(&self, other: &Antenna) -> HashSet<Coord> {
            let mut antinodes = HashSet::new();

            let rise = other.location.0 - self.location.0;
            let run = other.location.1 - self.location.1;
            // Inner antinodes
            if rise % 3 == 0 && run % 3 == 0 {
                // Close to self
                let delta_rise = rise / 3;
                let delta_run = run / 3;
                antinodes.insert((self.location.0 + delta_rise, self.location.1 + delta_run));
                // Close to other
                let delta_rise = 2 * rise / 3;
                let delta_run = 2 * run / 3;
                antinodes.insert((self.location.0 + delta_rise, self.location.1 + delta_run));
            }
            // Outer Antinodes
            // Past self
            antinodes.insert((self.location.0 - rise, self.location.1 - run));
            // Past other
            antinodes.insert((other.location.0 + rise, other.location.1 + run));

            antinodes
        }

        fn resonance_line_with(
            &self,
            other: &Antenna,
            height: isize,
            width: isize,
        ) -> HashSet<Coord> {
            let mut line = HashSet::new();

            let mut rise = other.location.0 - self.location.0;
            let mut run = other.location.1 - self.location.1;
            
            for i in (1..=min(rise,run)).rev() {
                if rise.abs() % i == 0 && run.abs() % i == 0 {
                    rise = rise / i;
                    run = run / i;
                }
            }
            
            if rise < 0 {
                rise *= -1;
                run *= -1;
            }
            
            let (rise, run) = (rise, run);

            for row in 0..height {
                if (row - self.location.0) % rise != 0 { continue }
                let steps = (row - self.location.0) / rise;
                let delta_run = steps * run;
                let col = delta_run + self.location.1;
                if col < 0 || col >= width { continue }
                line.insert((row, col));
            }

            line
        }
    }

    struct City {
        antennas: Vec<Antenna>,
        height: isize,
        width: isize,
    }

    impl City {
        fn new(filename: &str) -> Self {
            let mut height = 0;
            let mut width = 0;
            let mut antennas = Vec::new();

            let mut lines = read_lines(filename);
            while let Some(Ok(line)) = lines.next() {
                for (i, c) in line.chars().enumerate() {
                    if !c.is_alphanumeric() {
                        continue;
                    }
                    antennas.push(Antenna {
                        location: (height, i as isize),
                        frequency: c,
                    })
                }
                height += 1;
                width = line.len() as isize;
            }

            Self {
                antennas,
                height,
                width,
            }
        }
    }
    
    enum Part {
        Part1, Part2
    }

    fn analyze(filename: &str, p: Part) -> usize {
        let city = City::new(filename);
        let mut antinodes = HashSet::new();

        for a in &city.antennas {
            for b in &city.antennas {
                if a == b {
                    continue;
                }
                if a.frequency != b.frequency {
                    continue;
                }
                let new = match p { 
                    Part::Part1 => &a.antinodes_with(b),
                    Part::Part2 => &a.resonance_line_with(b, city.height, city.width)
                };
                antinodes = antinodes.union(new).map(|e| *e).collect();
            }
        }

        antinodes = antinodes
            .iter()
            .filter(|coord| {
                coord.0 >= 0 && coord.0 < city.height && coord.1 >= 0 && coord.1 < city.width
            })
            .map(|e| *e)
            .collect();
        antinodes.len()
    }

    #[cfg(test)]
    mod tests {
        mod part1 {
            use crate::year2024day8::year2024day8::{analyze, Antenna};
            use std::collections::HashSet;
            use crate::year2024day8::year2024day8::Part::Part1;

            #[test]
            fn single() {
                let a = Antenna {
                    frequency: 'a',
                    location: (3, 4),
                };
                let b = Antenna {
                    frequency: 'a',
                    location: (5, 5),
                };
                let antinodes = a.antinodes_with(&b);
                let expected = HashSet::from([(1, 3), (7, 6)]);

                assert_eq!(expected, antinodes);
            }

            #[test]
            fn example1() {
                assert_eq!(14, analyze("input/2024-08-e1.txt", Part1))
            }

            #[test]
            fn example2() {
                assert_eq!(4, analyze("input/2024-08-e2.txt", Part1))
            }

            #[test]
            fn solution() {
                assert_eq!(413, analyze("input/2024-08-input.txt", Part1))
            }
        }

        mod part2 {
            use std::collections::HashSet;
            use crate::year2024day8::year2024day8::{analyze, Antenna};
            use crate::year2024day8::year2024day8::Part::Part2;

            #[test]
            fn single() {
                let a = Antenna { frequency: 'a', location: (0, 0) };
                let b = Antenna { frequency: 'a', location: (1, 3) };

                let actual = a.resonance_line_with(&b, 4, 10);
                let expected = HashSet::from([
                    (0, 0),
                    (1, 3),
                    (2, 6),
                    (3, 9)
                ]);
                
                assert_eq!(expected, actual);
            }
            
            #[test]
            fn example1() {
                assert_eq!(9, analyze("input/2024-08-e3.txt", Part2))
            }

            #[test]
            fn solution() {
                assert_eq!(1417, analyze("input/2024-08-input.txt", Part2))
            }
        }
    }
}
