use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let file_contents = read_to_string("inputs/day14.txt").unwrap();
    let result = process_input_pt1(&file_contents);
    println!("{}", result);
    let result = process_input_pt2(&file_contents, 1000000000);
    println!("{}", result);
}

fn process_input_pt1(input: &str) -> usize {
    let lines = split_str(input);
    let cols_count = lines[0].len();
    let rows_count = lines.len();
    let mut total_weight = 0;
    for col in 0..cols_count {
        let mut current_chain_size = 0;
        for row in (0..rows_count).rev() {
            let char = lines[row][col];
            match char {
                '#' => {
                    total_weight += calc_chain_weight(current_chain_size, row + 1, rows_count);
                    current_chain_size = 0;
                },
                'O' => {
                    current_chain_size += 1;
                },
                '.' => {},
                _ => panic!("unexpected char {}", char),
            }
        }
        total_weight += calc_chain_weight(current_chain_size, 0, rows_count);
    }
    total_weight
}

fn split_str(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn calc_chain_weight(chain_size: usize, last_chain_row: usize, rows_count: usize) -> usize {
    if chain_size == 0 {
        return 0;
    }
    let last_stone_weight = rows_count - last_chain_row;
    // n + n-1 + n-2 ... n-k
    (last_stone_weight * chain_size) - (chain_size * (chain_size - 1) / 2)
}

#[test]
fn test_process_input() {
    let input = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
    let result = process_input_pt1(input);
    assert_eq!(136, result);

    let result = process_input_pt2(input, 1000000000);
    assert_eq!(64, result);
}

fn process_input_pt2(input: &str, cycles: usize) -> usize {
    let mut lines = split_str(input);

    let mut cache = HashMap::new();
    let mut cycle_first = 0;
    let mut cycle_last = 0;
    for iteration in 0..cycles {
        if iteration > 10000 {
            panic!("did not find cycle after {} iterations", iteration);
        }
        let state = map_to_string(&lines);
        if let Some(&same_iter) = cache.get(&state) {
            cycle_first = same_iter;
            cycle_last = iteration;
            break;
        }
        cache.insert(state, iteration);

        do_cycle(&mut lines);
    }
    let cycle_last_result = if cycle_last == 0 {
        // we have exited the loop because cycles is too small
        lines
    } else {
        let cycle_len = cycle_last - cycle_first;
        let last_cycle_index = ((cycles - cycle_first) % cycle_len) + cycle_first;
        let cycle_last_result = cache
            .iter()
            .find(|&(_, val)| *val == last_cycle_index)
            .map(|(key, _)| key)
            .unwrap();
        split_str(cycle_last_result)
    };
    calc_current_load(&cycle_last_result)
}

fn do_cycle(map: &mut[Vec<char>]) {
    let rows_count = map.len();
    let cols_count = map[0].len();

    for col in 0..cols_count {
        let mut available_row = 0;
        for row in 0..rows_count {
            let char = map[row][col];
            match char {
                '#' => available_row = row + 1,
                'O' => {
                    if available_row != row {
                        map[available_row][col] = char;
                        map[row][col] = '.'
                    }
                    available_row += 1;
                },
                '.' => {},
                _ => panic!("unexpected char {}", char),
            }
        }
    }

    for row in 0..rows_count {
        let mut available_col = 0;
        for col in 0..cols_count {
            let char = map[row][col];
            match char {
                '#' => available_col = col + 1,
                'O' => {
                    if available_col != col {
                        map[row][available_col] = char;
                        map[row][col] = '.'
                    }
                    available_col += 1;
                },
                '.' => {},
                _ => panic!("unexpected char {}", char),
            }
        }
    }

    for col in 0..cols_count {
        let mut available_row = rows_count - 1;
        for row in (0..rows_count).rev() {
            let char = map[row][col];
            match char {
                '#' => if row > 0 { available_row = row - 1 },
                'O' => {
                    if available_row != row {
                        map[available_row][col] = char;
                        map[row][col] = '.'
                    }
                    if available_row > 0 {
                        available_row -= 1;
                    }
                },
                '.' => {},
                _ => panic!("unexpected char {}", char),
            }
        }
    }


    for row in 0..rows_count {
        let mut available_col = cols_count - 1;
        for col in (0..cols_count).rev() {
            let char = map[row][col];
            match char {
                '#' => if col > 0 { available_col = col - 1 },
                'O' => {
                    if available_col != col {
                        map[row][available_col] = char;
                        map[row][col] = '.'
                    }
                    if available_col > 0 {
                        available_col -= 1;
                    }
                },
                '.' => {},
                _ => panic!("unexpected char {}", char),
            }
        }
    }
}

#[test]
fn test_do_cycle() {
    let input = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
    let mut lines = split_str(input);

    do_cycle(&mut lines);
    let expected = "
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
";
    let expected = expected.trim();
    assert_eq!(expected, map_to_string(&lines).as_str());

    do_cycle(&mut lines);
    let expected = "
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O
";
    let expected = expected.trim();
    assert_eq!(expected, map_to_string(&lines).as_str());

    do_cycle(&mut lines);
    let expected = "
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
";
    let expected = expected.trim();
    assert_eq!(expected, map_to_string(&lines).as_str());
}

fn calc_current_load(map: &[Vec<char>]) -> usize {
    let cols_count = map[0].len();
    let rows_count = map.len();
    let mut total_weight = 0;
    for row in 0..rows_count {
        for col in 0..cols_count {
            if map[row][col] == 'O' {
                total_weight += rows_count - row;
            }
        }
    }
    total_weight
}

fn map_to_string(map: &[Vec<char>]) -> String {
    map
        .iter()
        .map(|x| x.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}
