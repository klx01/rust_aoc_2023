use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let file_contents = read_to_string("inputs/day08.txt").unwrap();
    let result = process_input(&file_contents);
    println!("{}", result);
}

fn process_input(input: &str) -> usize {
    let (moves, map_str) = input.trim().split_once('\n').unwrap();
    let moves = moves.trim().chars().collect::<Vec<_>>();
    let map = parse_map(map_str);

    /*
    the description does not explicitly state this, but each path reaches Z after a set amount of iterations
    i.e. while following a path you can't encounter a Z after 3 steps, then after 8 more, then after 2 more.
    if you've encountered a Z after 3 steps, then the next one will be after 3 more, and then after 3 more again
    which is what makes this solution possible
     */
    map
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|&x| count_iterations_until_z(&moves, x, &map))
        .fold(1, least_common_multiple)
}

#[test]
fn test_process_input() {
    let input = "
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
    let result = process_input(input);
    assert_eq!(2, result);

    let input = "
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
    let result = process_input(input);
    assert_eq!(6, result);

    let input = "
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
    let result = process_input(input);
    assert_eq!(6, result);
}

fn parse_map(input: &str) -> HashMap<&str, (&str, &str)> {
    let mut map = HashMap::new();
    for line in input.trim().lines() {
        let (key, data) = line.split_once('=').unwrap();
        let data = data.trim().trim_start_matches('(').trim_end_matches(')');
        let (left, right) = data.split_once(',').unwrap();
        map.insert(key.trim(), (left.trim(), right.trim()));
    }
    map
}

fn count_iterations_until_z(moves: &[char], starting_key: &str, map: &HashMap<&str, (&str, &str)>) -> usize {
    let mut current_key = starting_key;
    let mut iter_count = 0usize;
    'outer: loop {
        for &move_char in moves.iter() {
            if current_key.ends_with('Z') {
                break 'outer;
            }
            if iter_count > 1000000 {
                panic!("too many iterations for starting key {}", starting_key);
            }
            iter_count += 1;
            let next = map.get(current_key).unwrap();
            current_key = match move_char {
                'L' => next.0,
                'R' => next.1,
                _ => unreachable!(),
            }
        }
    }
    iter_count
}

fn least_common_multiple(a: usize, b: usize) -> usize {
    (a / greatest_common_divisor(a, b)) * b
}

fn greatest_common_divisor(a: usize, b: usize) -> usize {
    let mut b = b;
    let mut a = a;
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

