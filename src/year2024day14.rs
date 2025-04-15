use crate::read_lines;
use regex::Regex;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;

type XY = (isize, isize);

struct InitialRobot {
    position: XY,
    velocity: XY
}

fn parse(filenamee: &str) -> Vec<InitialRobot> {
    let mut robots = Vec::new();
    let pattern = Regex::new(r"^p=([+\-\d]+),([+\-\d]+) v=([+\-\d]+),([+\-\d]+)$").unwrap();
    let mut lines = read_lines(filenamee);
    while let Some(Ok(line)) = lines.next() {
        let captures = pattern.captures(line.as_str()).unwrap();
        robots.push(InitialRobot {
            position: (captures[1].parse().unwrap(), captures[2].parse().unwrap()),
            velocity: (captures[3].parse().unwrap(), captures[4].parse().unwrap())
        })
    }
    
    robots
}

fn part1(filename: &str, room: XY) -> usize {
    let robots = parse(filename);
    let mid_x = room.0 / 2;
    let mid_y = room.1 / 2;
    let quadrants = robots.iter().filter_map(|robot| {
        let mut x = (robot.position.0 + robot.velocity.0 * 100) % room.0;
        while x < 0 { x += room.0 }
        let mut y = (robot.position.1 + robot.velocity.1 * 100) % room.1;
        while y < 0 { y += room.1 }
        if x < mid_x && y < mid_y {
            Some(1)
        } else if x < mid_x && y > mid_y {
            Some(2)
        } else if x > mid_x && y < mid_y {
            Some(3)
        } else if x > mid_x && y > mid_y {
            Some(4)
        } else {
            None
        }
    }).fold(HashMap::new(), |mut map, q| {
        map.entry(q)
            .and_modify(|e| *e += 1)
            .or_insert(1);
        map
    });
    quadrants.values().product::<isize>() as usize
}

fn step(robot: &InitialRobot, n: isize, room: XY) -> XY {
    let mut x = (robot.position.0 + robot.velocity.0 * n) % room.0;
    while x < 0 { x += room.0 }
    let mut y = (robot.position.1 + robot.velocity.1 * n) % room.1;
    while y < 0 { y += room.1 }
    (x,y)
}

fn print_to_file(file: &mut File, room: (isize, isize), positions: Vec<XY>) {
    for y in 0..room.1 {
        for x in 0..room.0 {
            if positions.iter().any(|p| p.0 == x && p.1 == y) {
                write!(file, "#").unwrap();
            } else {
                write!(file, " ").unwrap();
            }
        }
        writeln!(file).unwrap();
    }
    io::stdout().flush().unwrap();
}

fn tree_score(positions: &[XY]) -> usize {
    let mut adjacent_bots = 0;
    for (index, bot) in positions.iter().enumerate() {
        for other in positions.iter().skip(index+1) {
            if bot.0.abs_diff(other.0) <= 1 && bot.1.abs_diff(other.1) <= 1 {
                adjacent_bots += 1;
            }
        }
    }
    adjacent_bots
    // old version
    // let quadrants = positions.iter().map(|p| {
    //     let x = p.0;
    //     if x <= room.0 / 4 {
    //         1
    //     } else if x <= room.0 / 2 {
    //         2
    //     } else if x <= room.0 * 3 / 4 {
    //         3
    //     } else  {
    //         4
    //     }
    // }).fold(HashMap::new(), |mut map, q| {
    //     map.entry(q)
    //         .and_modify(|e| *e += 1)
    //         .or_insert(1usize);
    //     map
    // });
    // 
    // let dif_1_4 = 1000 * (quadrants[&1].abs_diff(quadrants[&4])) / quadrants[&1];
    // if dif_1_4 > 100 {
    //     return 0;
    // }
    // let dif_2_3 = 1000 * (quadrants[&2].abs_diff(quadrants[&3])) / quadrants[&2];
    // if dif_2_3 > 100 {
    //     return 0
    // }
    // // score is dif_1_2
    // 1000 * (quadrants[&1].abs_diff(quadrants[&2])) / quadrants[&1]
}

fn part2() {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("debug_output.txt")
        .unwrap();
    let robots = parse("input/2024-14-input.txt");
    let room = (101, 103);
    let mut winners = Vec::new();
    struct Winner {
        positions: Vec<XY>,
        score: usize,
        seconds: isize
    }
    let duration = 10000;
    for seconds in 0..duration {
        if seconds % (duration / 100) == 0 {
            println!("{:?} seconds of {:?}", seconds, duration);
        }
        let positions = robots.iter().map(|robot| {
            step(robot, seconds, room)
        }).collect::<Vec<_>>();
        let score = tree_score(&positions);
        let w = Winner {
            positions, score, seconds
        };
        winners.push(w);
        winners.sort_by_key(|w| w.score);
        while winners.len() > 10 {
            winners.remove(0);
        }
    }
    
    for w in winners {
        writeln!(file, "---------------------------------------------------------------------------").unwrap();
        writeln!(file, "Found at {:?} seconds", w.seconds).unwrap();
        print_to_file(&mut file, room, w.positions)
    }
}

#[cfg(test)]
mod tests {
    mod part1 {
        use crate::year2024day14::{parse, part1};

        #[test]
        fn test_parse() {
            let robots = parse("input/2024-14-e1.txt");
            assert_eq!(12, robots.len());
        }
        
        #[test]
        fn example() {
            let actual = part1("input/2024-14-e1.txt", (11, 7));
            assert_eq!(12, actual);
        }
        
        #[test]
        fn solution() {
            let actual = part1("input/2024-14-input.txt", (101, 103));
            assert_eq!(228410028, actual);
        }
    }
    
    mod part2 {
        use crate::year2024day14::part2;

        #[test]
        fn test() {
            part2()
            // found it at 8258 seconds
        }
    }
}