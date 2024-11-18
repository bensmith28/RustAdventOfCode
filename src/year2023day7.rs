mod year2023day7 {
    use std::cmp::Ordering;
    use std::cmp::Ordering::Equal;
    use std::collections::HashMap;
    use std::str::FromStr;
    use crate::read_lines;
    use crate::year2023day7::year2023day7::HandType::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};

    type Cards = Vec<char>;
    type Bid = usize;

    #[derive(PartialEq, Debug, Eq)]
    struct Hand {
        cards: Cards,
        bid: Bid,
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
        fn new(cards: Cards, bid: Bid) -> Self {
            Self { cards, bid }
        }

        fn hand_type(&self) -> HandType {
            let mut count_map: HashMap<char, usize> = self.cards
                .iter()
                .fold(HashMap::new(), |mut map, &card| {
                    *map.entry(card).or_insert(0) += 1;
                    map
                });

            let wilds = count_map.remove(&'W').unwrap_or(0);
            let mut counts: Vec<_> = count_map.values().copied().collect();
            counts.sort_unstable_by(|a, b| b.cmp(a));

            // Add a 0 if counts is empty to avoid panic
            if counts.is_empty() {
                counts.push(0);
            }

            // 6. Match with guards for better readability
            match (counts[0] + wilds, counts.get(1).copied().unwrap_or(0)) {
                (5, _) => FiveOfAKind,
                (4, _) => FourOfAKind,
                (3, 2) => FullHouse,
                (3, _) => ThreeOfAKind,
                (2, 2) => TwoPair,
                (2, _) => OnePair,
                _ => HighCard,
            }
        }

        const fn card_to_score(card: &char) -> usize {
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
                _ => panic!("Invalid Card")
            }
        }
    }

    #[derive(Debug)]
    enum HandError {
        InvalidFormat,
        ParseBidError(std::num::ParseIntError),
    }

    impl std::error::Error for HandError {}

    impl std::fmt::Display for HandError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                HandError::InvalidFormat => write!(f, "Invalid hand format"),
                HandError::ParseBidError(e) => write!(f, "Failed to parse bid: {}", e),
            }
        }
    }

    impl FromStr for Hand {
        type Err = HandError;

        fn from_str(value: &str) -> Result<Self, Self::Err> {
            let (cards_str, bid_str) = value
                .split_once(' ')
                .ok_or(HandError::InvalidFormat)?;

            let cards: Cards = cards_str.chars().collect();
            let bid = bid_str.trim()
                .parse()
                .map_err(HandError::ParseBidError)?;

            Ok(Self::new(cards, bid))
        }
    }

    impl std::fmt::Display for Hand {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {}", self.cards.iter().collect::<String>(), self.bid)
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

    fn total_winnings(hands: Vec<Hand>) -> usize {
        let mut hands = hands;
        hands.sort_unstable();  // sort_unstable is faster when stable sort isn't needed
        hands.iter()
            .rev()
            .enumerate()
            .map(|(i, hand)| (i + 1) * hand.bid)
            .sum()
    }

    #[cfg(test)]
    mod tests {
        mod parse {
            use crate::year2023day7::year2023day7::{parse_input, Hand};

            #[test]
            fn parse_hand() {
                let hand_str = "32T3K 765";
                let actual: Hand = hand_str.parse().unwrap();
                let expected = Hand::new(
                    vec!['3', '2', 'T', '3', 'K'],
                    765
                );
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