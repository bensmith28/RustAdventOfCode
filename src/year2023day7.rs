mod year2023day7 {
    use std::cmp::Ordering;
    use std::cmp::Ordering::Equal;
    use std::collections::HashMap;
    use std::str::FromStr;
    use crate::read_lines;
    use crate::year2023day7::year2023day7::HandType::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

    #[derive(PartialEq, Debug, Eq)]
    struct Hand {
        cards: Vec<char>,
        bid: usize
    }

    #[derive(PartialEq, Debug, Ord, PartialOrd, Eq)]
    enum HandType {
        FiveOfAKind,
        FourOfAKind,
        FullHouse,
        ThreeOfAKind,
        TwoPair,
        OnePair,
        HighCard
    }

    impl Hand {
        fn hand_type(&self) -> HandType {
            let mut count_map: HashMap<char, usize> = HashMap::new();
            for card in &self.cards {
                count_map.entry(*card).and_modify(|v| *v += 1).or_insert(1);
            }
            let wilds = *count_map.get(&'W').unwrap_or(&0usize);
            let mut counts = count_map.into_values().collect::<Vec<_>>();
            counts.push(0); // always have at least two values so the match line doesn't panic
            counts.sort();
            counts.reverse();
            match (counts[0], counts[1]) {
                (5, _) => FiveOfAKind,
                (4, _) => match wilds {
                    1 | 4 => FiveOfAKind,
                    0 => FourOfAKind,
                    _ => unreachable!("How'd we get here?")
                },
                (3, 2) => match wilds {
                    3 | 2 => FiveOfAKind,
                    0 => FullHouse,
                    _ => unreachable!("How'd we get here?")
                },
                (3, other) => match (wilds, other) {
                    (3, 2) => unreachable!("covered by full house"),
                    (3, 1) => FourOfAKind,
                    (3, 0) => ThreeOfAKind,
                    (2, _) => unreachable!("covered by full house"),
                    (1, _) => FourOfAKind,
                    (0, _) => ThreeOfAKind,
                    (_,_) => unreachable!("How'd we get here?")
                },
                (2, 2) => match wilds {
                    2 => FourOfAKind,
                    1 => FullHouse,
                    0 => TwoPair,
                    _ => unreachable!("How'd we get here?")
                },
                (2, _) => match wilds {
                    1 | 2 => ThreeOfAKind,
                    0 => OnePair,
                    _ => unreachable!("How'd we get here?")
                },
                (_, _) => match wilds {
                    1 => OnePair,
                    0 => HighCard,
                    _ => unreachable!("How'd we get here?")
                }
            }
        }

        fn card_to_score(card: &char) -> usize {
            // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
            match card {
                'A' => 0,
                'K' => 1,
                'Q' => 2,
                'J' => 3,
                'T' => 4,
                '9' => 5,
                '8' => 6,
                '7' => 7,
                '6' => 8,
                '5' => 9,
                '4' => 10,
                '3' => 11,
                '2' => 12,
                'W' => 13,
                _ => unreachable!()
            }
        }
    }

    impl FromStr for Hand {
        type Err = &'static str;

        fn from_str(value: &str) -> Result<Self, Self::Err> {
            let mut split = value.split(' ');
            if let (Some(cards_str), Some(bid_str)) = (split.next(), split.next()) {
                let mut cards = Vec::new();
                for card_char in cards_str.chars() {
                    cards.push(card_char);
                }
                let bid = bid_str.trim().parse::<usize>().unwrap();
                Ok(Hand {
                    cards: cards,
                    bid: bid
                })
            } else {
                Err("Invalid hand")
            }
        }
    }

    impl PartialOrd<Self> for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            let type_cmp = self.hand_type().cmp(&other.hand_type());
            if type_cmp != Equal {
                return type_cmp;
            }
            let mut self_cards = self.cards.iter();
            let mut other_cards = other.cards.iter();
            while let (Some(s), Some(o)) = (self_cards.next(), other_cards.next()) {
                let cmp = Hand::card_to_score(s).cmp(&Hand::card_to_score(o));
                if cmp != Equal {
                    return cmp;
                }
            }

            Equal
        }
    }

    fn parse_input(filename: &str, jacks_wild: bool) -> Vec<Hand> {
        let mut result = Vec::new();
        for line in read_lines(filename) {
            let mut line = line.unwrap();
            if jacks_wild {
                line = line.replace("J", "W");
            }
            result.push(line.parse().unwrap());
        }

        result
    }

    fn total_winnings(mut hands: Vec<Hand>) -> usize {
        let mut sum = 0usize;
        hands.sort();
        for (i, hand) in hands.iter().rev().enumerate() {
            sum += (i + 1) * hand.bid;
        }

        sum
    }

    #[cfg(test)]
    mod tests {
        mod parse {
            use crate::year2023day7::year2023day7::{parse_input, Hand};

            #[test]
            fn parse_hand() {
                let hand_str = "32T3K 765";
                let actual: Hand = hand_str.parse().unwrap();
                let expected = Hand {
                    cards: vec!['3', '2', 'T', '3', 'K'],
                    bid: 765
                };
                assert_eq!(actual, expected);
            }

            #[test]
            fn handle_input() {
                let actual = parse_input("input/2023-07-e1.txt", false);
                assert_eq!(5, actual.len());
            }
        }

        mod hand_type {
            use crate::year2023day7::year2023day7::{parse_input, Hand};
            use crate::year2023day7::year2023day7::HandType::{OnePair, ThreeOfAKind, TwoPair};

            #[test]
            fn handle_hand_type() {
                let hand: Hand = "32T3K 765".parse().unwrap();
                let actual = hand.hand_type();
                let expected = OnePair;
                assert_eq!(expected, actual);
            }

            #[test]
            fn example() {
                let actual = parse_input("input/2023-07-e1.txt", false)
                    .iter().map(|h| h.hand_type()).collect::<Vec<_>>();
                let expected = vec![OnePair, ThreeOfAKind, TwoPair, TwoPair, ThreeOfAKind];
                assert_eq!(actual, expected);
            }
        }
        mod sort {
            use crate::year2023day7::year2023day7::HandType::*;
            use crate::year2023day7::year2023day7::{parse_input, Hand};

            #[test]
            fn example_sort_hand_types() {
                let mut actual = parse_input("input/2023-07-e1.txt", false)
                    .iter().map(|h| h.hand_type()).collect::<Vec<_>>();
                actual.sort();
                let expected = vec![ThreeOfAKind, ThreeOfAKind, TwoPair, TwoPair, OnePair];
                assert_eq!(actual, expected);
            }

            #[test]
            fn sort_ex1() {
                let mut actual: Vec<Hand> = vec![
                    "2AAAA 0".parse().unwrap(),
                    "33332 0".parse().unwrap(),
                ];
                actual.sort();
                let expected: Vec<Hand> = vec![
                    "33332 0".parse().unwrap(),
                    "2AAAA 0".parse().unwrap(),
                ];
                assert_eq!(actual, expected);
            }

            #[test]
            fn sort_ex2() {
                let mut actual: Vec<Hand> = vec![
                    "77788 0".parse().unwrap(),
                    "77888 0".parse().unwrap(),
                ];
                actual.sort();
                let expected: Vec<Hand> = vec![
                    "77888 0".parse().unwrap(),
                    "77788 0".parse().unwrap(),
                ];
                assert_eq!(actual, expected);
            }
        }
        
        mod part1 {
            use crate::year2023day7::year2023day7::{parse_input, total_winnings};

            #[test]
            fn example() {
                let hands = parse_input("input/2023-07-e1.txt", false);
                let actual = total_winnings(hands);
                assert_eq!(6440, actual);
            }

            #[test]
            fn solution() {
                let hands = parse_input("input/2023-07-input.txt", false);
                let actual = total_winnings(hands);
                assert_eq!(248453531, actual);
            }
        }
        
        mod part2 {
            use crate::year2023day7::year2023day7::{parse_input, total_winnings};

            #[test]
            fn example() {
                let hands = parse_input("input/2023-07-e1.txt", true);
                let actual = total_winnings(hands);
                assert_eq!(5905, actual);
            }

            #[test]
            fn solution() {
                let hands = parse_input("input/2023-07-input.txt", true);
                let actual = total_winnings(hands);
                assert_eq!(248781813, actual);
            }
        }
    }
}