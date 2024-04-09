use std::cmp::Ordering;
use std::fs::read_to_string;

fn main() {
    let file_contents = read_to_string("inputs/day07.txt").unwrap();
    let result = process_input(&file_contents, true);
    println!("{}", result);
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
#[derive(PartialEq, Debug, Clone, Copy)]
struct Hand([usize; 5]);

#[derive(Debug, Clone, Copy)]
struct HandWithType {
    hand: Hand,
    hand_type: HandType,
}

fn process_input(input: &str, j_is_joker: bool) -> usize {
    let mut hands_and_bids = vec![];
    for line in input.trim().lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        let hand = parse_hand_with_type(hand, j_is_joker);
        let bid = bid.parse::<usize>().unwrap();
        hands_and_bids.push((hand, bid));
    }
    hands_and_bids.sort_by(|x, y| compare_hands(x.0, y.0));
    let mut total = 0;
    for (index, &(_, bid)) in hands_and_bids.iter().enumerate() {
        total += (index + 1) * bid;
    }
    total
}

#[test]
fn test_process_input() {
    let input = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
    let result = process_input(input, false);
    assert_eq!(6440, result);

    let result = process_input(input, true);
    assert_eq!(5905, result);
}

fn parse_hand_with_type(str: &str, j_is_joker: bool) -> HandWithType {
    let hand = parse_hand(str, j_is_joker);
    let hand_type = get_hand_type(hand);
    HandWithType{hand, hand_type}
}

fn parse_hand(str: &str, j_is_joker: bool) -> Hand {
    if str.len() < 5 {
        panic!("hand too small {} {}", str.len(), str);
    }
    let mut chars = str.chars();
    Hand([
        parse_card(chars.next().unwrap(), j_is_joker),
        parse_card(chars.next().unwrap(), j_is_joker),
        parse_card(chars.next().unwrap(), j_is_joker),
        parse_card(chars.next().unwrap(), j_is_joker),
        parse_card(chars.next().unwrap(), j_is_joker),
    ])
}

#[test]
fn test_parse_hand() {
    assert_eq!(Hand([3, 2, 10, 3, 13]), parse_hand("32T3K", false));
    assert_eq!(Hand([11, 11, 11, 11, 11]), parse_hand("JJJJJ", false));
    assert_eq!(Hand([0, 0, 0, 0, 0]), parse_hand("JJJJJ", true));
}

fn parse_card(char: char, j_is_joker: bool) -> usize {
    // not using FromStr for simplicity
    match char {
        '2'..='9' => char.to_digit(10).unwrap() as usize,
        'T' => 10,
        'J' => if j_is_joker {0} else {11},
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("unexpected card {}", char),
    }
}

#[test]
fn test_parse_card() {
    assert_eq!(2, parse_card('2', false));
    assert_eq!(9, parse_card('9', false));
    assert_eq!(10, parse_card('T', false));
    assert_eq!(14, parse_card('A', false));
    assert_eq!(11, parse_card('J', false));
    assert_eq!(0, parse_card('J', true));
}

fn get_hand_type(hand: Hand) -> HandType {
    /*
    0, 1, 2, 3, 4
    5 of a kind - 4 + 3 + 2 + 1 = 10 matches    cant have jokers
    4 of a kind - 3 + 2 + 1 = 6 matches         + joker - 5 of a kind (10 matches)
    full house - 2 + 1 + 1 = 4 matches          cant have jokers
    3 of a kind - 2 + 1 = 3 matches             + joker - 4 of a kind (6 matches)
    2 pair - 1 + 1 = 2 matches                  + joker - full house (4 matches)
    1 pair - 1 match                            + joker - 3 of a kind (3 matches)
    high card - 0 matches                       + joker - 1 pair (1 matches)
     */
    let mut matches_count = 0;
    let mut jokers_count = 0;
    for (first_card_index, &first_card) in hand.0.iter().enumerate() {
        if first_card == 0 {
            jokers_count += 1;
        } else {
            for &second_card in hand.0.iter().skip(first_card_index + 1) {
                if first_card == second_card {
                    matches_count += 1;
                }
            }
        }
    }
    match (matches_count, jokers_count) {
        (10, 0) => HandType::FiveOfAKind,
        (6, 0) => HandType::FourOfAKind,
        (6, 1) => HandType::FiveOfAKind,
        (4, 0) => HandType::FullHouse,
        (3, 0) => HandType::ThreeOfAKind,
        (3, 1) => HandType::FourOfAKind,
        (3, 2) => HandType::FiveOfAKind,
        (2, 0) => HandType::TwoPair,
        (2, 1) => HandType::FullHouse,
        (1, 0) => HandType::OnePair,
        (1, 1) => HandType::ThreeOfAKind,
        (1, 2) => HandType::FourOfAKind,
        (1, 3) => HandType::FiveOfAKind,
        (0, 0) => HandType::HighCard,
        (0, 1) => HandType::OnePair,
        (0, 2) => HandType::ThreeOfAKind,
        (0, 3) => HandType::FourOfAKind,
        (0, 4) => HandType::FiveOfAKind,
        (0, 5) => HandType::FiveOfAKind,
        _ => panic!("strange hand with {} matches and {} jokers {:?}", matches_count, jokers_count, hand),
    }
}

