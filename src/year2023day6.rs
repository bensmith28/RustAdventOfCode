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
    
    fn does_it_win(race_record: &RaceRecord, hold: usize) -> bool {
        (hold * (race_record.time - hold)) > race_record.distance
    }
    
    fn part1(filename: &str) -> usize {
        let sheet = parse_sheet(filename);
        let mut ways_to_win = 1;
        for race in sheet.iter() {
            let mut wins = 0;
            for hold in 0..race.time {
                if does_it_win(race, hold) {
                    wins += 1;
                }
            }
            ways_to_win = ways_to_win * wins;
        }
        
        ways_to_win
    }

    #[cfg(test)]
    mod tests {
        mod parse {
            use crate::year2023day6::year2023day6::{parse_sheet, RaceRecord};

            #[test]
            fn example() {
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
    }
}