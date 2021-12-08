use std::{collections::HashSet, fs};

type Card = Vec<Vec<(u32, bool)>>;

fn parse_input(text: String) -> (Vec<u32>, Vec<Vec<Vec<(u32, bool)>>>) {
    let mut split = text.split_whitespace();

    let numbers: Vec<u32> = split
        .next()
        .unwrap()
        .split(",")
        .map(|x| u32::from_str_radix(x, 10).unwrap())
        .collect();

    let mut cards: Vec<Card> = vec![vec![vec![]]];

    for line in split {
        let value = (u32::from_str_radix(line, 10).unwrap(), false);

        let last_card = cards.last_mut().unwrap();

        let row_size = last_card.len();
        let last_row = last_card.last_mut().unwrap();

        if last_row.len() == 5 && row_size < 5 {
            last_card.push(vec![value]);
        } else if last_row.len() == 5 && row_size == 5 {
            cards.push(vec![vec![value]]);
        } else {
            last_row.push(value);
        }
    }

    return (numbers, cards);
}

fn update_cards(cards: Vec<Card>, x: u32) -> Vec<Card> {
    return cards
        .iter()
        .map(|card| {
            return card
                .iter()
                .map(|row| {
                    return row
                        .iter()
                        .map(|(value, seen)| {
                            if *value == x {
                                return (*value, true);
                            } else {
                                return (*value, *seen);
                            }
                        })
                        .collect();
                })
                .collect();
        })
        .collect();
}

fn has_won(card: Card) -> bool {
    let mut completed_columns: HashSet<usize> = HashSet::from_iter(0..5);

    for row in card.clone() {
        let mut completed_row = true;

        for (i, (_val, seen)) in row.iter().enumerate() {
            completed_row = completed_row && *seen;

            if completed_columns.contains(&i) && !*seen {
                completed_columns.remove(&i);
            }
        }

        if completed_row {
            return true;
        }
    }

    return !completed_columns.is_empty();
}

fn play(numbers: Vec<u32>, cards: Vec<Card>) -> (Option<(u32, Card)>, Option<(u32, Card)>) {
    let mut current_cards = cards;

    let mut first_winner: Option<(u32, Card)> = None;
    let mut last_winner: Option<(u32, Card)> = None;

    for x in numbers {
        let new_cards: Vec<(Card, bool)> = update_cards(current_cards.clone(), x)
            .into_iter()
            .map(|card| (card.clone(), has_won(card.clone())))
            .collect();

        if first_winner == None {
            if let Some((card, _winner)) = new_cards.iter().find(|(_card, winner)| *winner) {
                first_winner = Some((x, card.clone()));
            }
        }

        if new_cards.len() == 1 && new_cards[0].1 {
            last_winner = Some((x, new_cards[0].0.clone()));
            break;
        }

        current_cards = new_cards
            .into_iter()
            .filter_map(|(card, winner)| if winner { None } else { Some(card) })
            .collect();
    }

    (first_winner, last_winner)
}

fn get_card_value((x, card): (u32, Card)) -> u32 {
    let mut sum = 0;
    for row in card {
        for (val, seen) in row {
            if !seen {
                sum += val;
            }
        }
    }

    return sum * x;
}

pub fn main() -> () {
    let text = fs::read_to_string("./src/data/day4.txt").unwrap();

    let (numbers, cards) = parse_input(text);

    let (first_winner, last_winner) = play(numbers, cards);

    if let Some(value) = first_winner {
        println!("Part one: {}", get_card_value(value));
    }

    if let Some(value) = last_winner {
        println!("{:?}", value);
        println!("Part two: {}", get_card_value(value));
    }
}
