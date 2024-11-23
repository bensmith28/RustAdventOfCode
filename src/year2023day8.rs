mod year2023day8 {
    use crate::read_lines;
    use regex::Regex;
    use std::collections::HashMap;

    struct MapNode {
        root: String,
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
                nodes.insert(root.clone(), MapNode { root, left, right });
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
        struct Link {
            root: String,
            left: usize,
            right: usize,
            is_end: bool
        }
        
        let mut links = Vec::with_capacity(input.nodes.len());
        
        for (n, _) in input.nodes.iter() {
            links.push(Link {
                root: n.to_string(),
                left: 0,
                right: 0,
                is_end: n.ends_with("Z")
            })
        }
        let mut positions: Vec<_> = Vec::new();
        for (_, node) in input.nodes.iter() {
            let left_idx = links.iter().position(|l| l.root == node.left).unwrap();
            let right_idx = links.iter().position(|l| l.root == node.right).unwrap();
            let root_idx = links.iter().position(|l| l.root == node.root).unwrap();

            links[root_idx].left = left_idx;
            links[root_idx].right = right_idx;
            
            if node.root.ends_with("A") {
                positions.push(root_idx);
            }
        }
        
        let mut counter = 0;
        
        loop {
            if positions.iter().all(|&pos| links[pos].is_end) {
                break;
            }

            match &input.directions[counter % positions.len()] {
                Direction::Left => {
                    for p in positions.iter_mut() {
                        *p = links[*p].left
                    }
                }
                Direction::Right => {
                    for p in positions.iter_mut() {
                        *p = links[*p].right
                    }
                }
            }
            counter += 1;
        }
        
        counter
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
                assert_eq!(follow_ghost_path(input), 12083);
            }
        }
    }
}