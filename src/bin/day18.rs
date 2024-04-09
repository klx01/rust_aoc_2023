use std::fs::read_to_string;

fn main() {
    let file_contents = read_to_string("inputs/day18.txt").unwrap();
    let result = process_input(&file_contents, false);
    println!("{}", result);
    let result = process_input(&file_contents, true);
    println!("{}", result);
}

fn process_input(input: &str, is_pt2: bool) -> usize {
    let lines = input.trim().lines().map(|x| parse_line(x, is_pt2)).collect::<Vec<_>>();
    let mut x_next = 0;
    let mut y_next = 0;
    let mut area = 0;
    for (direction, length) in lines {
        let (x_cur, y_cur) = (x_next, y_next);
        match direction {
            'L' => x_next -= length,
            'U' => y_next -= length,
            'R' => x_next += length,
            'D' => y_next += length,
            _ => panic!("unknown direction {direction}"),
        }
        area += (x_cur * y_next) - (x_next * y_cur) + length;
    }
    assert_eq!(0, x_next, "shape is incorrect");
    assert_eq!(0, y_next, "shape is incorrect");
    /*
    shoelace formula should have one more iteration, with (x[last] * y[0]) - (x[0] * y[last])
    but because our starting point is (0,0) the result of that iteration is going to be 0, so we can omit it
     */
    (area.abs() as usize / 2) + 1
}

#[test]
fn test_process_input() {
    let input = "
R 1 (#70c710)
D 1 (#70c710)
L 1 (#70c710)
U 1 (#70c710)
";
    let result = process_input(input, false);
    assert_eq!(4, result);

    let input = "
R 2 (#70c710)
D 2 (#70c710)
L 2 (#70c710)
U 2 (#70c710)
";
    let result = process_input(input, false);
    assert_eq!(9, result);

    let input = "
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";
    let result = process_input(input, false);
    assert_eq!(62, result);
    let result = process_input(input, true);
    assert_eq!(952408144115, result);
}

fn parse_line(line: &str, is_pt2: bool) -> (char, isize) {
    let mut split = line.split(' ');
    let direction = split.next().unwrap().chars().next().unwrap();
    let count = split.next().unwrap().parse().unwrap();
    if !is_pt2 {
        return (direction, count);
    }
    let color = split.next().unwrap();
    let color = &color[2..(color.len() - 1)];
    let (count, direction) = color.split_at(5);
    let direction = match direction {
        "0" => 'R',
        "1" => 'D',
        "2" => 'L',
        "3" => 'U',
        _ => panic!("bad direction in pt2 {}", direction),
    };
    let count = isize::from_str_radix(count, 16).unwrap();
    (direction, count)
}

#[test]
fn test_parse_line() {
    assert_eq!(('R', 6), parse_line("R 6 (#70c710)", false));
    assert_eq!(('R', 461937), parse_line("R 6 (#70c710)", true));
}
