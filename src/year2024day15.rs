use std::cmp::PartialEq;
use crate::read_lines;

type Room = Vec<Vec<Tile>>;

#[derive(PartialEq, Eq, Debug)]
enum Tile {
    Wall,
    Box,
    Empty,
    Robot
}

enum Move {
    Up, 
    Down, 
    Left, 
    Right
}

fn parse(filename: &str) -> (Room, Vec<Move>) {
    let mut room = Vec::new();

    let mut lines = read_lines(filename);
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut row = Vec::new();
        for c in line.chars() {
            let t = match c { 
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                'O' => Tile::Box,
                '@' => Tile::Robot,
                _ => panic!("unknown tile")
            };
            row.push(t);
        }
        room.push(row);
    }

    let mut moves = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        for c in line.chars() {
            match c {
                '^' => moves.push(Move::Up),
                'v' => moves.push(Move::Down),
                '<' => moves.push(Move::Left),
                '>' => moves.push(Move::Right),
                _ => panic!("unknown move: {}", c)
            }
        }
    }
    (room, moves)
}

fn part1(filename: &str) -> usize {
    let (mut room, moves) = parse(filename);
    let mut robot: (isize, isize) = {
        let mut row = -1;
        let mut col = -1;
        for (r, tiles) in room.iter().enumerate() {
            for (c, tile) in tiles.iter().enumerate() {
                if let Tile::Robot = tile {
                    row = r as isize;
                    col = c as isize;
                    break;
                }
            }
        }
        (row, col)
    };
    for m in moves {
        let delta = match m {
            Move::Up => (-1, 0),
            Move::Down => (1, 0),
            Move::Left => (0, -1),
            Move::Right => (0, 1)
        };
        let mut next_position = (robot.0 + delta.0, robot.1 + delta.1);
        loop {
            let next_tile = &room[next_position.0 as usize][next_position.1 as usize];
            match next_tile {
                Tile::Wall => break,
                Tile::Box => {
                    next_position = (next_position.0 + delta.0, next_position.1 + delta.1)
                },
                Tile::Empty => {
                    room[next_position.0 as usize][next_position.1 as usize] = Tile::Box;
                    room[robot.0 as usize][robot.1 as usize] = Tile::Empty;
                    robot = (robot.0 + delta.0, robot.1 + delta.1);
                    room[robot.0 as usize][robot.1 as usize] = Tile::Robot;
                    break;
                },
                _ => panic!("unknown tile {:?}", next_tile)
            }
        }
    }
    // still need to calculate the score
    let mut score = 0;
    for (r, tiles) in room.iter().enumerate() {
        for (c, tile) in tiles.iter().enumerate() {
            if let Tile::Box = tile {
                score += (r * 100) + c;
            }
        }
    }
    score
}

type XY = (isize, isize);

#[derive(Clone, Eq, PartialEq)]
struct Box {
    position: XY
}

impl Box {
    fn new(position: XY) -> Self {
        Self { position }
    }
    
    fn contains(&self, position: XY) -> bool {
        self.position.0 == position.0 && (self.position.1 == position.1 || self.position.1  + 1 == position.1)
    }
    
    fn edges(&self) -> Vec<XY> {
        vec![self.position, (self.position.0, self.position.1 + 1)]
    }
    
    fn score(&self) -> usize {
        (100 * self.position.0 + self.position.1) as usize
    }
}

struct StretchedRoom {
    boxes: Vec<Box>,
    walls: Vec<XY>,
    robot: XY
}

impl StretchedRoom {
    fn new(room: Room) -> Self {
        let mut boxes = Vec::new();
        let mut walls = Vec::new();
        let mut robot = (-1,-1);
        for (r, tiles) in room.iter().enumerate() {
            for (c, tile) in tiles.iter().enumerate() {
                match tile {
                    Tile::Box => {
                        boxes.push(Box::new((r as isize, (c * 2)as isize)));
                    },
                    Tile::Wall => {
                        walls.push((r as isize, (c*2) as isize));
                        walls.push((r as isize, (c*2 + 1) as isize));

                    },
                    Tile::Robot => {
                        robot = (r as isize, (c * 2) as isize);
                    },
                    Tile::Empty => {}
                }
            }
        }
        Self {
            boxes,
            walls,
            robot
        }
    }
    
    fn move_robot(&mut self, m: &Move) {
        let delta = match m { 
            Move::Up => (-1, 0),
            Move::Down => (1, 0),
            Move::Left => (0, -1),
            Move::Right => (0, 1)
        };
        
        let potential_robot_position = (self.robot.0 + delta.0, self.robot.1 + delta.1);
        if self.walls.contains(&potential_robot_position) {
            return
        }
        
        let mut collisions = self.boxes.iter()
            .filter(|b| b.contains(potential_robot_position))
            .collect::<Vec<_>>();
        let mut moved = Vec::new();
        
        while let Some(b) = collisions.pop() {
            for p in b.edges() {
                let p_moved = (p.0 + delta.0, p.1 + delta.1);
                if self.walls.contains(&p_moved) { return; }
                for c in self.boxes.iter().filter(|other| !other.eq(&b) && other.contains(p_moved)) {
                    collisions.push(c);
                }
                moved.push(b.position);
            }
        }
        
        self.robot = potential_robot_position;
        for b in self.boxes.iter_mut().filter(|b| moved.contains(&b.position)) {
            b.position = (b.position.0 + delta.0, b.position.1 + delta.1);
        }
    }
    
    fn score(&self) -> usize {
        let mut score = 0;
        for b in &self.boxes {
            score += b.score();
        }
        score
    }
}

fn part2(filename: &str) -> usize {
    let (room, moves) = parse(filename);
    let mut stretched_room = StretchedRoom::new(room);
    for m in moves {
        stretched_room.move_robot(&m);
    }
    stretched_room.score()
}

#[cfg(test)]
mod tests {
    mod parse {
        use crate::year2024day15::parse;
        #[test]
        fn test_parse() {
            let (room, moves) = parse("input/2024-15-e2.txt");
            assert_eq!(8, room.len());
            assert_eq!(15, moves.len());
        }
    }
    
    mod part1 {
        use crate::year2024day15::part1;
        #[test]
        fn example1() {
            assert_eq!(10092, part1("input/2024-15-e1.txt"));
        }
        #[test]
        fn example2() {
            assert_eq!(2028, part1("input/2024-15-e2.txt"));
        }
        #[test]
        fn actual() {
            assert_eq!(1383666, part1("input/2024-15-input.txt"));
        }
    }
    
    mod part2 {
        use crate::year2024day15::part2;

        #[test]
        fn example() {
            assert_eq!(9021, part2("input/2024-15-e1.txt"));
        }
        
        #[test]
        fn solution() {
            assert_eq!(1412866, part2("input/2024-15-input.txt"));}
    }
}