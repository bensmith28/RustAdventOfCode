mod year2024day9 {
    use std::collections::HashSet;
    use crate::read_string;
    
    struct File {
        id: usize,
        length: usize,
    }

    fn part1(filename: &str) -> usize {
        let diskmap = read_string(filename)
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>();
        let mut checksum = 0;

        let mut pointer_next = 0;
        let mut map_consumed = diskmap.len();
        let mut map_index = 0;
        
        let mut last_file = File {
            id: usize::MAX,
            length: 0
        };
        
        while map_index < map_consumed {
            let length = diskmap[map_index];
            if map_index % 2 == 0 {
                // Map of file
                let id = map_index / 2;
                for i in pointer_next..pointer_next + length {
                    checksum += i * id;
                }
                pointer_next += length;
            } else {
                // Map of free space
                'consume_free_space: for i in pointer_next..pointer_next + length {
                    if last_file.length == 0 {
                        // take next file off the diskmap
                        'move_block_pointer: loop {
                            map_consumed -= 1;
                            if map_consumed <= map_index { break 'consume_free_space; }
                            if map_consumed % 2 == 0 { break 'move_block_pointer; }
                        }
                        last_file = File {
                            id: map_consumed / 2,
                            length: diskmap[map_consumed]
                        }
                    }
                    last_file.length -= 1;

                    checksum += i * last_file.id;
                }
                pointer_next += length;
            }
            map_index += 1;
        }
        while last_file.length > 0 {
            checksum += last_file.id * pointer_next;
            pointer_next += 1;
            last_file.length -= 1;
        }

        checksum
    }
    
    fn part2(filename: &str) -> usize {
        let diskmap = read_string(filename)
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>();
        let mut checksum = 0;

        let mut pointer_next = 0;
        let mut map_index = 0;
        
        let mut files_consumed: HashSet<usize> = HashSet::new();
        
        while map_index < diskmap.len() {
            let length = diskmap[map_index];
            if map_index % 2 != 0 || files_consumed.contains(&(map_index / 2)) {
                // free
                let mut length = length;
                if map_index % 2 == 0 && map_index + 1 < diskmap.len() {
                    length += diskmap[map_index + 1];
                    map_index += 1;
                }
                let mut scan = diskmap.len() - 1;
                while length > 0  && scan > map_index {
                    'look_for_a_file: while scan > map_index {
                        if scan % 2 != 0 { 
                            scan -= 1;
                            continue 
                        }
                        let scan_id = scan / 2;
                        if files_consumed.contains(&scan_id) { 
                            scan -= 1;
                            continue 
                        }
                        let scan_length = diskmap[scan];
                        scan -= 1;
                        if scan_length <= length {
                            for i in pointer_next..pointer_next + scan_length {
                                checksum += i * scan_id;
                            }
                            pointer_next += scan_length;
                            files_consumed.insert(scan_id);
                            length -= scan_length;
                            scan = diskmap.len() - 1;
                            break 'look_for_a_file;
                        }
                    }
                }
                pointer_next += length;
            } else {
                // file
                let id = map_index / 2;
                for i in pointer_next..pointer_next + length {
                    checksum += i * id;
                }
                pointer_next += length;
            }
            map_index += 1;
        }
        
        checksum
    }
    
    #[cfg(test)]
    mod tests {
        mod part1 {
            use crate::year2024day9::year2024day9::part1;

            #[test]
            fn example1() {
                // 022111222
                // 012345678
                // 0
                //  2
                //   4
                //    3
                //     4
                //      5
                //       12
                //        14
                //         16
                // 60
                assert_eq!(60, part1("input/2024-09-e1.txt"))
            }
            
            #[test]
            fn example2() {
                assert_eq!(1928, part1("input/2024-09-e2.txt"))
            }
            
            #[test]
            fn solution() {
                assert_eq!(6307275788409, part1("input/2024-09-input.txt"))
            }
        }
        
        mod part2 {
            use crate::year2024day9::year2024day9::part2;

            #[test]
            fn example1() {
                // 0..111....22222
                // 012345678901234
                // 000
                //    3
                //     4
                //      5
                //       0000
                //           20
                //            22
                //             24
                //              26
                //               28
                // 132
                assert_eq!(132, part2("input/2024-09-e1.txt"))
            }
            
            #[test]
            fn example2() {
                // TODO scan from the right, dummy
                assert_eq!(2858, part2("input/2024-09-e2.txt"))
            }
        }
    }
}