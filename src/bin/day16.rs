use std::fs::read_to_string;

fn main() {
    let file_contents = read_to_string("inputs/day16.txt").unwrap();
    let result = process_input_pt1(&file_contents);
    println!("{}", result);
    let result = process_input_pt2(&file_contents);
    println!("{}", result);
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum CameFrom {
    Left,
    Top,
    Right,
    Bottom,
}

fn process_input_pt1(input: &str) -> usize {
    let map = parse_input(input);
    count_energized(&map, (0, 0, CameFrom::Left))
}

fn process_input_pt2(input: &str) -> usize {
    let map = parse_input(input);

    let mut attempts = vec![];
    let last_row = (map.len() - 1) as isize;
    let last_col = (map[0].len() - 1) as isize;
    for row in 0..=last_row {
        attempts.push((row, 0, CameFrom::Left));
        attempts.push((row, last_col, CameFrom::Right));
    }
    for col in 0..=last_col {
        attempts.push((0, col, CameFrom::Top));
        attempts.push((last_row, col, CameFrom::Bottom));
    }

    attempts
        .iter()
        .map(|&start_from| count_energized(&map, start_from))
        .max()
        .unwrap()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn count_energized(map: &[Vec<char>], start_from: (isize, isize, CameFrom)) -> usize {
    let mut map = map.iter().map(|row| row.clone()).collect::<Vec<_>>();
    let rows_count = map.len() as isize;
    let cols_count = map[0].len() as isize;
    let mut stack = vec![start_from];
    let mut iterations = 0usize;
    while let Some((row, col, came_from)) = stack.pop() {
        iterations += 1;
        if iterations > 100000 {
            panic!("too many iterations");
        }
        if (row < 0) || (col < 0) || (row >= rows_count) || (col >= cols_count) {
            continue;
        }

        let row_usize = row as usize;
        let col_usize = col as usize;
        let char = map[row_usize][col_usize];
        let is_horizontal = (came_from == CameFrom::Left) || (came_from == CameFrom::Right);
        match (char, is_horizontal) {
            ('+', _) => continue,
            ('=', true) => continue,
            ('!', false) => continue,
            _ => (),
        }
        match char {
            '=' => map[row_usize][col_usize] = '+',
            '!' => map[row_usize][col_usize] = '+',
            '.' => map[row_usize][col_usize] = if is_horizontal { '=' } else { '!' },
            '\\' => map[row_usize][col_usize] = 'a',
            '/' => map[row_usize][col_usize] = 'b',
            '|' => map[row_usize][col_usize] = 'c',
            '-' => map[row_usize][col_usize] = 'd',
            _ => (),
        }
        match char {
            '\\' | 'a' => {
                let next = match came_from {
                    CameFrom::Left => (row + 1, col, CameFrom::Top),
                    CameFrom::Top => (row, col + 1, CameFrom::Left),
                    CameFrom::Right => (row - 1, col, CameFrom::Bottom),
                    CameFrom::Bottom => (row, col - 1, CameFrom::Right),
                };
                stack.push(next);
            },
            '/' | 'b' => {
                let next = match came_from {
                    CameFrom::Left => (row - 1, col, CameFrom::Bottom),
                    CameFrom::Top => (row, col - 1, CameFrom::Right),
                    CameFrom::Right => (row + 1, col, CameFrom::Top),
                    CameFrom::Bottom => (row, col + 1, CameFrom::Left),
                };
                stack.push(next);
            },
            '|' | 'c' => match came_from {
                CameFrom::Left | CameFrom::Right => {
                    stack.push((row + 1, col, CameFrom::Top));
                    stack.push((row - 1, col, CameFrom::Bottom));
                },
                CameFrom::Top => stack.push((row + 1, col, came_from)),
                CameFrom::Bottom => stack.push((row - 1, col, came_from)),
            },
            '-' | 'd' => match came_from {
                CameFrom::Top | CameFrom::Bottom => {
                    stack.push((row, col + 1, CameFrom::Left));
                    stack.push((row, col - 1, CameFrom::Right));
                },
                CameFrom::Left => stack.push((row, col + 1, came_from)),
                CameFrom::Right => stack.push((row, col - 1, came_from)),
            },
            _ => {
                let next = match came_from {
                    CameFrom::Left => (row, col + 1, came_from),
                    CameFrom::Top => (row + 1, col, came_from),
                    CameFrom::Right => (row, col - 1, came_from),
                    CameFrom::Bottom => (row - 1, col, came_from),
                };
                stack.push(next);
            }
        }
    }

    map
        .iter()
        .map(
            |row|
                row.iter().map(
                    |&x|
                        ((x != '.') && (x != '\\') && (x != '/') && (x != '|') && (x != '-')) as usize
                )
                    .sum::<usize>()
        )
        .sum()
}

#[test]
fn test_process_input() {
    let input = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";
    let result = process_input_pt1(input);
    assert_eq!(46, result);

    let result = process_input_pt2(input);
    assert_eq!(51, result);
}
