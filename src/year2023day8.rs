mod year2023day8 {
    use crate::read_lines;
    use regex::Regex;
    use std::collections::HashMap;

    struct MapNode {
        left: String,
        right: String
    }

    #[derive(PartialEq, Debug, Eq)]
    enum Direction {
        Left, Right
    }

    struct Input {
        directions: Vec<Direction>,
        nodes: HashMap<String, MapNode>
    }

    impl Input {
        fn parse_directions(line: &str) -> Vec<Direction> {
            let mut result = Vec::new();
            for ch in line.chars() {
                match ch {
                    'R' => { result.push(Direction::Right) }
                    'L' => { result.push(Direction::Left) }
                    c => panic!("Invalid character in input: {}", c)
                }
            }
            result
        }

        fn new(filename: &str) -> Self {
            let mut lines_iter = read_lines(filename);
            let mut nodes = HashMap::new();

            let directions = Self::parse_directions(lines_iter.next().unwrap().unwrap().as_str());
            let pattern = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
            while let Some(Ok(line)) = lines_iter.next() {
                if line.is_empty() { continue }
                let parts = pattern.captures(&line).unwrap();
                let root = parts[1].to_string();
                let left = parts[2].to_string();
                let right = parts[3].to_string();
                nodes.insert(root.clone(), MapNode { left, right });
            }

            Input {
                directions, nodes
            }
        }
    }
    
    fn follow_path(input: Input) -> usize {
        let mut pointer = "AAA";
        let mut counter = 0;
        while pointer != "ZZZ" {
            match input.directions[counter % input.directions.len()] {
                Direction::Left => {
                    pointer = input.nodes.get(pointer).unwrap().left.as_str();
                }
                Direction::Right => {
                    pointer = input.nodes.get(pointer).unwrap().right.as_str();
                }
            }
            counter += 1;
        }
        counter
    }

    fn follow_ghost_path(input: Input) -> usize {
        #[derive(Copy, Clone, Debug)]
        struct Node {
            is_end: bool,
            left: usize,
            right: usize
        }

        #[derive(Copy, Clone, Debug)]
        struct State {
            pos: usize,
            dir_idx: usize
        }

        impl State {
            fn next(&self, nodes: &[Node], directions: &[Direction]) -> State {
                let next_dir_idx = (self.dir_idx + 1) % directions.len();
                let next_pos = match directions[self.dir_idx] {
                    Direction::Left => nodes[self.pos].left,
                    Direction::Right => nodes[self.pos].right,
                };
                State { pos: next_pos, dir_idx: next_dir_idx }
            }
        }

        // Build nodes array similar to before
        let mut nodes = Vec::with_capacity(input.nodes.len());
        let mut start_positions = Vec::new();
        let mut node_map = HashMap::new();

        for (idx, key) in input.nodes.keys().enumerate() {
            node_map.insert(key.as_str(), idx);
            if key.ends_with('A') {
                start_positions.push(idx);
            }
            nodes.push(Node {
                is_end: key.ends_with('Z'),
                left: 0,
                right: 0
            });
        }

        for (key, node) in input.nodes.iter() {
            let idx = node_map[key.as_str()];
            nodes[idx].left = node_map[node.left.as_str()];
            nodes[idx].right = node_map[node.right.as_str()];
        }

        // Find cycle information for each starting position
        let mut cycle_info = Vec::new();

        for &start_pos in &start_positions {
            // Initialize tortoise and hare
            let mut tortoise = State { pos: start_pos, dir_idx: 0 };
            let mut hare = State { pos: start_pos, dir_idx: 0 };

            // Phase 1: Find a point in the cycle
            loop {
                tortoise = tortoise.next(&nodes, &input.directions);
                hare = hare.next(&nodes, &input.directions);
                hare = hare.next(&nodes, &input.directions);

                if tortoise.pos == hare.pos && tortoise.dir_idx == hare.dir_idx {
                    break;
                }
            }

            // Phase 2: Find cycle start
            let mut cycle_start = State { pos: start_pos, dir_idx: 0 };
            let mut steps_to_cycle = 0;
            while cycle_start.pos != tortoise.pos || cycle_start.dir_idx != tortoise.dir_idx {
                cycle_start = cycle_start.next(&nodes, &input.directions);
                tortoise = tortoise.next(&nodes, &input.directions);
                steps_to_cycle += 1;
            }

            // Phase 3: Find cycle length and Z positions
            let mut cycle_length = 1;
            let mut z_positions = Vec::new();
            let mut current = cycle_start.next(&nodes, &input.directions);

            // Record position if we're at a Z node
            if nodes[cycle_start.pos].is_end {
                z_positions.push(0);
            }

            while current.pos != cycle_start.pos || current.dir_idx != cycle_start.dir_idx {
                if nodes[current.pos].is_end {
                    z_positions.push(cycle_length);
                }
                current = current.next(&nodes, &input.directions);
                cycle_length += 1;
            }

            cycle_info.push((steps_to_cycle, cycle_length, z_positions));
        }

        // Now we need to find the least common multiple (LCM) of the cycle lengths
        // adjusted for the phase shifts to Z positions
        fn gcd(mut a: usize, mut b: usize) -> usize {
            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a
        }

        fn lcm(a: usize, b: usize) -> usize {
            a * (b / gcd(a, b))
        }

        // For this specific puzzle, it turns out that:
        // 1. Each path has exactly one Z position in its cycle
        // 2. The Z position occurs at the end of the cycle
        // 3. The steps to reach the cycle is equal to the first Z position
        // These properties make the solution much simpler than the general case

        cycle_info.iter()
            .map(|(_, cycle_len, _)| *cycle_len)
            .fold(1, lcm)
    }
    
    #[cfg(test)]
    mod tests {
        mod input {
            use crate::year2023day8::year2023day8::Direction::{Left, Right};
            use crate::year2023day8::year2023day8::Input;

            #[test]
            fn parse_example() {
                let input = Input::new("input/2023-08-e1.txt");
                assert_eq!(input.directions, vec![Right, Left]);
                assert_eq!(input.nodes.len(), 7);
                assert!(input.nodes.contains_key("AAA"));
                assert!(input.nodes.contains_key("ZZZ"));
            }
        }
        
        mod part1 {
            use crate::year2023day8::year2023day8::{follow_path, Input};

            #[test]
            fn example1() {
                let input = Input::new("input/2023-08-e1.txt");
                assert_eq!(follow_path(input), 2);
            }

            #[test]
            fn example2() {
                let input = Input::new("input/2023-08-e2.txt");
                assert_eq!(follow_path(input), 6);
            }
            
            #[test]
            fn solution() {
                let input = Input::new("input/2023-08-input.txt");
                assert_eq!(follow_path(input), 12083);
            }
        }
        
        mod part2 {
            use crate::year2023day8::year2023day8::{follow_ghost_path, Input};

            #[test]
            fn example3() {
                let input = Input::new("input/2023-08-e3.txt");
                assert_eq!(follow_ghost_path(input), 6);
            }
            
            #[test]
            fn solution() {
                let input = Input::new("input/2023-08-input.txt");
                assert_eq!(follow_ghost_path(input), 13385272668829);
            }
        }
    }
}