#[test]
fn test_get_hand_type() {
    let j_is_joker = false;
    assert_eq!(HandType::FiveOfAKind, parse_hand_with_type("22222", j_is_joker).hand_type);
    assert_eq!(HandType::FourOfAKind, parse_hand_with_type("22223", j_is_joker).hand_type);
    assert_eq!(HandType::FullHouse, parse_hand_with_type("22233", j_is_joker).hand_type);
    assert_eq!(HandType::ThreeOfAKind, parse_hand_with_type("22234", j_is_joker).hand_type);
    assert_eq!(HandType::TwoPair, parse_hand_with_type("22334", j_is_joker).hand_type);
    assert_eq!(HandType::OnePair, parse_hand_with_type("22345", j_is_joker).hand_type);
    assert_eq!(HandType::HighCard, parse_hand_with_type("23456", j_is_joker).hand_type);

    assert_eq!(HandType::FourOfAKind, parse_hand_with_type("2222J", false).hand_type);
    assert_eq!(HandType::FiveOfAKind, parse_hand_with_type("2222J", true).hand_type);
    assert_eq!(HandType::FullHouse, parse_hand_with_type("222JJ", false).hand_type);
    assert_eq!(HandType::FiveOfAKind, parse_hand_with_type("222JJ", true).hand_type);
    assert_eq!(HandType::ThreeOfAKind, parse_hand_with_type("2223J", false).hand_type);
    assert_eq!(HandType::FourOfAKind, parse_hand_with_type("2223J", true).hand_type);
    assert_eq!(HandType::FullHouse, parse_hand_with_type("22JJJ", false).hand_type);
    assert_eq!(HandType::FiveOfAKind, parse_hand_with_type("22JJJ", true).hand_type);
    assert_eq!(HandType::TwoPair, parse_hand_with_type("223JJ", false).hand_type);
    assert_eq!(HandType::FourOfAKind, parse_hand_with_type("223JJ", true).hand_type);
    assert_eq!(HandType::TwoPair, parse_hand_with_type("2233J", false).hand_type);
    assert_eq!(HandType::FullHouse, parse_hand_with_type("2233J", true).hand_type);
    assert_eq!(HandType::FourOfAKind, parse_hand_with_type("2JJJJ", false).hand_type);
    assert_eq!(HandType::FiveOfAKind, parse_hand_with_type("2JJJJ", true).hand_type);
    assert_eq!(HandType::ThreeOfAKind, parse_hand_with_type("23JJJ", false).hand_type);
    assert_eq!(HandType::FourOfAKind, parse_hand_with_type("23JJJ", true).hand_type);
    assert_eq!(HandType::TwoPair, parse_hand_with_type("233JJ", false).hand_type);
    assert_eq!(HandType::FourOfAKind, parse_hand_with_type("233JJ", true).hand_type);
    assert_eq!(HandType::ThreeOfAKind, parse_hand_with_type("2333J", false).hand_type);
    assert_eq!(HandType::FourOfAKind, parse_hand_with_type("2333J", true).hand_type);
    assert_eq!(HandType::FiveOfAKind, parse_hand_with_type("JJJJJ", false).hand_type);
    assert_eq!(HandType::FiveOfAKind, parse_hand_with_type("JJJJJ", true).hand_type);
}

fn compare_hands(hand1: HandWithType, hand2: HandWithType) -> Ordering {
    let type_order = hand1.hand_type.partial_cmp(&hand2.hand_type).unwrap();
    if type_order != Ordering::Equal {
        return type_order;
    }
    for (index, &card1) in hand1.hand.0.iter().enumerate() {
        let card2 = hand2.hand.0[index];
        let card_order = card1.partial_cmp(&card2).unwrap();
        if card_order != Ordering::Equal {
            return card_order;
        }
    }
    Ordering::Equal
}

#[test]
fn test_compare_hands() {
    let j_is_joker = false;
    assert_eq!(Ordering::Equal, compare_hands(parse_hand_with_type("22222", j_is_joker), parse_hand_with_type("22222", j_is_joker)));
    assert_eq!(Ordering::Greater, compare_hands(parse_hand_with_type("22222", j_is_joker), parse_hand_with_type("22223", j_is_joker)));
    assert_eq!(Ordering::Less, compare_hands(parse_hand_with_type("22223", j_is_joker), parse_hand_with_type("22222", j_is_joker)));
    assert_eq!(Ordering::Greater, compare_hands(parse_hand_with_type("33333", j_is_joker), parse_hand_with_type("22222", j_is_joker)));
    assert_eq!(Ordering::Less, compare_hands(parse_hand_with_type("22222", j_is_joker), parse_hand_with_type("33333", j_is_joker)));
}
