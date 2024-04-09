use std::fs::read_to_string;
use std::cmp;

fn main() {
    let file_contents = read_to_string("inputs/day11.txt").unwrap();
    let result = process_input(&file_contents, 2);
    println!("{}", result);
    let result = process_input(&file_contents, 1000000);
    println!("{}", result);
}

fn process_input(input: &str, expand_factor: usize) -> usize {
    let input = input.trim();
    let galaxy_indexes = input
        .chars()
        .enumerate()
        .filter(|&(_, x)| x == '#')
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let map = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let cols_count = map[0].len();
    let orig_cols_count = cols_count + 1;
    let galaxy_indexes = galaxy_indexes
        .into_iter()
        .map(|x| (x / orig_cols_count, x % orig_cols_count))
        .collect::<Vec<_>>();
    let rows_is_empty = map
        .iter()
        .map(|row| row.iter().all(|&x| x == '.') as u8)
        .collect::<Vec<_>>();
    let cols_is_empty = (0..cols_count)
        .map(|i| map.iter().all(|row| row[i] == '.') as u8)
        .collect::<Vec<_>>();

    let expand_factor = expand_factor - 1; // they are already counted once in the _normal counts

    let mut dist_sum = 0;
    for (first_index, &(first_row, first_col)) in galaxy_indexes.iter().enumerate() {
        for &(second_row, second_col) in galaxy_indexes.iter().skip(first_index + 1) {
            let row_from = cmp::min(first_row, second_row);
            let row_to = cmp::max(first_row, second_row);
            let rows_diff_normal = row_to - row_from;
            let empty_rows_between = rows_is_empty[row_from..row_to].iter().sum::<u8>() as usize;

            let col_from = cmp::min(first_col, second_col);
            let col_to = cmp::max(first_col, second_col);
            let cols_diff_normal = col_to - col_from;
            let empty_cols_between = cols_is_empty[col_from..col_to].iter().sum::<u8>() as usize;

            let distance = rows_diff_normal + cols_diff_normal + ((empty_rows_between + empty_cols_between) * expand_factor);
            dist_sum += distance;
        }
    }

    dist_sum
}

#[test]
fn test_process_input() {
    let input = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
    let result = process_input(input, 2);
    assert_eq!(374, result);
    let result = process_input(input, 10);
    assert_eq!(1030, result);
    let result = process_input(input, 100);
    assert_eq!(8410, result);
}
