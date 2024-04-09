use std::fs::read_to_string;
use std::mem;

fn main() {
    let file_contents = read_to_string("inputs/day21.txt").unwrap();
    let result = process_input(&file_contents, 64, false);
    println!("{}", result);
    let _result = process_input(&file_contents, 64, true);
}

fn process_input(input: &str, steps_count: usize, is_pt2: bool) -> usize {
    let mut map = input.trim().lines().map(|x| x.as_bytes().to_vec()).collect::<Vec<_>>();
    let (start_row, start_col) = map
        .iter()
        .enumerate()
        .find_map(
            |(row_no, row)|
                row
                    .iter()
                    .enumerate()
                    .find(|(_, &val)| val == b'S')
                    .map(|(col_no, _)| col_no)
                    .and_then(|col_no| Some((row_no, col_no)))
        )
        .unwrap();
    let count_rows = map.iter().count() as isize;
    let count_cols = map[0].iter().count() as isize;
    let mut locations_to_check = vec![];
    let mut next_locations = vec![(start_row as isize, start_col as isize)];
    let mut is_odd_step = false;
    let mut step = 0;
    let mut steps_to_top = None;
    let mut steps_to_top_left = None;
    let mut steps_to_left = None;
    let mut steps_to_bottom_left = None;
    let mut steps_to_bottom = None;
    let mut steps_to_bottom_right = None;
    let mut steps_to_right = None;
    let mut steps_to_top_right = None;
    loop {
        if next_locations.len() == 0 {
            break;
        }
        step += 1;
        if !is_pt2 && (step > steps_count) {
            break;
        }
        is_odd_step = !is_odd_step;
        mem::swap(&mut locations_to_check, &mut next_locations);
        while let Some((current_row, current_col)) = locations_to_check.pop() {
            let possible_next = [
                (current_row - 1, current_col),
                (current_row, current_col - 1),
                (current_row + 1, current_col),
                (current_row, current_col + 1),
            ];
            let set_val = if is_odd_step { b'1' } else { b'2' };
            for (next_row, next_col) in possible_next {
                if (next_row < 0) || (next_row >= count_rows) {
                    continue;
                }
                if (next_col < 0) || (next_col >= count_cols) {
                    continue;
                }
                let next_val = map[next_row as usize][next_col as usize];
                if (next_val == b'#') || (next_val == set_val) {
                    continue;
                }

                let is_top = next_row == 0;
                let is_left = next_col == 0;
                let is_bottom = next_row == count_rows - 1;
                let is_right = next_col == count_cols - 1;

                if steps_to_top.is_none() && is_top {
                    steps_to_top = Some(step);
                }
                if steps_to_top_left.is_none() && is_top && is_left {
                    steps_to_top_left = Some(step);
                }
                if steps_to_left.is_none() && is_left {
                    steps_to_left = Some(step);
                }
                if steps_to_bottom_left.is_none() && is_bottom && is_left {
                    steps_to_bottom_left = Some(step);
                }
                if steps_to_bottom.is_none() && is_bottom {
                    steps_to_bottom = Some(step);
                }
                if steps_to_bottom_right.is_none() && is_bottom && is_right {
                    steps_to_bottom_right = Some(step);
                }
                if steps_to_right.is_none() && is_right {
                    steps_to_right = Some(step);
                }
                if steps_to_top_right.is_none() && is_top && is_right {
                    steps_to_top_right = Some(step);
                }
                map[next_row as usize][next_col as usize] = set_val;
                next_locations.push((next_row, next_col));
            }
        }

        /*let map_str = map
            .iter()
            .map(|line| line.iter().map(|&x| x as char).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        println!("{map_str}");
        println!();*/ // todo: debug
    }
    if is_pt2 {
        let step = step - 1;
        let odd = map
            .iter()
            .map(|row| row.iter().filter(|&val| *val == b'1').count())
            .sum::<usize>();
        let even = map
            .iter()
            .map(|row| row.iter().filter(|&val| *val == b'2').count())
            .sum::<usize>();
        dbg!((
            step,
            steps_to_top.unwrap(),
            steps_to_top_left.unwrap(),
            steps_to_left.unwrap(),
            steps_to_bottom_left.unwrap(),
            steps_to_bottom.unwrap(),
            steps_to_bottom_right.unwrap(),
            steps_to_right.unwrap(),
            steps_to_top_right.unwrap(),
            odd,
            even,
        ));
        1
    } else {
        map
            .iter()
            .map(|row| row.iter().filter(|&val| *val == b'2').count())
            .sum()
    }
}

#[test]
fn test_process_input() {
    let input = "
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";
    let result = process_input(input, 6, false);
    assert_eq!(16, result);

    let result = process_input(input, 6, true);
    assert_eq!(1, result);
}
