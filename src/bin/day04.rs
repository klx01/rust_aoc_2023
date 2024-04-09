use std::collections::HashSet;
use std::fs::read_to_string;
use std::cmp;

fn main() {
    let file_contents = read_to_string("inputs/day04.txt").unwrap();
    let result = process_input_pt1(&file_contents);
    println!("{}", result);
    let result = process_input_pt2(&file_contents);
    println!("{}", result);
}

fn process_input_pt1(input: &str) -> usize {
    input.trim().lines().map(line_to_number_pt1).sum()
}

#[test]
fn test_process_input() {
    let input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
    let result = process_input_pt1(input);
    assert_eq!(13, result);

    let result = process_input_pt2(input);
    assert_eq!(30, result);
}


fn line_to_number_pt1(line: &str) -> usize {
    let win_number_count = get_number_of_wins(line);
    if win_number_count > 0 {
        2usize.pow((win_number_count as u32) - 1)
    } else {
        0
    }
}

#[test]
fn test_line_to_number_pt1() {
    assert_eq!(8, line_to_number_pt1("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"));
    assert_eq!(1, line_to_number_pt1("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"));
    assert_eq!(0, line_to_number_pt1("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"));
}

fn get_number_of_wins(line: &str) -> usize {
    let (_, card_data) = line.split_once(':').unwrap();
    let (wins, numbers) = card_data.split_once('|').unwrap();
    let wins: HashSet<_> = HashSet::from_iter(parse_numbers(wins));
    let numbers = parse_numbers(numbers);
    numbers.iter().map(|x| wins.contains(x) as usize).sum()
}

#[test]
fn test_get_number_of_wins() {
    assert_eq!(4, get_number_of_wins("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"));
    assert_eq!(1, get_number_of_wins("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"));
    assert_eq!(0, get_number_of_wins("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"));
}

fn parse_numbers(line: &str) -> Vec<usize> {
    line
        .split(' ')
        .filter(|x| *x != "")
        .map(|x| x.parse().unwrap())
        .collect()
}

#[test]
fn test_parse_numbers() {
    assert_eq!([41, 48, 83, 86, 17], parse_numbers("41 48 83 86 17")[..]);
    assert_eq!([83, 86, 6, 31, 17, 9, 48, 53], parse_numbers("83 86  6 31 17  9 48 53")[..]);
}

fn process_input_pt2(input: &str) -> usize {
    let input = input.trim();
    let lines_count = input.lines().count();
    let mut copy_counts = vec![1; lines_count];
    let last_index = lines_count - 1;
    for (index, line) in input.lines().enumerate() {
        let number_of_wins = get_number_of_wins(line);
        let number_of_copies = copy_counts[index];
        for copy_index in (index + 1)..=(cmp::min(index + number_of_wins, last_index)) {
            copy_counts[copy_index] += number_of_copies;
        }
    }
    copy_counts.iter().sum()
}
