mod year2023day10 {
    use crate::read_lines;
    use crate::year2023day10::year2023day10::Direction::*;
    use crate::year2023day10::year2023day10::LoopError::DoesNotLoop;
    use crate::year2023day10::year2023day10::StepError::{BadDirection, FromStart, NoPipe};
    use crate::year2023day10::year2023day10::Tile::*;

    #[derive(Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    type Coord = (usize, usize);

    struct Position {
        coord: Coord,
        from: Direction,
    }

    enum Tile {
        NE,
        NS,
        NW,
        EW,
        SE,
        SW,
        Ground,
        Start,
    }

    struct Maze {
        tiles: Vec<Vec<Tile>>,
    }

    enum StepError {
        NoPipe,
        BadDirection,
        FromStart
    }
    
    enum LoopError {
        DoesNotLoop
    }

    impl Maze {
        fn new(filename: &str) -> Self {
            let mut tiles = Vec::new();
            let mut lines = read_lines(filename);

            while let Some(Ok(line)) = lines.next() {
                let mut row = Vec::new();
                for c in line.chars() {
                    match c {
                        '|' => row.push(NS),
                        '-' => row.push(EW),
                        'L' => row.push(NE),
                        'J' => row.push(NW),
                        '7' => row.push(SW),
                        'F' => row.push(SE),
                        '.' => row.push(Ground),
                        'S' => row.push(Start),
                        c => panic!("Unexpected character: {}", c)
                    }
                }
                tiles.push(row)
            }

            Maze { tiles }
        }

        fn find_start(&self) -> Coord {
            for (i, row) in self.tiles.iter().enumerate() {
                for (j, tile) in row.iter().enumerate() {
                    match tile {
                        Start => return (i, j),
                        _ => {}
                    }
                }
            }
            panic!("No start found")
        }

        fn get(&self, row: usize, col: usize) -> &Tile {
            &self.tiles[row][col]
        }

        fn step(&self, position: Position) -> Result<Position, StepError> {
            let (r, c) = position.coord;
            match (self.get(r,c), position.from) {
                (NS, North) => Ok(Position { coord: (r+1, c), from: North }),
                (NS, South) => Ok(Position { coord: (r-1, c), from: South }),
                (NS, _) => Err(BadDirection),
                (EW, East) => Ok(Position { coord: (r, c-1), from: East }),
                (EW, West) => Ok(Position { coord: (r, c+1), from:West }),
                (EW, _) => Err(BadDirection),
                (NE, North) => Ok(Position { coord: (r, c+1), from: West }),
                (NE, East) => Ok(Position { coord: (r-1, c), from: South }),
                (NE, _) => Err(BadDirection),
                (NW, North) => Ok(Position { coord: (r, c-1), from: East }),
                (NW, West) => Ok(Position { coord: (r-1, c), from: South }),
                (NW, _) => Err(BadDirection),
                (SW, South) => Ok(Position { coord: (r, c-1), from: East }),
                (SW, West) => Ok(Position { coord: (r+1, c), from: North }),
                (SW, _) => Err(BadDirection),
                (SE, South) => Ok(Position { coord: (r, c+1), from: West }),
                (SE, East) => Ok(Position { coord: (r+1, c), from: North }),
                (SE, _) => Err(BadDirection),
                (Start, _) => Err(FromStart),
                (_, _) => Err(NoPipe),
            }
        }
        
        fn count_loop(&self, position: Position) -> Result<usize, LoopError> {
            let mut count = 0;

            let tile = self.get(position.coord.0, position.coord.1);
            match tile {
                Ground => return Err(DoesNotLoop),
                _ => {}
            }
            
            let mut position = position;
            loop {
                position = match self.step(position) {
                    Ok(p) => {
                        count += 1;
                        p
                    },
                    Err(FromStart) => return Ok(count),
                    Err(_) => return Err(DoesNotLoop)
                }
            }
        }
        
        fn steps_on_loop(&self) -> usize {
            let start = self.find_start();
            // Try from North
            let p = Position { coord: (start.0+1, start.1), from: North };
            if let Ok(steps) = self.count_loop(p) {
                return steps;
            }
            // Try from South
            let p = Position { coord: (start.0-1, start.1), from: South };
            if let Ok(steps) = self.count_loop(p) {
                return steps;
            }
            // Try from East
            let p = Position { coord: (start.0, start.1-1), from: East };
            if let Ok(steps) = self.count_loop(p) {
                return steps;
            }
            // Try from West
            let p = Position { coord: (start.0, start.1+1), from: West };
            if let Ok(steps) = self.count_loop(p) {
                return steps;
            }
            
            for d in vec![North, East, South, West] {
                if let Ok(steps) = self.count_loop(Position { coord: start, from: d }) {
                    return steps;
                }
            }
            
            panic!("WTF");
        }
        
        fn furthest(&self) -> usize {
            self.steps_on_loop() / 2 + 1
        }
    }
    
    
    
    #[cfg(test)]
    mod tests {
        mod part1 {
            use crate::year2023day10::year2023day10::Maze;

            #[test]
            fn example_steps() {
                let maze = Maze::new("input/2023-10-e1.txt");
                assert_eq!(7, maze.steps_on_loop())
            }
            
            #[test]
            fn example1() {
                let maze = Maze::new("input/2023-10-e1.txt");
                assert_eq!(4, maze.furthest())
            }

            #[test]
            fn example2() {
                let maze = Maze::new("input/2023-10-e2.txt");
                assert_eq!(8, maze.furthest())
            }
            
            #[test]
            fn solution() {
                let maze = Maze::new("input/2023-10-input.txt");
                assert_eq!(6856, maze.furthest())
            }
        }
    }
}