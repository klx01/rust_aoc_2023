use std::fs::read_to_string;
use std::cmp;

fn main() {
    let file_contents = read_to_string("inputs/day02.txt").unwrap();
    let result = process_input_pt1(&file_contents, RED_COUNT, GREEN_COUNT, BLUE_COUNT);
    println!("{}", result);
    let result = process_input_pt2(&file_contents);
    println!("{}", result);
}

const RED_COUNT: usize = 12;
const GREEN_COUNT: usize = 13;
const BLUE_COUNT: usize = 14;

fn process_input_pt1(input: &str, red_count: usize, green_count: usize, blue_count: usize) -> usize {
    input.trim().lines().map(|line| get_line_result_pt1(line, red_count, green_count, blue_count)).sum()
}

#[test]
fn test_process_input() {
    let input = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
    let result = process_input_pt1(input, RED_COUNT, GREEN_COUNT, BLUE_COUNT);
    assert_eq!(8, result);

    let result = process_input_pt2(input);
    assert_eq!(2286, result);
}

fn get_line_result_pt1(line: &str, red_count: usize, green_count: usize, blue_count: usize) -> usize {
    let (game, tries) = line.split_once(':').unwrap();
    let impossible_try = tries.split(';').find(|try_str| is_impossible_try(try_str, red_count, green_count, blue_count));
    match impossible_try {
        None => parse_game_id(game),
        Some(_) => 0
    }
}

#[test]
fn test_get_line_result_pt1() {
    assert_eq!(2, get_line_result_pt1("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", RED_COUNT, GREEN_COUNT, BLUE_COUNT));
    assert_eq!(0, get_line_result_pt1("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", RED_COUNT, GREEN_COUNT, BLUE_COUNT));
}

fn is_impossible_try(try_str: &str, red_count: usize, green_count: usize, blue_count: usize) -> bool {
    let counts = parse_counts(try_str);
    let is_impossible = (counts.red > red_count) || (counts.green > green_count) || (counts.blue > blue_count);
    is_impossible
}

#[test]
fn test_is_impossible_try() {
    assert_eq!(false, is_impossible_try("1 blue, 2 green", RED_COUNT, GREEN_COUNT, BLUE_COUNT));
    assert_eq!(true, is_impossible_try("8 green, 6 blue, 20 red", RED_COUNT, GREEN_COUNT, BLUE_COUNT));
}


#[derive(PartialEq, Debug)]
struct Counts {
    red: usize,
    green: usize,
    blue: usize,
}

fn parse_counts(try_str: &str) -> Counts {
    let mut counts = Counts{red: 0, green: 0, blue: 0};
    for count_str in try_str.split(',') {
        let (num_str, color) = count_str.trim().split_once(' ').unwrap();
        let num = num_str.parse().unwrap();
        match color {
            "red" => counts.red = num,
            "green" => counts.green = num,
            "blue" => counts.blue = num,
            _ => panic!("unexpected color {}", color),
        }
    }
    counts
}

#[test]
fn test_parse_counts() {
    assert_eq!(Counts{red: 0, green: 2, blue: 1}, parse_counts("1 blue, 2 green"));
    assert_eq!(Counts{red: 20, green: 8, blue: 6}, parse_counts("8 green, 6 blue, 20 red"));
}

fn parse_game_id(str: &str) -> usize {
    let (_, id_str) = str.split_once(' ').unwrap();
    id_str.parse().unwrap()
}

#[test]
fn test_parse_game_id() {
    assert_eq!(1, parse_game_id("Game 1"));
    assert_eq!(3, parse_game_id("Game 3"));
}

fn process_input_pt2(input: &str) -> usize {
    input.trim().lines().map(|line| get_line_result_pt2(line)).sum()
}

fn get_line_result_pt2(line: &str) -> usize {
    let (_, tries) = line.split_once(':').unwrap();
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    let tries_counts = tries.split(';').map(|try_str| parse_counts(try_str));
    for try_counts in tries_counts {
        max_red = cmp::max(max_red, try_counts.red);
        max_green = cmp::max(max_green, try_counts.green);
        max_blue = cmp::max(max_blue, try_counts.blue);
    }
    max_red * max_green * max_blue
}

#[test]
fn test_get_line_result_pt2() {
    assert_eq!(12, get_line_result_pt2("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"));
    assert_eq!(1560, get_line_result_pt2("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"));
}
