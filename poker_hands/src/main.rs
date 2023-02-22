use std::collections::HashMap;
mod input;

#[derive(Debug, Copy, Clone)]
struct Card {
    number: i32,
    suit: char,
}

impl Card {
    fn new(number: i32, suit: char) -> Self {
        Self { number, suit }
    }
    fn from_string(card: &str) -> Self {
        let card: Vec<char> = card.chars().collect();
        let number = match card[0] {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => card[0] as i32 - 48,
        };
        Self {
            number,
            suit: card[1],
        }
    }
}

#[derive(Debug)]
enum RESULT {
    WIN,
    DRAW,
    LOST,
    UNKNOW,
}

fn main() {
    let mut result = 0;
    for hands in input::VARIABLE {
        let cards: Vec<&str> = hands.split(' ').collect();
        let mut chunks = cards.chunks(5);
        let player1: Vec<Card> = chunks
            .next()
            .unwrap()
            .iter()
            .map(|x| Card::from_string(x))
            .collect();
        let player2: Vec<Card> = chunks
            .next()
            .unwrap()
            .iter()
            .map(|x| Card::from_string(x))
            .collect();
        if does_player1_win(&mut vec_to_arr(player1), &mut vec_to_arr(player2)) {
            result += 1;
        }
    }
    println!("{:?}", result);
}

fn does_player1_win(player1: &mut [Card; 5], player2: &mut [Card; 5]) -> bool {
    let analyzed_hand_player1 = analyze_hand(player1);
    let analyzed_hand_player2 = analyze_hand(player2);
    match does_player_1_win_by_rank(&analyzed_hand_player1, &analyzed_hand_player2) {
        RESULT::WIN => true,
        RESULT::LOST => false,
        _ => {
            for i in (0..5).rev() {
                if player1[i].number > player2[i].number {
                    return true;
                }
                if player1[i].number < player2[i].number {
                    return false;
                }
            }
            false
        }
    }
}

fn analyze_hand(hand: &mut [Card; 5]) -> (i32, Vec<i32>) {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut is_consecutive = true;
    let mut all_same_suit = true;
    let mut sum_of_numbers = 0;
    hand.sort_by_key(|k| k.number);
    for index in 0..4 {
        if is_consecutive && hand[index].number + 1 != hand[index + 1].number {
            is_consecutive = false;
        }
        if all_same_suit && hand[index].suit != hand[index + 1].suit {
            all_same_suit = false;
        }
        if hand[index].number == hand[index + 1].number {
            let entry = map.entry(hand[index].number).or_insert(1);
            *entry += 1;
        }
        sum_of_numbers += hand[index].number;
    }
    sum_of_numbers += hand[4].number;
    get_rank(is_consecutive, all_same_suit, map, sum_of_numbers, hand)
}

fn get_rank(
    is_consecutive: bool,
    all_same_suit: bool,
    map: HashMap<i32, i32>,
    sum_of_numbers: i32,
    hand: &[Card; 5],
) -> (i32, Vec<i32>) {
    if all_same_suit {
        if is_consecutive {
            if sum_of_numbers == 60 {
                return (10, vec![14]);
            } else {
                return (9, vec![hand[4].number]);
            }
        }
    }
    let mut repeating_cards: Vec<i32> = map.clone().into_values().collect();
    let mut keys: Vec<i32> = map.clone().into_keys().collect();
    if repeating_cards.contains(&4) {
        return (8, vec![keys[0]]);
    }
    if repeating_cards.contains(&3) && repeating_cards.contains(&2) {
        let mut a = 0;
        let mut b = 0;
        for (key, value) in map {
            if value == 3 {
                a = key;
            }
            if value == 2 {
                b = key;
            }
        }
        return (7, vec![a, b]);
    }
    if all_same_suit {
        return (6, vec![hand[4].number]);
    }
    if is_consecutive {
        return (5, vec![hand[4].number]);
    }
    if repeating_cards.contains(&3) {
        return (4, vec![keys[0]]);
    }
    let two_pairs: Vec<i32> = repeating_cards
        .iter()
        .filter(|x| **x == 2)
        .cloned()
        .collect();
    if two_pairs.iter().len() == 2 {
        keys.sort();
        return (3, vec![keys[1], keys[0]]);
    };
    if repeating_cards.contains(&2) {
        return (2, vec![keys[0]]);
    }
    (1, vec![hand[4].number])
}

fn does_player_1_win_by_rank(player1: &(i32, Vec<i32>), player2: &(i32, Vec<i32>)) -> RESULT {
    if player1.0 > player2.0 {
        return RESULT::WIN;
    }
    if player1.0 < player2.0 {
        return RESULT::LOST;
    }
    if player1.0 == player2.0 {
        for i in 0..player1.1.len() {
            if player1.1[i] > player2.1[i] {
                return RESULT::WIN;
            }
            if player1.1[i] < player2.1[i] {
                return RESULT::LOST;
            }
        }
    };
    RESULT::UNKNOW
}

fn vec_to_arr(vec: Vec<Card>) -> [Card; 5] {
    let mut result = [Card::new(0, ' '); 5];
    for (index, card) in vec.iter().enumerate() {
        result[index] = *card
    }
    result
}