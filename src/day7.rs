use std::fs::File;
use std::io::{self};
use std::cmp::Ordering;


const CARDS_PART1: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
const CARDS: [char; 13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];


#[derive(Debug)]
struct Hand{
    content: String,
    value: usize,
    bid: usize,
}


fn cmp_hands(a: &Hand, b: &Hand) -> Ordering {
    let cards: Vec<char> = CARDS.to_vec();
    if a.value == b.value {
        for (i, c) in a.content.chars().enumerate() {
            let b_char = b.content.chars().nth(i).unwrap();
            if c != b_char {
                let index_c = cards
                    .iter()
                    .position(|&x| x == c)
                    .unwrap();
                let index_b = cards
                    .iter()
                    .position(|&x| x == b_char)
                    .unwrap();
                return index_c.cmp(&index_b).reverse()
            }
        }
    }
    a.value.cmp(&b.value)
}


fn hand_value_part1(line: &String) -> usize {
    let cards: Vec<char> = CARDS.to_vec();
    let mut labels: Vec<usize> = vec![0; cards.len()];
    let chars: Vec<char> = line.chars().collect();
    for x in chars {
        labels[cards.iter().position(|&c| c == x).unwrap()] += 1;
    }
    let counts = labels.iter().filter(|&x| x > &0);
    match counts.clone().count() {
        1 => 7,
        2 => {if counts.clone().any(|&x| x == 4) {6} else {5}}, // why .any and .count are mutating??
        3 => {if counts.clone().any(|&x| x == 3) {4} else {3}},
        4 => 2,
        5 => 1,
        _ => panic!("Wrong number of cards")
    }
}


fn hand_value(line: &String) -> usize {
    let cards: Vec<char> = CARDS.to_vec();
    let mut labels: Vec<usize> = vec![0; cards.len()];
    let chars: Vec<char> = line.chars().collect();
    for x in chars {
        labels[cards.iter().position(|&c| c == x).unwrap()] += 1;
    }

    let (joker, labels_no_joker) = labels.split_last_mut().unwrap();
    let max = labels_no_joker.iter().max().unwrap();
    let index = labels_no_joker.iter().position(|element| element == max).unwrap();
    labels_no_joker[index] += *joker;
    let counts = labels_no_joker.iter().filter(|&x| x > &0);
    
    match counts.clone().count() {
        1 => 7,
        2 => {if counts.clone().any(|&x| x == 4) {6} else {5}}, // why .any and .count are mutating??
        3 => {if counts.clone().any(|&x| x == 3) {4} else {3}},
        4 => 2,
        5 => 1,
        _ => panic!("Wrong number of cards")
    }
}


pub fn day7(lines: std::io::Lines<io::BufReader<File>>) {
    let mut hands: Vec<Hand> = vec![];
    for line in lines {
        if let Ok(ip) = line {
            let parsed: Vec<String> = ip.split_whitespace().map(|x| x.to_string()).collect::<Vec<_>>();
            let (hand, bid): (&String, &String) = (parsed.first().unwrap(), parsed.last().unwrap());
            hands.push(Hand {content: hand.clone(), value: hand_value(hand), bid: bid.parse().unwrap()});
        }
    }
    hands.sort_by(|a, b| cmp_hands(a, b));
    let result: usize = hands.iter().enumerate()
                    .map(|(i, x)| (i+1)*x.bid)
                    .sum();
    println!("Result: {:?}", result);
}
    
