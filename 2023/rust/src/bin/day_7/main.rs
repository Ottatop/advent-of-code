use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    // Part 1

    let mut hands = parse_input();
    hands.sort_by_key(|hand| {
        let mut cards = hand.hand.chars().map(char_to_card);

        (
            hand.hand_type(),
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
        )
    });

    let answer = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| {
            let Hand { hand: _, bid } = hand;
            (i + 1) * bid
        })
        .sum::<usize>();

    println!("Part 1: The answer is {answer}");

    // Part 2

    let mut hands = parse_input();
    hands.sort_by_key(|hand| {
        let mut cards = hand.hand.chars().map(char_to_card_with_joker);

        (
            hand.hand_type_with_joker(),
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
            cards.next().unwrap(),
        )
    });

    let answer = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| {
            let Hand { hand: _, bid } = hand;
            (i + 1) * bid
        })
        .sum::<usize>();

    println!("Part 2: The answer is {answer}");
}

struct Hand {
    hand: String,
    bid: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum CardWithJoker {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

fn char_to_card(ch: char) -> Card {
    match ch {
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::T,
        'J' => Card::J,
        'Q' => Card::Q,
        'K' => Card::K,
        'A' => Card::A,
        _ => panic!(),
    }
}

fn char_to_card_with_joker(ch: char) -> CardWithJoker {
    match ch {
        '2' => CardWithJoker::Two,
        '3' => CardWithJoker::Three,
        '4' => CardWithJoker::Four,
        '5' => CardWithJoker::Five,
        '6' => CardWithJoker::Six,
        '7' => CardWithJoker::Seven,
        '8' => CardWithJoker::Eight,
        '9' => CardWithJoker::Nine,
        'T' => CardWithJoker::T,
        'J' => CardWithJoker::J,
        'Q' => CardWithJoker::Q,
        'K' => CardWithJoker::K,
        'A' => CardWithJoker::A,
        _ => panic!(),
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        assert_eq!(self.hand.len(), 5);

        let mut counts = HashMap::<char, usize>::new();

        for card in self.hand.chars() {
            *counts.entry(card).or_insert(0) += 1;
        }

        let mut counts = counts.values().collect::<Vec<_>>();
        counts.sort();

        match counts.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 4] => HandType::FourOfAKind,
            [5] => HandType::FiveOfAKind,
            _ => unreachable!(),
        }
    }

    fn hand_type_with_joker(&self) -> HandType {
        assert_eq!(self.hand.len(), 5);

        let mut counts = HashMap::<char, usize>::new();

        for card in self.hand.chars() {
            *counts.entry(card).or_insert(0) += 1;
        }

        counts.remove(&'J');

        let mut counts = counts.values().collect::<Vec<_>>();
        counts.sort();

        match counts.as_slice() {
            [_] | [] => HandType::FiveOfAKind,
            [1, 4] | [1, _] => HandType::FourOfAKind,
            [2, 3] | [2, _] => HandType::FullHouse,
            [1, 1, 3] | [1, 1, _] => HandType::ThreeOfAKind,
            [1, 2, 2] | [1, 2, _] => HandType::TwoPair,
            [1, 1, 1, _] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn parse_input() -> Vec<Hand> {
    INPUT
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let hand = split.next().unwrap().to_string();
            let bid = split.next().unwrap().parse::<usize>().unwrap();

            Hand { hand, bid }
        })
        .collect()
}
