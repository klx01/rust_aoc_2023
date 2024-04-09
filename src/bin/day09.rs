use std::fs::read_to_string;

fn main() {
    let file_contents = read_to_string("inputs/day09.txt").unwrap();
    let result = process_input(&file_contents, false);
    println!("{}", result);
    let result = process_input(&file_contents, true);
    println!("{}", result);
}

fn process_input(input: &str, is_predict_back: bool) -> isize {
    input.trim().lines().map(|x| process_line(x, is_predict_back)).sum()
}

fn process_line(line: &str, is_predict_back: bool) -> isize {
    let nums = line.trim().split(' ').map(|x| x.parse().unwrap()).collect::<Vec<_>>();
    predict_next_value(&nums, is_predict_back)
}

#[test]
fn test_process_input() {
    let input = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
    let result = process_input(input, false);
    assert_eq!(114, result);
    let result = process_input(input, true);
    assert_eq!(2, result);
}

fn predict_next_value(sequence: &[isize], is_predict_back: bool) -> isize {
    let mut all_diffs = vec![sequence.to_owned()];
    let mut iter_count = 0usize;
    loop {
        let prev_diffs = all_diffs.last().unwrap();
        if prev_diffs.iter().all(|&x| x == 0) {
            break;
        }
        iter_count += 1;
        if iter_count > 100000 {
            panic!("too many iterations for sequence {:?}", sequence);
        }
        let mut diffs = vec![];
        for i in 0..(prev_diffs.len() - 1) {
            diffs.push(prev_diffs[i + 1] - prev_diffs[i]);
        }
        all_diffs.push(diffs);
    }
    if is_predict_back {
        all_diffs.iter().rev().fold(0, |acc, x| x[0] - acc)
    } else {
        all_diffs.iter().map(|x| x.last().unwrap()).sum()
    }
}

#[test]
fn test_predict_next_value() {
    assert_eq!(18, predict_next_value(&[0, 3, 6, 9, 12, 15], false));
    assert_eq!(28, predict_next_value(&[1, 3, 6, 10, 15, 21], false));
    assert_eq!(68, predict_next_value(&[10, 13, 16, 21, 30, 45], false));
    assert_eq!(-3, predict_next_value(&[0, 3, 6, 9, 12, 15], true));
    assert_eq!(0, predict_next_value(&[1, 3, 6, 10, 15, 21], true));
    assert_eq!(5, predict_next_value(&[10, 13, 16, 21, 30, 45], true));
}
