use std::fs::read_to_string;
use std::cmp;

fn main() {
    let file_contents = read_to_string("inputs/day03.txt").unwrap();
    let result = process_input_pt1(&file_contents);
    println!("{}", result);
    let result = process_input_pt2(&file_contents);
    println!("{}", result);
}

fn process_input_pt1(input: &str) -> usize {
    let chars = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;
    let mut current_number = String::new();
    let mut is_adjacent = false;
    for (row_no, row_data) in chars.iter().enumerate() {
        let last_col_no = row_data.len() - 1;
        for (col_no, &current_char) in row_data.iter().enumerate() {
            let is_digit = current_char.is_ascii_digit();
            if is_digit {
                current_number.push(current_char);
                if !is_adjacent {
                    is_adjacent = check_is_adjacent_pt1(&chars, row_no, col_no);
                }
            }
            if (!is_digit) || (col_no == last_col_no) {
                if current_number.len() > 0 {
                    if is_adjacent {
                        sum += current_number.parse::<usize>().unwrap();
                    }
                    is_adjacent = false;
                    current_number.truncate(0);
                }
            }
        }
    }
    sum
}

fn check_is_adjacent_pt1(chars: &[Vec<char>], current_row_no: usize, current_col_no: usize) -> bool {
    let current_row_no = current_row_no as isize;
    let current_col_no = current_col_no as isize;
    let list_to_check = [
        (current_row_no - 1, current_col_no - 1),
        (current_row_no - 1, current_col_no    ),
        (current_row_no - 1, current_col_no + 1),
        (current_row_no    , current_col_no + 1),
        (current_row_no + 1, current_col_no + 1),
        (current_row_no + 1, current_col_no    ),
        (current_row_no + 1, current_col_no - 1),
        (current_row_no    , current_col_no - 1),
    ];
    for (check_row, check_col) in list_to_check {
        if (check_row < 0) || (check_col < 0) {
            continue;
        }
        let check_val = chars.get(check_row as usize)
            .and_then(|row| row.get(check_col as usize));
        match check_val {
            None => (),
            Some(&check_char) => if (check_char != '.') && ((check_char < '0') || (check_char > '9')) {
                return true;
            }
        }
    }
    return false;
}

#[test]
fn test_process_input() {
    let input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
    let result = process_input_pt1(input);
    assert_eq!(4361, result);

    let result = process_input_pt2(input);
    assert_eq!(467835, result);
}

fn process_input_pt2(input: &str) -> usize {
    let chars = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (row_no, row_data) in chars.iter().enumerate() {
        for (col_no, &current_char) in row_data.iter().enumerate() {
            if current_char != '*' {
                continue;
            }
            let adjacent_numbers = get_adjacent_numbers(&chars, row_no, col_no);
            if adjacent_numbers.len() != 2 {
                continue;
            }
            sum += adjacent_numbers[0] * adjacent_numbers[1];
        }
    }
    sum
}

fn get_adjacent_numbers(chars: &[Vec<char>], current_row_no: usize, current_col_no: usize) -> Vec<usize> {
    let mut adjacent_numbers = vec![];
    let row_from = cmp::max(current_row_no, 1) - 1;
    let row_to = cmp::min(current_row_no + 1, chars.len() - 1);
    let col_from_default = cmp::max(current_col_no, 1) - 1;

    for check_row_no in row_from..=row_to {
        let row = &chars[check_row_no];
        let mut col_from = col_from_default;
        while (col_from > 0) && row[col_from].is_ascii_digit() {
            col_from -= 1;
        }
        let mut current_number = String::new();

        let mut check_col_no = col_from;
        let last_possible_col_no = row.len() - 1;
        let check_at_least_until_col_no = current_col_no + 1;
        loop {
            let current_char = row[check_col_no];
            let is_digit = current_char.is_ascii_digit();
            let is_last_column = check_col_no == last_possible_col_no;
            if is_digit {
                current_number.push(current_char);
            }
            if (!is_digit) || is_last_column {
                if current_number.len() > 0 {
                    adjacent_numbers.push(current_number.parse().unwrap());
                    current_number.truncate(0);
                }
            }
            if is_last_column {
                break;
            }
            if (!is_digit) && (check_col_no >= check_at_least_until_col_no) {
                break;
            }
            check_col_no += 1;
        }
    }
    adjacent_numbers
}
