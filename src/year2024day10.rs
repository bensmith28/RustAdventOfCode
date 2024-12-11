mod year2024day10 {
    use crate::read_lines;
    use std::collections::HashSet;
    use std::sync::mpsc;
    use std::thread;

    type TrailMap = Vec<Vec<usize>>;
    type Coord = (usize, usize);

    fn parse_trail_map(filename: &str) -> TrailMap {
        let mut m = Vec::new();

        let mut lines = read_lines(filename);
        while let Some(Ok(line)) = lines.next() {
            m.push(line.chars()
                .map(|c| {
                    match c.to_digit(10) {
                        Some(d) => d as usize,
                        _ => usize::MAX
                    }
                })
                .collect());
        }

        m
    }

    fn wander_part_1(trailhead: Coord, trail_map: TrailMap) -> usize {
        let mut peaks: HashSet<Coord> = HashSet::new();
        let height = trail_map.len();
        let width = trail_map[0].len();
        let mut options = vec![trailhead];
        while !options.is_empty() {
            let current = options.pop().unwrap();
            let next_elevation = trail_map[current.0][current.1] + 1;
            if next_elevation == 10 {
                peaks.insert(current);
                continue;
            }
            // Up
            if current.0 > 0 && trail_map[current.0 - 1][current.1] == next_elevation {
                options.push((current.0 - 1, current.1));
            }
            // Down
            if current.0 < height - 1 && trail_map[current.0 + 1][current.1] == next_elevation {
                options.push((current.0 + 1, current.1));
            }
            // Left
            if current.1 > 0 && trail_map[current.0][current.1 - 1] == next_elevation {
                options.push((current.0, current.1 - 1));
            }
            // Right
            if current.1 < width - 1 && trail_map[current.0][current.1 + 1] == next_elevation {
                options.push((current.0, current.1 + 1));
            }
        }
        peaks.len()
    }
    
    type Trail = Vec<Coord>;

    fn wander_part_2(trailhead: Coord, trail_map: TrailMap) -> usize {
        let mut trails: HashSet<Trail> = HashSet::new();
        let height = trail_map.len();
        let width = trail_map[0].len();
        let mut options: Vec<Trail> = vec![vec![trailhead]];
        while !options.is_empty() {
            let current_trail = options.pop().unwrap();
            let current = current_trail.last().unwrap();
            let next_elevation = trail_map[current.0][current.1] + 1;
            if next_elevation == 10 {
                trails.insert(current_trail);
                continue;
            }
            // Up
            if current.0 > 0 && trail_map[current.0 - 1][current.1] == next_elevation {
                let up = (current.0 - 1, current.1);
                let mut trail = current_trail.clone();
                trail.push(up);
                options.push(trail);
            }
            // Down
            if current.0 < height - 1 && trail_map[current.0 + 1][current.1] == next_elevation {
                let down = (current.0 + 1, current.1);
                let mut trail = current_trail.clone();
                trail.push(down);
                options.push(trail);
            }
            // Left
            if current.1 > 0 && trail_map[current.0][current.1 - 1] == next_elevation {
                let left = (current.0, current.1 - 1);
                let mut trail = current_trail.clone();
                trail.push(left);
                options.push(trail);
            }
            // Right
            if current.1 < width - 1 && trail_map[current.0][current.1 + 1] == next_elevation {
                let right = (current.0, current.1 + 1);
                let mut trail = current_trail.clone();
                trail.push(right);
                options.push(trail);
            }
        }
        trails.len()
    }

    fn evaluate_trail_map(
        filename: &str,
        wander: fn(Coord, TrailMap) -> usize,
    ) -> usize {
        let trail_map = parse_trail_map(filename);
        let mut trailheads: Vec<Coord> = Vec::new();
        for (r, row) in trail_map.iter().enumerate() {
            for (c, &elevation) in row.iter().enumerate() {
                if elevation == 0 {
                    trailheads.push((r, c));
                }
            }
        }
        let mut score = 0;

        let (tx, rx) = mpsc::channel();
        for th in trailheads {
            let tx1 = tx.clone();
            let tm = trail_map.clone();
            let _ = thread::spawn(move || {
                tx1.send(wander(th, tm)).unwrap();
            });
        }
        drop(tx);

        while let Ok(s) = rx.recv() {
            score += s;
        }

        score
    }

    fn part1(filename: &str) -> usize {
        evaluate_trail_map(filename, wander_part_1)
    }

    fn part2(filename: &str) -> usize {
        evaluate_trail_map(filename, wander_part_2)
    }

    #[cfg(test)]
    mod tests {
        mod part1 {
            use crate::year2024day10::year2024day10::part1;

            #[test]
            fn example1() {
                assert_eq!(part1("input/2024-10-e1.txt", ), 2);
            }

            #[test]
            fn example2() {
                assert_eq!(part1("input/2024-10-e2.txt", ), 4);
            }

            #[test]
            fn example3() {
                assert_eq!(part1("input/2024-10-e3.txt", ), 3);
            }

            #[test]
            fn example4() {
                assert_eq!(part1("input/2024-10-e4.txt", ), 36);
            }

            #[test]
            fn solution() {
                assert_eq!(part1("input/2024-10-input.txt", ), 688);
            }
        }

        mod part2 {
            use crate::year2024day10::year2024day10::part2;

            #[test]
            fn example5() {
                assert_eq!(part2("input/2024-10-e5.txt", ), 3);
            }

            #[test]
            fn example6() {
                assert_eq!(part2("input/2024-10-e6.txt", ), 13);
            }

            #[test]
            fn example7() {
                assert_eq!(part2("input/2024-10-e7.txt", ), 227);
            }

            #[test]
            fn example4() {
                assert_eq!(part2("input/2024-10-e4.txt", ), 81);
            }

            #[test]
            fn solution() {
                assert_eq!(part2("input/2024-10-input.txt", ), 1459);
            }
        }
    }
}