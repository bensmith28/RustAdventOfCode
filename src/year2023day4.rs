mod year2023day4 {
    use std::cmp::min;
    use std::collections::{HashMap, VecDeque};
    use std::ops::Range;

    #[derive(PartialEq)]
    #[derive(Debug)]
    struct Card {
        id: usize,
        winners: Vec<usize>,
        numbers: Vec<usize>,
    }

    impl Card {
        fn parse(s: &str) -> Card {
            let colon_idx = s.find(':').unwrap();
            let pipe_idx = s.find('|').unwrap();

            let id: usize = s[5..colon_idx].trim().parse().unwrap();
            let mut winners: Vec<usize> = vec![];
            for winner_str in s[colon_idx + 1..pipe_idx].split_whitespace() {
                winners.push(winner_str.parse().unwrap());
            }
            let mut numbers: Vec<usize> = vec![];
            for number_str in s[pipe_idx + 1..s.len()].split_whitespace() {
                numbers.push(number_str.parse().unwrap());
            }

            Card { id, winners, numbers }
        }

        fn score(&self) -> usize {
            let winning_numbers: Vec<_> = self.numbers.iter().filter(|n| {
                self.winners.contains(n)
            }).collect();
            if winning_numbers.is_empty() { 0 }
            else {
                let mut score = 1usize;
                for _ in 1..winning_numbers.len() {
                    score *= 2;
                }
                score
            }
        }

        fn won_cards(&self, max_id: usize) -> Range<usize> {
            let winning_numbers: Vec<_> = self.numbers.iter().filter(|n| {
                self.winners.contains(n)
            }).collect();
            if winning_numbers.is_empty() { return 0..0; }
            self.id+1..min(self.id+winning_numbers.len()+1, max_id+1)
        }
    }

    fn count_cards(cards: &[Card]) -> usize {
        let mut queue: VecDeque<&Card> = VecDeque::new();
        let max_id = cards.iter().map(|card| card.id).max().unwrap_or(0);

        for card in cards {
            queue.push_front(card);
        }

        let mut count = 0usize;
        let mut history: HashMap<usize, usize> = HashMap::new();
        // process the cards in reverse order of the input file, so that the last
        // card (which should have no follow-up cards) gets processed first.
        while let Some(card) = queue.pop_front() {
            // If we've seen this card before, just add its known value
            if let Some(&value) = history.get(&card.id) {
                count += value;
                continue;
            }
            
            // Calculate additional cards won
            let mut local_count = 1; // Count this card itself
            // in the case where a card has no follow-up cards, this for loop
            // will short circuit.
            for won_id in card.won_cards(max_id) {
                if let Some(&additional) = history.get(&won_id) {
                    local_count += additional;
                } else {
                    // If we haven't processed this card yet, add it to the queue
                    if let Some(won_card) = cards.iter().find(|c| c.id == won_id) {
                        queue.push_back(won_card);
                    }
                }
            }

            // Store the result for this card
            history.insert(card.id, local_count);
            count += local_count;
        }
        count
    }

    #[cfg(test)]
    mod tests {
        mod parse {
            use crate::year2023day4::year2023day4::Card;

            #[test]
            fn handle_example_line_one() {
                let card = Card::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
                let expected = Card {
                    id: 1,
                    winners: vec![41, 48, 83, 86, 17],
                    numbers: vec![83, 86,  6, 31, 17,  9, 48, 53],
                };

                assert_eq!(card, expected);
            }
        }

        mod part1 {
            use crate::read_lines;
            use crate::year2023day4::year2023day4::Card;

            #[test]
            fn handles_single_card_score() {
                let card = Card::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
                assert_eq!(card.score(), 8);
            }

            #[test]
            fn example() {
                let result: usize = read_lines("input/2023-04-e1.txt")
                    .map(|l| Card::parse(l.unwrap().as_str()).score())
                    .sum();
                assert_eq!(result, 13);
            }

            #[test]
            fn solution() {
                let result: usize = read_lines("input/2023-04-input.txt")
                    .map(|l| Card::parse(l.unwrap().as_str()).score())
                    .sum();
                assert_eq!(result, 17782);
            }
        }

        mod part2 {
            use crate::read_lines;
            use crate::year2023day4::year2023day4::{count_cards, Card};

            #[test]
            fn handles_single_win_cards() {
                let card = Card::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
                assert_eq!(card.won_cards(6), 2..6);
            }

            #[test]
            fn handles_example_win_cards() {
                let result: Vec<_> = read_lines("input/2023-04-e1.txt")
                    .map(|l| Card::parse(l.unwrap().as_str()).won_cards(6))
                    .collect();
                assert_eq!(result, vec![
                    2..6,
                    3..5,
                    4..6,
                    5..6,
                    0..0,
                    0..0
                ]);
            }

            #[test]
            fn example() {
                let cards: Vec<_> = read_lines("input/2023-04-e1.txt")
                    .map(|l| Card::parse(l.unwrap().as_str()))
                    .collect();
                let result = count_cards(&cards);
                assert_eq!(result, 30);
            }

            #[test]
            fn solution() {
                let cards: Vec<_> = read_lines("input/2023-04-input.txt")
                    .map(|l| Card::parse(l.unwrap().as_str()))
                    .collect();
                let result = count_cards(&cards);
                assert_eq!(result, 8477787);
            }
        }
    }
}