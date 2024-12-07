mod year2024day6 {
    use crate::read_lines;
    use crate::year2024day6::year2024day6::Direction::*;
    use crate::year2024day6::year2024day6::TraceError::Loop;
    use std::collections::HashSet;
    use std::sync::mpsc;
    use std::thread;

    struct Floor {
        guard: Position,
        guard_start: Position,
        obstacles: Vec<Coord>,
        path: Vec<Position>,
        height: isize,
        width: isize,
    }

    type Coord = (isize, isize);

    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    struct Position {
        location: Coord,
        direction: Direction,
    }

    impl Position {
        fn step(&self) -> Self {
            let l = match &self.direction {
                UP => (self.location.0 - 1, self.location.1),
                DOWN => (self.location.0 + 1, self.location.1),
                RIGHT => (self.location.0, self.location.1 + 1),
                LEFT => (self.location.0, self.location.1 - 1)
            };
            Position { location: l, direction: self.direction }
        }
    }

    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    enum Direction {
        UP, DOWN, LEFT, RIGHT
    }
    
    enum TraceError {
        Loop
    }

    impl Floor {
        fn new(filename: &str) -> Self {
            let mut height = 0;
            let mut width = 0;
            let mut obstacles = Vec::new();
            let mut guard_location = (0, 0);
            let mut lines = read_lines(filename);
            while let Some(Ok(line)) = lines.next() {
                width = line.len() as isize;
                for (i, c) in line.chars().enumerate() {
                    match c {
                        '.' => {} // Do nothing
                        '#' => obstacles.push((height, i as isize)),
                        '^' => guard_location = (height, i as isize),
                        c => panic!("Unexpected character {}", c)
                    }
                }
                height += 1;
            }
            let guard = Position { location: guard_location, direction: UP };
            Floor {
                guard: guard.clone(),
                guard_start: guard.clone(),
                obstacles,
                path: vec![guard],
                height,
                width
            }
        }

        fn trace_path(&mut self) -> Result<(), TraceError> {
            let out_of_bounds = |coord: Coord| -> bool {
                coord.0 < 0 || coord.1 < 0 || coord.0 >= self.height || coord.1 >= self.width
            };
            let mut log = HashSet::new();
            loop {
                let next_location = self.guard.step();
                if out_of_bounds(next_location.location) { break }
                if self.obstacles.iter().any(|&o| o == next_location.location) {
                    self.guard.direction = match self.guard.direction {
                        UP => RIGHT,
                        RIGHT => DOWN,
                        DOWN => LEFT,
                        LEFT => UP
                    };
                    if !log.insert(self.guard.clone()) {
                        return Err(Loop);
                    }
                    self.path.push(self.guard.clone());
                } else {
                    self.guard = next_location;
                    if !log.insert(self.guard.clone()) {
                        return Err(Loop);
                    }
                    self.path.push(self.guard.clone());
                }
            }
            Ok(())
        }
        
        fn plus_obstacle(&self, obstacle: Coord) -> Self {
            let mut obstacles = self.obstacles.clone();
            obstacles.push(obstacle);
            Floor {
                guard: self.guard_start.clone(),
                guard_start: self.guard_start.clone(),
                obstacles,
                path: vec![self.guard_start.clone()],
                height: self.height,
                width: self.width
            }
        }
    }

    fn part1(filename: &str) -> usize {
        let mut floor = Floor::new(filename);
        let _ = floor.trace_path().is_ok();
        let mut positions = HashSet::new();
        for p in floor.path {
            positions.insert(p.location.clone());
        }
        positions.len()
    }

    fn part2(filename: &str) -> usize {
        let mut floor = Floor::new(filename);
        let _ = floor.trace_path();
        let floor = floor;
        let mut looping_obstacles = HashSet::new();
        let (tx, rx) = mpsc::channel();
        for (i, &p) in floor.path.iter().enumerate() {
            if i == 0 { continue }
            let mut attempt = floor.plus_obstacle(p.location);
            let tx1 = tx.clone();
            let _ = thread::spawn(move || {
                match attempt.trace_path() {
                    Err(_) => tx1.send(Some(p.location.clone())).unwrap(),
                    Ok(_) => tx1.send(None).unwrap()
                }
            });
        }
        
        for _ in 0..floor.path.len() - 1 {
            if let Some(p) = rx.recv().unwrap() {
                looping_obstacles.insert(p);
            }
        }
        
        looping_obstacles.len()
    }

    #[cfg(test)]
    mod test {
        mod part1 {
            use crate::year2024day6::year2024day6::part1;

            #[test]
            fn example() {
                assert_eq!(41, part1("input/2024-06-e1.txt"));
            }

            #[test]
            fn solution() {
                assert_eq!(4433, part1("input/2024-06-input.txt"));
            }
        }

        mod part2 {
            use crate::year2024day6::year2024day6::{part2, Floor};
            use std::collections::HashSet;

            #[test]
            fn path_contains_potential_obstacles() {
                let mut uut = Floor::new("input/2024-06-e1.txt");
                let _ = uut.trace_path();
                let expected = vec![
                    (6,3),
                    (7,6),
                    (7,7),
                    (8,1),
                    (8,3),
                    (9,7)];
                let actual: HashSet<_> = uut.path.iter().map(|p| p.location).collect();
                for c in expected {
                    assert!(actual.contains(&c));
                }
            }
            
            #[test]
            fn detect_loop() {
                let mut uut = Floor::new("input/2024-06-e1.txt")
                    .plus_obstacle((6,3));
                assert!(uut.trace_path().is_err());
            }

            #[test]
            fn example() {
                assert_eq!(6, part2("input/2024-06-e1.txt"));
            }

            #[test]
            fn solution() {
                assert_eq!(1516, part2("input/2024-06-input.txt"));
            }
        }
    }
}