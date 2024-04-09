use std::collections::HashMap;
use std::fs::read_to_string;
use std::thread;

fn main() {
    let file_contents = read_to_string("inputs/day12.txt").unwrap();
    let result = process_input(&file_contents, 0);
    println!("{}", result);
    let result = process_input(&file_contents, 5);
    println!("{}", result);
}

fn process_input(input: &str, unfold_times: usize) -> usize {
    /*
    time cargo run (--release)? --bin day12
    debug, no threads: 2.52 secs
    release, no threads: 691.81 millis
    debug, with threads: 677.24 millis
    release, with threads: 451.49 millis
     */
    let use_threads = true;
    if use_threads {
        thread::scope(|scope| {
            let mut threads = vec![];
            for line in input.trim().lines() {
                threads.push(scope.spawn(
                    || get_line_result(line, unfold_times)
                ));
            }
            threads.into_iter().map(|x| x.join().unwrap()).sum()
        })
    } else {
        input.trim().lines().map(|line| get_line_result(line, unfold_times)).sum()
    }
}

#[test]
fn test_process_input() {
    let input = "
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
    let result = process_input(input, 0);
    assert_eq!(21, result);
    let result = process_input(input, 5);
    assert_eq!(525152, result);
}

fn get_line_result(line: &str, unfold_times: usize) -> usize {
    let (map, known_groups) = line.split_once(' ').unwrap();
    let known_groups = known_groups.split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let map = unfold_map(map, unfold_times);
    let known_groups = unfold_groups(known_groups, unfold_times);
    let mut cache = HashMap::new();
    get_possible_fits_count(map.as_bytes(), &known_groups, 0, 0, &mut cache)
}

#[test]
fn test_get_line_result() {
    let unfold = 0;
    assert_eq!(1, get_line_result("### 3", unfold));
    assert_eq!(0, get_line_result("### 2", unfold));
    assert_eq!(0, get_line_result("### 4", unfold));
    assert_eq!(0, get_line_result("### 3,1", unfold));
    assert_eq!(0, get_line_result("### 1,1", unfold));
    assert_eq!(0, get_line_result("### 0", unfold));
    assert_eq!(1, get_line_result(".###. 3", unfold));
    assert_eq!(1, get_line_result("###...# 3,1", unfold));

    assert_eq!(1, get_line_result("??? 1,1", unfold));
    assert_eq!(3, get_line_result("???? 1,1", unfold));
    assert_eq!(6, get_line_result("????? 1,1", unfold));
    assert_eq!(1, get_line_result("???.### 1,1,3", unfold));
    assert_eq!(4, get_line_result("???.?.### 1,1,3", unfold));
    assert_eq!(1, get_line_result("????.#...#... 4,1,1", unfold));
    assert_eq!(4, get_line_result("????.######..#####. 1,6,5", unfold));

    assert_eq!(1, get_line_result("???.### 1,1,3", unfold));
    assert_eq!(4, get_line_result(".??..??...###. 1,1,3", unfold));
    assert_eq!(1, get_line_result("????.#...#... 4,1,1", unfold));
    assert_eq!(4, get_line_result("????.######..#####. 1,6,5", unfold));
    assert_eq!(4, get_line_result(".??..??...?##. 1,1,3", unfold));
    assert_eq!(1, get_line_result("?#?#?#?#?#?#?#? 1,3,1,6", unfold));
    assert_eq!(10, get_line_result("?###???????? 3,2,1", unfold));

    let unfold = 5;
    assert_eq!(1, get_line_result("???.### 1,1,3", unfold));
    assert_eq!(16, get_line_result("????.#...#... 4,1,1", unfold));
    assert_eq!(2500, get_line_result("????.######..#####. 1,6,5", unfold));
    assert_eq!(16384, get_line_result(".??..??...?##. 1,1,3", unfold));
    assert_eq!(1, get_line_result("?#?#?#?#?#?#?#? 1,3,1,6", unfold));
    assert_eq!(506250, get_line_result("?###???????? 3,2,1", unfold));
}

fn unfold_map(map: &str, unfold_times: usize) -> String {
    let mut map = String::from(map);
    if unfold_times > 1 {
        map.push('?');
        map = map.repeat(unfold_times);
        map.pop();
    }
    return map;
}

fn unfold_groups(groups: Vec<usize>, unfold_times: usize) -> Vec<usize> {
    if unfold_times > 1 {
        groups.repeat(unfold_times)
    } else {
        groups
    }
}

#[test]
fn test_unfold() {
    let unfold = 5;
    assert_eq!(".#?.#?.#?.#?.#", &unfold_map(".#", unfold));
    assert_eq!("???.###????.###????.###????.###????.###", &unfold_map("???.###", unfold));
    assert_eq!([1,1,1,1,1], unfold_groups([1].to_vec(), unfold)[..]);
    assert_eq!([1,1,3,1,1,3,1,1,3,1,1,3,1,1,3], unfold_groups([1,1,3].to_vec(), unfold)[..]);
}

fn get_possible_fits_count(map: &[u8], next_groups: &[usize], current_group_actual: usize, current_group_expected: usize, cache: &mut HashMap<(usize, usize, usize, usize), usize>) -> usize {
    if map.len() == 0 {
        if (current_group_actual == current_group_expected) && (next_groups.len() == 0) {
            return 1;
        } else {
            return 0;
        }
    }
    let char = map[0] as char;
    let map = &map[1..];

    let cache_key = (map.len(), next_groups.len(), current_group_actual, current_group_expected);
    if let Some(&cached_value) = cache.get(&cache_key) {
        return cached_value;
    }

    let count_if_empty = if char == '.' || char == '?' {
        if current_group_actual != current_group_expected {
            0
        } else {
            get_possible_fits_count(map, next_groups, 0, 0, cache)
        }
    } else {
        0
    };
    let count_if_filled = if char == '#' || char == '?' {
        let current_group_actual = current_group_actual + 1;
        let mut current_group_expected = current_group_expected;
        let mut next_groups = next_groups;
        if current_group_expected == 0 {
            if next_groups.len() == 0 {
                0
            } else {
                current_group_expected = next_groups[0];
                next_groups = &next_groups[1..];
                get_possible_fits_count(map, next_groups, current_group_actual, current_group_expected, cache)
            }
        } else {
            get_possible_fits_count(map, next_groups, current_group_actual, current_group_expected, cache)
        }
    } else {
        0
    };
    let result = count_if_empty + count_if_filled;
    cache.insert(cache_key, result);
    result
}
