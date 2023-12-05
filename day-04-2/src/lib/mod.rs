use std::collections::{HashSet, VecDeque};

struct Card {
    index: usize,
    nb_matches: usize,
}

impl Card {
    fn new(index: usize, winning_numbers: &HashSet<i32>, our_numbers: &[i32]) -> Self {
        let nb_matches = get_nb_matching(winning_numbers, our_numbers);
        Card { index, nb_matches }
    }
}

fn get_nb_matching(winning_numbers: &HashSet<i32>, our_numbers: &[i32]) -> usize {
    our_numbers
        .iter()
        .filter(|our_number| winning_numbers.contains(our_number))
        .count()
}

fn parse_winning_numbers(winning_numbers: &str) -> HashSet<i32> {
    winning_numbers
        .split(' ')
        .filter(|winning_number| !winning_number.is_empty())
        .map(|winning_number| winning_number.trim())
        .map(|winning_number| winning_number.parse::<i32>().unwrap())
        .collect()
}

fn parse_our_numbers(our_numbers: &str) -> Vec<i32> {
    our_numbers
        .split(' ')
        .filter(|our_number| !our_number.is_empty())
        .map(|our_number| our_number.trim())
        .map(|our_number| our_number.parse::<i32>().unwrap())
        .collect()
}
fn parse_line(index: usize, line: &str) -> Card {
    let numbers = line.split(':').last().unwrap();
    let mut numbers = numbers.split('|');

    let winning_numbers = numbers.next().unwrap();
    let winning_numbers = parse_winning_numbers(winning_numbers);

    let our_numbers = numbers.next().unwrap();
    let our_numbers = parse_our_numbers(our_numbers);

    Card::new(index, &winning_numbers, &our_numbers)
}

pub fn process_lines<T>(lines: T) -> i32
where
    T: Iterator<Item = String>,
{
    let original_cards: Vec<Card> = lines
        .enumerate()
        .map(|(index, line)| parse_line(index, &line))
        .collect();

    let mut nb_cards = 0;
    let mut queue: VecDeque<_> = original_cards.iter().map(|card| card.index).collect();
    while let Some(index) = queue.pop_front() {
        nb_cards += 1;
        if let Some(card) = original_cards.get(index) {
            (card.index + 1..card.index + 1 + card.nb_matches)
                .for_each(|copy_index| queue.push_back(copy_index));
        }
    }

    nb_cards
}
