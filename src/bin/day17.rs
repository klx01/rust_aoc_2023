use std::cmp;
use std::collections::{BinaryHeap, HashMap};
use std::fs::read_to_string;

fn main() {
    let file_contents = read_to_string("inputs/day17.txt").unwrap();
    let result = process_input(&file_contents, 1, 3);
    println!("{}", result);
    let result = process_input(&file_contents, 4, 10);
    println!("{}", result);
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, Ord, PartialOrd)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

fn process_input(input: &str, min_steps: usize, max_steps: usize) -> usize {
    let map = input
        .trim()
        .lines()
        .map(
            |line|
                line
                    .chars()
                    .map(|x| x.to_digit(10).unwrap() as usize)
                    .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let last_row = map.len() as isize - 1;
    let last_col = map[0].len() as isize - 1;
    let mut queue = BinaryHeap::from([
        (0isize, 0usize, (0isize, 0isize, false)),
        (0isize, 0usize, (0isize, 0isize, true)),
    ]);
    let mut cache = HashMap::new();
    cache.insert((0isize, 0isize, false), 0);
    cache.insert((0isize, 0isize, true), 0);
    let mut came_from = HashMap::new();
    let mut iterations = 0usize;
    while let Some((_, total_heat_loss, current_state)) = queue.pop() {
        iterations += 1;
        if iterations > 1000000 {
            panic!("too many iterations");
        }

        let (row, col, was_horizontal) = current_state;
        if (row == last_row) && (col == last_col) {
            //_dump_trace(&map, &came_from, current_state); // debug
            //println!("{} {}", total_heat_loss, iterations); // debug
            return total_heat_loss;
        }

        for new_direction in [Direction::Left, Direction::Up, Direction::Right, Direction::Down] {
            let is_allowed = match new_direction {
                Direction::Left | Direction::Right => !was_horizontal,
                Direction::Up | Direction::Down => was_horizontal,
            };
            if !is_allowed {
                continue;
            }
            let (row_shift, col_shift) = match new_direction {
                Direction::Left => (0, -1),
                Direction::Up => (-1, 0),
                Direction::Right => (0, 1),
                Direction::Down => (1, 0),
            };

            let mut new_row = row;
            let mut new_col = col;
            let mut new_heat_loss = total_heat_loss;
            for steps in 1..=max_steps {
                new_row += row_shift;
                new_col += col_shift;

                if (new_row < 0) || (new_row > last_row) {
                    break;
                }
                if (new_col < 0) || (new_col > last_col) {
                    break;
                }

                new_heat_loss += map[new_row as usize][new_col as usize];

                if steps < min_steps {
                    continue;
                }

                let new_state = (new_row, new_col, !was_horizontal);
                if let Some(&exists_heat_loss) = cache.get(&new_state) {
                    if new_heat_loss >= exists_heat_loss {
                        continue;
                    }
                }
                cache.insert(new_state, new_heat_loss);
                came_from.insert(new_state, current_state);

                let new_cost = new_heat_loss as isize + (last_row - new_row) + (last_col - new_col);
                queue.push((-new_cost, new_heat_loss, new_state));
            }
        }
    }
    panic!("loop ended in {} iterations without reaching the goal", iterations);
}

#[test]
fn test_process_input() {
    let input = "
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
    let result = process_input(input, 1, 3);
    assert_eq!(102, result);
    let result = process_input(input, 4, 10);
    assert_eq!(94, result);
    let input = "
111111111111
999999999991
999999999991
999999999991
999999999991
";
    let result = process_input(input, 4, 10);
    assert_eq!(71, result);
}

// debug
fn _dump_trace(map: &[Vec<usize>], came_from: &HashMap<(isize, isize, bool), (isize, isize, bool)>, current_state: (isize, isize, bool)) {
    let mut map = map.iter().map(|x| x.clone()).collect::<Vec<_>>();
    let mut trace_state = current_state;
    while let Some(&came_from_state) = came_from.get(&trace_state) {
        let (from_row, from_col, _) = came_from_state;
        if map[from_row as usize][from_col as usize] == 0 {
            println!("{:?} {:?}", trace_state, came_from_state);
            break;
        }
        let row_from = cmp::min(trace_state.0, came_from_state.0);
        let row_to = cmp::max(trace_state.0, came_from_state.0);
        let col_from = cmp::min(trace_state.1, came_from_state.1);
        let col_to = cmp::max(trace_state.1, came_from_state.1);
        for row in row_from..=row_to {
            for col in col_from..=col_to {
                map[row as usize][col as usize] = 0;
            }
        }
        if (from_row == 0) && (from_col == 0) {
            break;
        }

        trace_state = came_from_state;
    }
    let map = map
        .iter()
        .map(
            |line|
                line
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<String>()
        )
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", map);
}
