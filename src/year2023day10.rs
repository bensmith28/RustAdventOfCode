mod year2023day10 {
    use crate::read_lines;
    use crate::year2023day10::year2023day10::Direction::*;
    use crate::year2023day10::year2023day10::LoopError::DoesNotLoop;
    use crate::year2023day10::year2023day10::Mark::*;
    use crate::year2023day10::year2023day10::StepError::{BadDirection, FromStart, NoPipe};
    use crate::year2023day10::year2023day10::Tile::*;

    #[derive(Debug, Clone)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    type Coord = (usize, usize);

    #[derive(Clone)]
    struct Position {
        coord: Coord,
        from: Direction,
    }

    #[derive(Debug)]
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
        
        fn try_loop(&self, position: Position) -> Result<Vec<Position>, LoopError> {
            let mut trace = Vec::new();

            let tile = self.get(position.coord.0, position.coord.1);
            match tile {
                Ground => return Err(DoesNotLoop),
                _ => {}
            }
            
            let mut position = position;
            loop {
                position = match self.step(position) {
                    Ok(p) => {
                        trace.push(p.clone());
                        p
                    },
                    Err(FromStart) => return Ok(trace),
                    Err(_) => return Err(DoesNotLoop)
                }
            }
        }
        
        fn trace_loop(&self) -> Vec<Position> {
            let start = self.find_start();
            // Try from North
            let p = Position { coord: (start.0+1, start.1), from: North };
            if let Ok(mut steps) = self.try_loop(p.clone()) {
                steps.insert(0, p);
                return steps;
            }
            // Try from South
            let p = Position { coord: (start.0-1, start.1), from: South };
            if let Ok(mut steps) = self.try_loop(p.clone()) {
                steps.insert(0, p);
                return steps;
            }
            // Try from East
            let p = Position { coord: (start.0, start.1-1), from: East };
            if let Ok(mut steps) = self.try_loop(p.clone()) {
                steps.insert(0, p);
                return steps;
            }
            // Try from West - redundant, should never get here
            /*let p = Position { coord: (start.0, start.1+1), from: West };
            if let Ok(steps) = self.try_loop(p) {
                return steps;
            }*/
            
            panic!("WTF");
        }
        
        fn furthest(&self) -> usize {
            self.trace_loop().len() / 2
        }
    }

    #[derive(Clone)]
    enum Mark {
        Loop, Left, Right, Unmarked
    }

    struct Field {
        marks: Vec<Vec<Mark>>,
        inside_mark: Mark
    }

    impl Field {
        fn new(maze: &Maze) -> Self {
            let mut marks = Vec::with_capacity(maze.tiles.len());
            for row in &maze.tiles {
                marks.push(vec![Unmarked; row.len()]);
            }
            let mut field = Self { marks, inside_mark: Left /* temp */ };

            field.set(maze.find_start(), Loop);
            let trace = maze.trace_loop();
            for p in &trace {
                field.set(p.coord, Loop);
                let tile = maze.get(p.coord.0, p.coord.1);
                match (tile, &p.from) {
                    (Start, _) => {},
                    (NS, North) => {
                        field.try_set((p.coord.0, p.coord.1 + 1), Left);
                        if p.coord.1 > 0 {
                            field.try_set((p.coord.0, p.coord.1 - 1), Right);
                        }
                    }
                    (NS, South) => {
                        field.try_set((p.coord.0, p.coord.1 + 1), Right);
                        if p.coord.1 > 0 {
                            field.try_set((p.coord.0, p.coord.1 - 1), Left);
                        }
                    }
                    (EW, East) => {
                        field.try_set((p.coord.0 + 1, p.coord.1), Left);
                        if p.coord.0 > 0 {
                            field.try_set((p.coord.0 - 1, p.coord.1), Right);
                        }
                    }
                    (EW, West) => {
                        field.try_set((p.coord.0 + 1, p.coord.1), Right);
                        if p.coord.0 > 0 {
                            field.try_set((p.coord.0 - 1, p.coord.1), Left);
                        }
                    }
                    (NE, North) => {
                        if p.coord.1 > 0 {
                            field.try_set((p.coord.0, p.coord.1 - 1), Right);
                        }
                        field.try_set((p.coord.0 + 1, p.coord.1), Right);
                    }
                    (NE, East) => {
                        if p.coord.1 > 0 {
                            field.try_set((p.coord.0, p.coord.1 - 1), Left);
                        }
                        field.try_set((p.coord.0 + 1, p.coord.1), Left);
                    }
                    (NW, North) => {
                        field.try_set((p.coord.0, p.coord.1 + 1), Left);
                        field.try_set((p.coord.0 + 1, p.coord.1), Left);
                    }
                    (NW, West) => {
                        field.try_set((p.coord.0, p.coord.1 + 1), Right);
                        field.try_set((p.coord.0 + 1, p.coord.1), Right);
                    }
                    (SW, South) => {
                        field.try_set((p.coord.0, p.coord.1 + 1), Right);
                        if p.coord.0 > 0 {
                            field.try_set((p.coord.0 - 1, p.coord.1), Right);
                        }
                    }
                    (SW, West) => {
                        field.try_set((p.coord.0, p.coord.1 + 1), Left);
                        if p.coord.0 > 0 {
                            field.try_set((p.coord.0 - 1, p.coord.1), Left);
                        }
                    }
                    (SE, South) => {
                        if p.coord.1 > 0 {
                            field.try_set((p.coord.0, p.coord.1 - 1), Left);
                        }
                        if p.coord.0 > 0 {
                            field.try_set((p.coord.0 - 1, p.coord.1), Left);
                        }
                    }
                    (SE, East) => {
                        if p.coord.1 > 0 {
                            field.try_set((p.coord.0, p.coord.1 - 1), Right);
                        }
                        if p.coord.0 > 0 {
                            field.try_set((p.coord.0 - 1, p.coord.1), Right);
                        }
                    }
                    (tile, from) => {
                        panic!("Invalid case: {:?} from the {:?}", tile, from);
                    }
                }
            }
            
            loop {
                if field.marks.iter().all(|row| row.iter().all(|mark| match mark {
                    Unmarked => false,
                    _ => true,
                })) { break }
                
                for r in 0..field.marks.len() {
                    for c in 0..field.marks.first().unwrap().len() {
                        let coord = (r, c);
                        match field.marks[r][c] {
                            Unmarked => {
                                field.try_set(coord, field.find_adjacents(coord));
                            }
                            _ => {}
                        }
                    }
                }
            }
            
            let lefts_on_the_loop = trace.iter().filter( |p| {
                let tile = maze.get(p.coord.0, p.coord.1);
                match (tile, &p.from) {
                    (NE, North) | (NW, West) | (SE, East) | (SW, South) => true,
                    _ => false,
                }
            }).count();
            let rights_on_the_loop = trace.iter().filter( |p| {
                let tile = maze.get(p.coord.0, p.coord.1);
                match (tile, &p.from) {
                    (NE, East) | (NW, North) | (SE, South) | (SW, West) => true,
                    _ => false,
                }
            }).count();
            
            let inside_mark = if lefts_on_the_loop > rights_on_the_loop {
                Left
            } else {
                Right
            };
            field.inside_mark = inside_mark;

            field
        }

        fn set(&mut self, coord: Coord, mark: Mark) {
            self.marks[coord.0][coord.1] = mark;
        }

        fn try_set(&mut self, coord: Coord, mark: Mark) {
            if coord.0 >= self.marks.len() {
                return;
            }
            if coord.1 >= self.marks.first().unwrap().len() {
                return;
            }

            match self.marks[coord.0][coord.1] {
                Unmarked => { self.set(coord, mark) }
                _ => {}
            }
        }
        
        fn try_get(&self, coord: Coord) -> &Mark {
            if coord.0 >= self.marks.len() {
                return &Unmarked;
            }
            if coord.1 >= self.marks.first().unwrap().len() {
                return &Unmarked;
            }
            
            &self.marks[coord.0][coord.1]
        }

        fn find_adjacents(&self, coord: Coord) -> Mark {
            if coord.0 > 0 {
                match self.try_get((coord.0 - 1, coord.1)) {
                    Left => return Left,
                    Right => return Right,
                    _ => {}
                };
            }
            match self.try_get((coord.0 + 1, coord.1)) {
                Left => return Left,
                Right => return Right,
                _ => {}
            };
            if coord.1 > 0 {
                match self.try_get((coord.0, coord.1 - 1)) {
                    Left => return Left,
                    Right => return Right,
                    _ => {}
                };
            }
            match self.try_get((coord.0, coord.1 + 1)) {
                Left => return Left,
                Right => return Right,
                _ => {}
            };
            
            Unmarked
        }
        
        fn count_inside(&self) -> usize {
            self.marks.iter().map(|row| {
                row.iter().filter(|mark| {
                    match (&self.inside_mark, mark) {
                        (Left, Left) | (Right, Right) => true,
                        _ => false,
                    }
                }).count()
            }).sum()
        }
    }
    
    #[cfg(test)]
    mod tests {
        mod part1 {
            use crate::year2023day10::year2023day10::Maze;

            #[test]
            fn example_steps() {
                let maze = Maze::new("input/2023-10-e1.txt");
                assert_eq!(8, maze.trace_loop().len())
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
        
        mod part2 {
            use crate::year2023day10::year2023day10::{Field, Maze};

            #[test]
            fn example_3() {
                let maze = Maze::new("input/2023-10-e3.txt");
                let field = Field::new(&maze);
                
                assert_eq!(4, field.count_inside())
            }

            #[test]
            fn example_4() {
                let maze = Maze::new("input/2023-10-e4.txt");
                let field = Field::new(&maze);

                assert_eq!(8, field.count_inside())
            }

            #[test]
            fn example_5() {
                let maze = Maze::new("input/2023-10-e5.txt");
                let field = Field::new(&maze);

                assert_eq!(10, field.count_inside())
            }

            #[test]
            fn solution() {
                let maze = Maze::new("input/2023-10-input.txt");
                let field = Field::new(&maze);

                assert_eq!(501, field.count_inside())
            }
        }
    }
}