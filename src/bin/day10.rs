use std::fs::read_to_string;

fn main() {
    let file_contents = read_to_string("inputs/day10.txt").unwrap();
    let (part1, part2) = process_input(&file_contents);
    println!("{}", part1);
    println!("{}", part2);
}

const TOP_BOTTOM: char = '|';
const LEFT_RIGHT: char = '-';
const TOP_RIGHT: char = 'L';
const TOP_LEFT: char = 'J';
const BOTTOM_RIGHT: char = 'F';
const BOTTOM_LEFT: char = '7';

const COUNT_IS_PATH: u8 = 9;

#[derive(Debug, Clone, Copy, PartialEq)]
enum CameFrom {
    Top,
    Bottom,
    Left,
    Right,
}

fn process_input(input: &str) -> (usize, usize) {
    let input = input.trim();
    let start_position = input.find('S').unwrap();
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    if map.len() == 0 {
        panic!("empty map");
    }
    let (path, is_clockwise) = find_path(&map, start_position);

    let steps_to_farthest = path.len() / 2;

    (steps_to_farthest, count_enclosed(&map, &path, is_clockwise))
}

#[test]
fn test_process_input() {
    let input = "
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";
    let (result, _) = process_input(input);
    assert_eq!(8, result);

    let input = "
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";
    let (_, result) = process_input(input);
    assert_eq!(4, result);

    let input = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";
    let (_, result) = process_input(input);
    assert_eq!(10, result);
}

fn find_path(map: &[Vec<char>], start_position: usize) -> (Vec<((usize, usize), CameFrom)>, bool) {
    let start_position = start_position as isize;
    let cols_count = map[0].len() as isize;
    let start_row = start_position / (cols_count + 1); // + 1 because start_position had new lines, but "map" does not
    let start_col = start_position % (cols_count + 1);
    let checks = [
        ((start_row - 1, start_col), CameFrom::Bottom),
        ((start_row, start_col + 1), CameFrom::Left),
        ((start_row + 1, start_col), CameFrom::Top),
        ((start_row, start_col - 1), CameFrom::Right), // the last one is probably always unreachable
    ];
    for ((start_check_row, start_check_col), start_came_from) in checks {
        if let Some(result) = trace_path(&map, start_check_row, start_check_col, start_came_from) {
            return result;
        }
    }
    panic!("no loops found?");
}

fn trace_path(map: &[Vec<char>], start_check_row: isize, start_check_col: isize, start_came_from: CameFrom) -> Option<(Vec<((usize, usize), CameFrom)>, bool)> {
    let mut check_row = start_check_row;
    let mut check_col = start_check_col;
    let mut came_from = start_came_from;
    let mut count_steps = 1;
    let mut rotation = 0; // positive - clockwise, negative - counter-clockwise
    let mut path = vec![];
    let rows_count = map.len() as isize;
    let cols_count = map[0].len() as isize;
    loop {
        if (check_row < 0) || (check_row >= rows_count) {
            return None;
        }
        if (check_col < 0) || (check_col >= cols_count) {
            return None;
        }
        let char = map[check_row as usize][check_col as usize];
        path.push(((check_row as usize, check_col as usize), came_from));
        if char == 'S' {
            break;
        }
        let (move_row, move_col, next_came_from, add_rotation) = match (came_from, char) {
            (CameFrom::Top, TOP_BOTTOM) => (1, 0, CameFrom::Top, 0),
            (CameFrom::Top, TOP_RIGHT) => (0, 1, CameFrom::Left, -1),
            (CameFrom::Top, TOP_LEFT) => (0, -1, CameFrom::Right, 1),
            (CameFrom::Bottom, TOP_BOTTOM) => (-1, 0, CameFrom::Bottom, 0),
            (CameFrom::Bottom, BOTTOM_RIGHT) => (0, 1, CameFrom::Left, 1),
            (CameFrom::Bottom, BOTTOM_LEFT) => (0, -1, CameFrom::Right, -1),
            (CameFrom::Left, LEFT_RIGHT) => (0, 1, CameFrom::Left, 0),
            (CameFrom::Left, TOP_LEFT) => (-1, 0, CameFrom::Bottom, -1),
            (CameFrom::Left, BOTTOM_LEFT) => (1, 0, CameFrom::Top, 1),
            (CameFrom::Right, LEFT_RIGHT) => (0, -1, CameFrom::Right, 0),
            (CameFrom::Right, TOP_RIGHT) => (-1, 0, CameFrom::Bottom, 1),
            (CameFrom::Right, BOTTOM_RIGHT) => (1, 0, CameFrom::Top, -1),
            _ => return None,
        };
        check_row += move_row;
        check_col += move_col;
        came_from = next_came_from;
        count_steps += 1;
        rotation += add_rotation;
        if count_steps > 100000 {
            panic!("too many steps for direction {:?}", start_came_from);
        }
    }
    Some((path, rotation > 0))
}

fn count_enclosed(map: &[Vec<char>], path: &[((usize, usize), CameFrom)], is_clockwise: bool) -> usize {
    let rows_count = map.len();
    let cols_count = map[0].len();
    let mut counts_map = vec![vec![0u8; cols_count]; rows_count];
    for &((row, col), _) in path.iter() {
        counts_map[row][col] = COUNT_IS_PATH;
    }
    let shift = if is_clockwise { 1 } else { -1 };
    for (index, &((path_row, path_col), came_from)) in path.iter().enumerate() {
        mark_as_internal(&mut counts_map, path_row, path_col, came_from, shift);
        let next_step = path.get(index + 1).or(Some(&path[0])).unwrap();
        if next_step.1 != came_from {
            // correctly process bends
            mark_as_internal(&mut counts_map, path_row, path_col, next_step.1, shift);
        }
    }
    if counts_map.iter().find(|row| row.iter().find(|&&x| (x != 0) && (x != 4) && (x != COUNT_IS_PATH)).is_some()).is_some() {
        for row in counts_map.iter() {
            println!("{:?}", row);
        }
        panic!("found unexpected count");
    }
    counts_map
        .iter()
        .map(
            |row|
            row
                .iter()
                .filter(|&&x| x == 4)
                .count()
        )
        .sum()
}

fn mark_as_internal(counts_map: &mut [Vec<u8>], path_row: usize, path_col: usize, came_from: CameFrom, shift: isize) {
    let rows_count = counts_map.len() as isize;
    let cols_count = counts_map[0].len() as isize;
    let (move_row, move_col) = match came_from {
        CameFrom::Top => (0, -shift),
        CameFrom::Bottom => (0, shift),
        CameFrom::Left => (shift, 0),
        CameFrom::Right => (-shift, 0),
    };
    let mut check_row = path_row as isize;
    let mut check_col = path_col as isize;
    let mut iter_count = 0;
    loop {
        check_row += move_row;
        check_col += move_col;
        if (check_row < 0) || (check_row >= rows_count) {
            break;
        }
        if (check_col < 0) || (check_col >= cols_count) {
            break;
        }
        if counts_map[check_row as usize][check_col as usize] == COUNT_IS_PATH {
            break;
        }
        // should be possible to just set to 1 instead of increasing counter by 1
        counts_map[check_row as usize][check_col as usize] += 1;
        iter_count += 1;
        if iter_count > 100000 {
            panic!("too many iterations for {} {}", path_row, path_col);
        }
    }
}