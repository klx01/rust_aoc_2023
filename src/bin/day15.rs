use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let file_contents = read_to_string("inputs/day15.txt").unwrap();
    let result = process_input_pt1(&file_contents);
    println!("{}", result);
    let result = process_input_pt2(&file_contents);
    println!("{}", result);
}

fn process_input_pt1(input: &str) -> usize {
    input.trim().split(',').map(calc_hash).sum()
}

#[test]
fn test_process_input() {
    let input = "HASH";
    let result = process_input_pt1(input);
    assert_eq!(52, result);

    let input = "
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";
    let result = process_input_pt1(input);
    assert_eq!(1320, result);

    let result = process_input_pt2(input);
    assert_eq!(145, result);
}

fn process_input_pt2(input: &str) -> usize {
    let mut boxes = vec![vec![]; 256];
    let regex = Regex::new(r"^(\w+)([-=])(\d+)?$").unwrap();
    for step in input.trim().split(',') {
        let parts = regex.captures(step).unwrap();
        let label = parts.get(1).unwrap().as_str();
        let operation = parts.get(2).unwrap().as_str();
        let number = parts.get(3).map(|x| x.as_str());

        let box_no = calc_hash(label);
        let current_box = &mut boxes[box_no];
        let existing_index = current_box
            .iter()
            .enumerate()
            .find(
                |(_, &(existing_label, _, is_active))|
                    is_active && (existing_label == label)
            )
            .map(|(index, _)| index);

        match operation {
            "-" => {
                if let Some(existing_index) = existing_index {
                    current_box[existing_index].2 = false;
                }
            },
            "=" => {
                let focal_length = number.unwrap().parse::<usize>().unwrap();
                if let Some(existing_index) = existing_index {
                    current_box[existing_index].1 = focal_length;
                } else {
                    current_box.push((label, focal_length, true));
                }
            },
            _ => unreachable!(),
        }
    }
    for current_box in boxes.iter_mut() {
        current_box.retain(|&(_, _, is_active)| is_active);
    }
    boxes
        .iter()
        .enumerate()
        .map(
            |(box_index, current_box)|
                current_box
                    .iter()
                    .enumerate()
                    .map(
                        |(lens_index, (_, focal_length, _))|
                            (box_index + 1) * (lens_index + 1) * focal_length
                    )
                    .sum::<usize>()
        )
        .sum::<usize>()
}

fn calc_hash(input: &str) -> usize {
    let mut current_value = 0usize;
    for char in input.chars() {
        current_value += char as usize;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}
