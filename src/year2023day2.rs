mod year2023_day2 {
    use std::cmp::max;

    #[derive(Debug)]
    #[derive(PartialEq)]
    struct Game {
        id: usize,
        hands: Vec<Hand>,
    }

    impl Game {
        fn parse(s: &str) -> Game {
            let mut input = s.to_string();
            input.drain(..5);
            let colon_i = input.find(':').unwrap();
            let id: usize = input.drain(..colon_i).as_str().parse().unwrap();
            input.drain(..2);
            let mut hands: Vec<Hand> = vec![];
            while !input.is_empty() {
                let end_i = match input.find(';') {
                    Some(i) => i + 2,
                    None => input.len(),
                };
                let hand = Hand::parse(input.drain(..end_i).as_str());
                hands.push(hand);
            }

            Game { id, hands }
        }
        
        fn power(&self) -> usize {
            let zeros = Hand {
                red: 0,
                blue: 0,
                green: 0
            };
            self.hands.iter().fold(zeros, |acc, hand| Hand {
                red: max(acc.red, hand.red),
                blue: max(acc.blue, hand.blue),
                green: max(acc.green, hand.green)
            }).power()
        }
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    struct Hand {
        blue: usize,
        red: usize,
        green: usize,
    }

    impl Hand {
        fn parse(s: &str) -> Hand {
            let mut input = s.to_string();
            let mut blue = 0usize;
            let mut red = 0usize;
            let mut green = 0usize;
            while !input.is_empty() {
                let space_i = input.find(' ').unwrap();
                let count: usize = input.drain(..space_i).as_str().parse().unwrap();
                input.drain(..1);
                let punc_i = input.find(|c: char| { c == ',' || c == ';' }).unwrap_or_else(|| input.len());
                match input.drain(..punc_i).as_str() {
                    "blue" => blue += count,
                    "red" => red += count,
                    "green" => green += count,
                    c => panic!("Bad Color match: {}", c)
                }
                match input.find(' ') {
                    Some(i) => {
                        input.drain(..=i);
                    }
                    None => {
                        input.drain(..input.len());
                    }
                }
            }
            Hand { blue, red, green }
        }

        fn possible_draw(&self, other: &Hand) -> bool {
            other.red <= self.red && other.green <= self.green && other.blue <= self.blue
        }
        
        fn power(&self) -> usize {
            self.red * self.blue * self.green
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        mod parse {
            use super::*;

            #[test]
            fn handles_hand_input() {
                let hand1 = Hand::parse("3 blue, 4 red");
                assert_eq!(hand1, Hand { blue: 3, red: 4, green: 0 });

                let hand2 = Hand::parse("1 red, 2 green, 6 blue");
                assert_eq!(hand2, Hand { blue: 6, red: 1, green: 2 });
            }

            #[test]
            fn handles_hand_punctuation() {
                let hand1 = Hand::parse("3 blue, 4 red; ");
                assert_eq!(hand1, Hand { blue: 3, red: 4, green: 0 });
            }

            #[test]
            fn handles_game_input() {
                let game1 = Game::parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
                let exptected1 = Game {
                    id: 1,
                    hands: vec![Hand {
                        blue: 3,
                        red: 4,
                        green: 0
                    },
                                Hand {
                                    red: 1,
                                    green: 2,
                                    blue: 6
                                },
                                Hand {
                                    red: 0,
                                    blue: 0,
                                    green: 2
                                }]
                };

                assert_eq!(game1, exptected1);
            }
        }
        mod part1 {
            use crate::read_lines;
            use crate::year2023day2::year2023_day2::{Game, Hand};
            
            const BAG: Hand = Hand {
                red: 12,
                green: 13,
                blue: 14,
            };

            #[test]
            fn example() {
                let input = read_lines("input/2023-02-e1.txt");
                let result = input.map(|line| Game::parse(&line.unwrap()))
                    .filter(|game| game.hands.iter().all(|hand| BAG.possible_draw(hand)))
                    .fold(0, |acc, game| acc + game.id);
                
                assert_eq!(result, 8);
            }

            #[test]
            fn solution() {
                let input = read_lines("input/2023-02-input.txt");
                let result = input.map(|line| Game::parse(&line.unwrap()))
                    .filter(|game| game.hands.iter().all(|hand| BAG.possible_draw(hand)))
                    .fold(0, |acc, game| acc + game.id);

                assert_eq!(result, 2169);
            }
        }
        
        mod part2 {
            use crate::read_lines;
            use crate::year2023day2::year2023_day2::Game;

            #[test]
            fn example() {
                let input = read_lines("input/2023-02-e1.txt");
                let games = input.map(|line| Game::parse(&line.unwrap()));
                
                let result: usize = games.map(|game| game.power()).sum();
                assert_eq!(result, 2286);
            }

            #[test]
            fn solution() {
                let input = read_lines("input/2023-02-input.txt");
                let games = input.map(|line| Game::parse(&line.unwrap()));

                let result: usize = games.map(|game| game.power()).sum();
                assert_eq!(result, 60948);
            }
        }
    }
}