use std::collections::HashSet;

struct Card {
    winning_numbers: HashSet<i32>,
    our_numbers: Vec<i32>,
}

impl Card {
    fn get_nb_matching(&self) -> usize {
        self.our_numbers
            .iter()
            .filter(|our_number| self.winning_numbers.contains(our_number))
            .count()
    }
}

fn nb_matching_to_score(nb_matching: usize) -> i32 {
    match nb_matching {
        0 => 0,
        _ => 2_i32.pow((nb_matching - 1) as u32),
    }
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
fn parse_line(line: &str) -> Card {
    let numbers = line.split(':').last().unwrap();
    let mut numbers = numbers.split('|');

    let winning_numbers = numbers.next().unwrap();
    let winning_numbers = parse_winning_numbers(winning_numbers);

    let our_numbers = numbers.next().unwrap();
    let our_numbers = parse_our_numbers(our_numbers);

    Card {
        winning_numbers,
        our_numbers,
    }
}

pub fn process_lines<T>(lines: T) -> i32
where
    T: Iterator<Item = String>,
{
    lines
        .map(|line| parse_line(&line))
        .map(|card| card.get_nb_matching())
        .map(nb_matching_to_score)
        .sum()
}
