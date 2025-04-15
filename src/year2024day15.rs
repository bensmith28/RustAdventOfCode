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
fn part2() {}

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
}