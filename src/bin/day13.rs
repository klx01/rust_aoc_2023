use std::cmp;
use std::fs::read_to_string;

fn main() {
    let file_contents = read_to_string("inputs/day13.txt").unwrap();
    let result = process_input(&file_contents, 0);
    println!("{}", result);
    let result = process_input(&file_contents, 1);
    println!("{}", result);
}

fn process_input(input: &str, expected_diff: u8) -> usize {
    input.trim().split("\n\n").map(|x| process_pattern(x, expected_diff)).sum()
}

#[test]
fn test_process_input() {
    let input = "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
    let result = process_input(input, 0);
    assert_eq!(405, result);

    let result = process_input(input, 1);
    assert_eq!(400, result);
}

fn process_pattern(input: &str, expected_diff: u8) -> usize {
    let lines = input.trim().lines().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let col_count = lines[0].len();
    let row_count = lines.len();
    'row_outer: for index in 0..(row_count - 1) {
        let mut total_diff = 0;
        let max_shift = cmp::min(index, row_count - index - 2);
        for shift in 0..=max_shift {
            let prev_index = index - shift;
            let next_index = index + shift + 1;
            total_diff += count_diffs_row(&lines, prev_index, next_index);
            if total_diff > expected_diff {
                continue 'row_outer
            }
        }
        if total_diff != expected_diff {
            continue;
        }
        return (index + 1) * 100;
    }

    'col_outer: for index in 0..(col_count - 1) {
        let mut total_diff = 0;
        let max_shift = cmp::min(index, col_count - index - 2);
        for shift in 0..=max_shift {
            let prev_index = index - shift;
            let next_index = index + shift + 1;
            total_diff += count_diffs_col(&lines, prev_index, next_index);
            if total_diff > expected_diff {
                continue 'col_outer
            }
        }
        if total_diff != expected_diff {
            continue;
        }
        return index + 1
    }
    0
}

#[test]
fn test_process_pattern() {
    let input = "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
";
    let result = process_pattern(input, 0);
    assert_eq!(5, result);

    let result = process_pattern(input, 1);
    assert_eq!(300, result);

    let input = "
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
    let result = process_pattern(input, 0);
    assert_eq!(400, result);

    let result = process_pattern(input, 1);
    assert_eq!(100, result);
}

fn count_diffs_row(lines: &[Vec<char>], index1: usize, index2: usize) -> u8 {
    let row1 = &lines[index1];
    let row2 = &lines[index2];
    row1
        .iter()
        .zip(row2)
        .map(|(&x, &y)| (x != y) as u8)
        .sum()
}

fn count_diffs_col(lines: &[Vec<char>], index1: usize, index2: usize) -> u8 {
    lines
        .iter()
        .map(|line| (line[index1] != line[index2]) as u8)
        .sum()
}
