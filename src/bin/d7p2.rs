use std::collections::HashMap;

type Card = char;

#[derive(Debug)]
struct Hand {
    #[allow(dead_code)]
    cards: Vec<Card>,
    bid: u64,
    score: u64,
}

fn main() {
    let input = include_str!("./inputs/d7-input.txt");

    let mut hands: Vec<Hand> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (hand_str, bid_str) = line
                .split_once(' ')
                .expect(&format!("Could not parse line {}", i + 1));

            let cards = hand_str.chars().collect();
            let score = calculate_score(&cards);

            Hand {
                cards,
                bid: bid_str
                    .parse()
                    .expect(&format!("Could not parse bid at line {}", i + 1)),
                score,
            }
        })
        .collect();

    hands.sort_by_key(|c| c.score);

    //println!("{:?}", hands);

    let result: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            let win = ((i as u64) + 1) * hand.bid;
            //println!("Hand {:?} wins {} * {} = {}", hand, i + 1, hand.bid, win);
            win
        })
        .sum();

    println!("Answer: {}", result);
}

fn calculate_score(cards: &Vec<Card>) -> u64 {
    hand_type_score(cards) + high_card_score(cards)
}

fn hand_type_score(cards: &Vec<Card>) -> u64 {
    let mut counts = HashMap::<Card, u64>::new();

    for card in cards {
        count_card(*card, &mut counts);
    }

    let j_count = counts.remove(&'J').unwrap_or(0);

    let mut counts_list: Vec<u64> = counts.iter().map(|c| *c.1).collect();
    counts_list.sort();

    let mut counts_iter = counts_list.iter().rev();
    let first = counts_iter.next().map_or(0, |v| *v);
    let second = counts_iter.next().map_or(0, |v| *v);

    let score: u64 = match first + j_count {
        5 => 6,
        4 => 5,
        3 => {
            if second == 2 {
                4
            } else {
                3
            }
        }
        2 => {
            if second == 2 {
                2
            } else {
                1
            }
        }
        1 => 0,
        _ => panic!(
            "Something went wrong counting cards {:?}: f:{} s:{}",
            cards, first, second
        ),
    };

    //println!("Hand {:?} gets a score of {}", cards, score);

    score << (5 * 8)
}

fn count_card(card: Card, counts: &mut HashMap<Card, u64>) {
    let count = counts.get(&card).map_or(0, |v| *v);
    counts.insert(card, count + 1);
}

fn high_card_score(cards: &Vec<Card>) -> u64 {
    cards
        .iter()
        .rev()
        .enumerate()
        .fold(0 as u64, |acc, (i, card)| {
            let value = card_value(*card);
            acc + (value << (i * 8))
        })
}

fn card_value(card: Card) -> u64 {
    match card {
        'J' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("Unknown card {}", card),
    }
}
