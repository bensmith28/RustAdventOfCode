mod year2023day6 {
    use crate::read_lines;

    #[derive(PartialEq)]
    #[derive(Debug)]
    struct RaceRecord {
        time: usize,
        distance: usize
    }

    fn parse_sheet(filename: &str) -> Vec<RaceRecord> {
        let mut lines = read_lines(filename);
        let time_line = lines.next().unwrap().unwrap();
        let mut time_iter = time_line.split_whitespace();
        let distance_line = lines.next().unwrap().unwrap();
        let mut distance_iter = distance_line.split_whitespace();

        // Drop line headers 'Time:' & 'Distance:'
        time_iter.next();
        distance_iter.next();

        let mut sheet = Vec::new();

        while let (Some(time), Some(distance)) = (time_iter.next(), distance_iter.next()) {
            sheet.push(RaceRecord {
                time: time.parse().unwrap(),
                distance: distance.parse().unwrap()
            });
        }

        sheet
    }

    fn parse_sheet_part2(filename: &str) -> RaceRecord {
        let mut lines = read_lines(filename);
        let time_line = lines.next().unwrap().unwrap();
        let mut time_iter = time_line.split_whitespace();
        let distance_line = lines.next().unwrap().unwrap();
        let mut distance_iter = distance_line.split_whitespace();

        // Drop line headers 'Time:' & 'Distance:'
        time_iter.next();
        distance_iter.next();
        
        let mut time_str = String::new();
        let mut distance_str = String::new();

        while let (Some(time), Some(distance)) = (time_iter.next(), distance_iter.next()) {
            time_str.push_str(time.trim());
            distance_str.push_str(distance.trim());
        }
        
        RaceRecord {
            time: time_str.parse().unwrap(),
            distance: distance_str.parse().unwrap()
        }
    }
    
    fn does_it_win(race_record: &RaceRecord, hold: usize) -> bool {
        (hold * (race_record.time - hold)) > race_record.distance
    }
    
    fn part1(filename: &str) -> usize {
        let sheet = parse_sheet(filename);
        ways_to_win(sheet)
    }
    
    fn part2(filename: &str) -> usize {
        let sheet = parse_sheet_part2(filename);
        ways_to_win(vec![sheet])
    }
    
    fn ways_to_win(sheet: Vec<RaceRecord>) -> usize {
        let mut ways_to_win = 1;
        for race in sheet.iter() {
            // y = ax^2 + bx + c
            // y: distance travelled by toy boat
            // x: time held on button
            // y = x * (race.time - x)
            // y = -xx + race.time*x
            // dy/dx = -2x + race.time
            // x = race.time/2
            
            let min_open = race.distance / race.time;
            let mut min_speed = 0;
            for hold in min_open..race.time {
                if does_it_win(race, hold) {
                    min_speed = hold;
                    break;
                }
            }
            let peak = race.time / 2;
            let max_open = peak + (peak - min_speed);
            let mut max_speed = 0;
            for hold in max_open..race.time {
                if !does_it_win(race, hold) {
                    max_speed = hold - 1;
                    break;
                }
            }
            
            let wins = max_speed - min_speed + 1;
            ways_to_win = ways_to_win * wins;
        }
        
        ways_to_win
    }

    #[cfg(test)]
    mod tests {
        mod parse {
            use crate::year2023day6::year2023day6::{parse_sheet, parse_sheet_part2, RaceRecord};

            #[test]
            fn part1() {
                let actual = parse_sheet("input/2023-06-e1.txt");
                let expected = vec![
                    RaceRecord {
                        time: 7, distance: 9
                    },
                    RaceRecord {
                        time: 15, distance: 40
                    },
                    RaceRecord {
                        time: 30, distance: 200
                    }
                ];
                
                assert_eq!(actual, expected);
            }
            
            #[test]
            fn part2() {
                let actual = parse_sheet_part2("input/2023-06-e1.txt");
                let expected = RaceRecord {
                    time: 71530, distance: 940200
                };
                assert_eq!(actual, expected);
            }
        }
        
        mod part1 {
            use crate::year2023day6::year2023day6::part1;

            #[test]
            fn example() {
                let actual = part1("input/2023-06-e1.txt");
                let expected = 288;
                assert_eq!(actual, expected);
            }
            
            #[test]
            fn solution() {
                let actual = part1("input/2023-06-input.txt");
                let expected = 1084752;
                assert_eq!(actual, expected);
            }
        }
        
        mod part2 {
            use crate::year2023day6::year2023day6::part2;

            #[test]
            fn example() {
                let actual = part2("input/2023-06-e1.txt");
                let expected = 71503;
                assert_eq!(actual, expected);
            }

            #[test]
            fn solution() {
                let actual = part2("input/2023-06-input.txt");
                let expected = 28228952;
                assert_eq!(actual, expected);
            }
        }
    }
}