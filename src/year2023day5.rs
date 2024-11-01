mod year2023day5 {
    use std::ops::Range;
    use crate::read_lines;

    #[derive(PartialEq)]
    #[derive(Debug)]
    #[derive(Clone)]
    struct RangeDelta {
        range: Range<usize>,
        delta: isize,
    }

    impl RangeDelta {
        fn parse(l: &str) -> RangeDelta {
            let mut iter = l.trim().split_whitespace();
            let dest_start = iter.next().unwrap().parse::<usize>().unwrap();
            let source_start = iter.next().unwrap().parse::<usize>().unwrap();
            let length = iter.next().unwrap().parse::<usize>().unwrap();

            RangeDelta {
                range: source_start..source_start + length,
                delta: dest_start as isize - source_start as isize,
            }
        }
    }

    #[derive(PartialEq)]
    #[derive(Debug)]
    struct Layer {
        ranges: Vec<RangeDelta>,
    }

    #[derive(PartialEq)]
    #[derive(Debug)]
    struct Input {
        seeds: Vec<usize>,
        layers: Vec<Layer>,
    }

    impl Input {
        fn parse(file_name: &str) -> Input {
            let mut lines = read_lines(file_name).into_iter();

            // parse seed line
            let mut seeds = Vec::new();
            if let Some(l) = lines.next() {
                for d in l.unwrap().trim().split_whitespace() {
                    if let Ok(d) = d.parse::<usize>() {
                        seeds.push(d);
                    }
                }
            }
            let mut layers = Vec::new();
            let mut name: Option<String> = None;
            let mut ranges = Vec::new();
            while let Some(l) = lines.next() {
                match l.unwrap() {
                    e if e.is_empty() => {
                        if name.is_some() {
                            layers.push(Layer { ranges: ranges.clone() });
                            name = None;
                            ranges = Vec::new();
                        }
                    }
                    n if n.ends_with(':') => {
                        name = Some(n[..n.len() - " map:".len()].to_string());
                    }
                    r => {
                        ranges.push(RangeDelta::parse(&r));
                    }
                }
            }
            if name.is_some() {
                layers.push(Layer { ranges: ranges.clone() });
            }

            Input {
                seeds,
                layers,
            }
        }
        
        fn part1(&self) -> isize {
            let mut lowest = isize::MAX;
            for seed in &self.seeds {
                let mut location = seed.clone() as isize;
                for layer in &self.layers {
                    let delta = layer.ranges.iter().find(|rd| {
                        rd.range.contains(&(location as usize))
                    }).map(|rd| { rd.delta }).unwrap_or(0);
                    location += delta;
                }
                if location < lowest {
                    lowest = location
                }
            }
            lowest
        }
    }

    #[cfg(test)]
    mod tests {
        mod parse {
            use crate::year2023day5::year2023day5::{Input, Layer, RangeDelta};

            #[test]
            fn handle_parse_range_delta() {
                let parsed = RangeDelta::parse("50 98 2");
                let expected = RangeDelta {
                    range: 98..100,
                    delta: -48,
                };
                assert_eq!(parsed, expected);
            }

            #[test]
            fn handle_parse_input() {
                let parsed = Input::parse("input/2023-05-e1.txt");
                let expected = Input {
                    seeds: vec![79, 14, 55, 13],
                    layers: Vec::new(),
                };
                assert_eq!(expected.seeds, parsed.seeds);

                let expected_first_layer = Layer {
                    ranges: vec![RangeDelta {
                        range: 98..100,
                        delta: -48,
                    }, RangeDelta {
                        range: 50..98,
                        delta: 2,
                    }]
                };
                assert_eq!(&expected_first_layer, parsed.layers.first().unwrap());
                assert_eq!(7, parsed.layers.len());
            }
        }
        
        mod part1 {
            use crate::year2023day5::year2023day5::Input;

            #[test]
            fn example() {
                let input = Input::parse("input/2023-05-e1.txt");
                assert_eq!(35, input.part1());
            }

            #[test]
            fn solution() {
                let input = Input::parse("input/2023-05-input.txt");
                assert_eq!(600279879, input.part1());
            }
        }
    }
}