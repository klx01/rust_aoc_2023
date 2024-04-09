use std::{cmp, thread};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    let file_contents = read_to_string("inputs/day23.txt").unwrap();
    let result = process_input(&file_contents, false);
    println!("{}", result);
    let result = process_input(&file_contents, true);
    println!("{}", result);
}

fn process_input(input: &str, is_pt2: bool) -> usize {
    let field = input.trim().lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let start_row = 0;
    let start_col = field[start_row].iter().position(|&x| x == '.').unwrap();
    let start_pos = (start_row as isize, start_col as isize);
    let last_row = field.len() - 1;
    let last_col = field[last_row].iter().position(|&x| x == '.').unwrap();
    let last_pos = (last_row as isize, last_col as isize);

    let mut graph = construct_graph(&field, start_pos, is_pt2);

    /*
    optimisation: the graph is a square grid
    for nodes that are on the perimeter of that grid, we can remove the edges that are leading backwards
    because if such an edge is followed, it would not be possible to arrive at the last node without visiting any node a second time
     */
    let node_after_start = *graph.get(&start_pos).unwrap().keys().next().unwrap();
    let trim_start_nodes = graph
        .get(&node_after_start)
        .unwrap()
        .keys()
        .filter(|&node| *node != start_pos)
        .map(|x| *x)
        .collect::<Vec<_>>();
    for trim_start_node in trim_start_nodes {
        let mut current_node = trim_start_node;
        loop {
            let next_nodes = graph.get(&current_node).unwrap();
            if next_nodes.contains_key(&last_pos) {
                break;
            }
            let next_edge_node = next_nodes
                .keys()
                .filter(|&node| *node != node_after_start)
                .find(|&node| graph.get(node).unwrap().len() < 4)
                .map(|x| *x)
                .unwrap();
            graph.get_mut(&next_edge_node).unwrap().remove(&current_node);
            current_node = next_edge_node;
        }
    }

    /*if is_pt2 {
        for (from_node, to_map) in graph.iter() {
            for (to_node, _steps) in to_map {
                //println!("{from_node:?} {to_node:?} {_steps}");
                println!("{}_{} {}_{}", from_node.0, from_node.1, to_node.0, to_node.1);
            }
        }
    }*/
    get_longest_path_steps(&graph, start_pos, last_row as isize, HashSet::new(), 0)
}

fn construct_graph(field: &[Vec<char>], start_pos: (isize, isize), is_pt2: bool) -> HashMap<(isize, isize), HashMap<(isize, isize), usize>> {
    let mut edges = HashMap::new();
    edges.insert(start_pos, HashMap::new());
    let mut stack = vec![(start_pos, (start_pos.0 + 1, start_pos.1))];
    let mut valid_next = Vec::with_capacity(4);
    let rows_count = field.len() as isize;
    let cols_count = field[0].len() as isize;
    while let Some((from_node, current_pos)) = stack.pop() {
        let mut steps= 0;
        let mut prev_pos = from_node;
        let mut current_pos = current_pos;
        loop {
            steps += 1;
            if current_pos.0 == (rows_count - 1) {
                edges.get_mut(&from_node).unwrap().insert(current_pos, steps);
                break;
            }
            let possible_next = [
                (current_pos.0 + 1, current_pos.1),
                (current_pos.0, current_pos.1 + 1),
                (current_pos.0 - 1, current_pos.1),
                (current_pos.0, current_pos.1 - 1),
            ];
            valid_next.truncate(0);
            for next_pos in possible_next {
                let (next_row, next_col) = next_pos;
                if (next_row < 0) || (next_row >= rows_count) {
                    continue;
                }
                if (next_col < 0) || (next_col >= cols_count) {
                    continue;
                }
                if (next_row == prev_pos.0) && (next_col == prev_pos.1) {
                    continue;
                }
                let next_val = field[next_row as usize][next_col as usize];
                if next_val == '#' {
                    continue;
                }
                valid_next.push(next_pos);
            }
            if valid_next.len() == 1 {
                prev_pos = current_pos;
                current_pos = valid_next[0];
                continue;
            }

            let to_node = current_pos;
            edges.get_mut(&from_node).unwrap().insert(to_node, steps);

            let is_already_processed = edges.get(&to_node).is_some();
            if !is_already_processed {
                edges.insert(to_node, HashMap::new());
            }
            let (prev_row, prev_col) = prev_pos;
            let prev_val = field[prev_row as usize][prev_col as usize];
            let can_go_back = is_pt2 || (prev_val == '.');
            if can_go_back {
                edges.get_mut(&to_node).unwrap().insert(from_node, steps);
            }

            if is_already_processed {
                break;
            }

            for &next_pos in valid_next.iter() {
                if !is_pt2 {
                    let (next_row, next_col) = next_pos;
                    let next_val = field[next_row as usize][next_col as usize];
                    let (current_row, current_col) = current_pos;
                    match next_val {
                        '>' => if next_col <= current_col { continue; },
                        '<' => if next_col >= current_col { continue; },
                        'v' => if next_row <= current_row { continue; },
                        '^' => if next_row >= current_row { continue; },
                        _ => (),
                    }
                }
                stack.push((to_node, next_pos));
            }
            break;
        }
    }
    edges
}

fn get_longest_path_steps(graph: &HashMap<(isize, isize), HashMap<(isize, isize), usize>>, current_node: (isize, isize), end_row: isize, mut visited: HashSet<(isize, isize)>, nest_level: usize) -> usize {
    if current_node.0 == end_row {
        return 0;
    }
    visited.insert(current_node);
    /*
    time cargo run (--release)? --bin day23
    no threads, debug mode - 12.33 sec
    no threads, release mode - 1.83 sec
    with threads for nest_level < 4, debug mode - 2.50 sec
    with threads for nest_level < 4, release mode - 739.71 millis
     */
    if nest_level < 4 {
        thread::scope(|s| {
            let mut threads = vec![];
            for (to_node, edge_steps) in graph.get(&current_node).unwrap() {
                if visited.contains(to_node) {
                    continue;
                }
                threads.push(s.spawn(
                    || get_longest_path_steps(graph, *to_node, end_row, visited.clone(), nest_level + 1) + *edge_steps
                ));
            }
            let mut max_steps = 0;
            for thread in threads {
                max_steps = cmp::max(max_steps, thread.join().unwrap());
            }
            max_steps
        })
    } else {
        let mut max_steps = 0;
        for (to_node, edge_steps) in graph.get(&current_node).unwrap() {
            if visited.contains(to_node) {
                continue;
            }
            max_steps = cmp::max(max_steps, get_longest_path_steps(graph, *to_node, end_row, visited.clone(), nest_level + 1) + *edge_steps);
        }
        max_steps
    }
}

#[test]
fn test_process_input() {
    let input = "
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";
    let result = process_input(input, false);
    assert_eq!(94, result);
    let result = process_input(input, true);
    assert_eq!(154, result);
}
