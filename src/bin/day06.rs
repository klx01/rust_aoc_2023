use std::fs::read_to_string;

fn main() {
    let file_contents = read_to_string("inputs/day06.txt").unwrap();
    let result = process_input(&file_contents, true);
    println!("{}", result);
}

fn process_input(input: &str, is_one_number: bool) -> isize {
    let mut times = vec![];
    let mut distances = vec![];
    for line in input.trim().lines() {
        let (line_head, line_body) = line.split_once(':').unwrap();
        match line_head {
            "Time" => times = parse_line(line_body, is_one_number),
            "Distance" => distances = parse_line(line_body, is_one_number),
            _ => panic!("unknown section {}", line_head),
        }
    }
    if times.len() != distances.len() {
        panic!("times num {} does not match distances num {}", times.len(), distances.len());
    }
    times.iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| count_possible_wins(time, distance))
        .fold(1, |res, x| res * x)
}

#[test]
fn test_process_input() {
    let input = "
Time:      7  15   30
Distance:  9  40  200
";
    let result = process_input(input, false);
    assert_eq!(288, result);
    let result = process_input(input, true);
    assert_eq!(71503, result);
}

fn parse_line(line: &str, is_one_number: bool) -> Vec<isize> {
    if is_one_number {
        let one_num = line
            .trim()
            .chars()
            .filter(|&x| x != ' ')
            .collect::<String>()
            .parse()
            .unwrap();
        vec![one_num]
    } else {
        line
            .trim()
            .split(' ')
            .filter(|&x| x != "")
            .map(|x| x.parse().unwrap())
            .collect()
    }
}

#[test]
fn test_parse_line() {
    assert_eq!([7, 15, 30], parse_line("      7  15   30", false)[..]);
    assert_eq!([71530], parse_line("      7  15   30", true)[..]);
}

fn count_possible_wins(time: isize, distance_to_beat: isize) -> isize {
    /*
    hold_time = 0..=time;
    my_dist = speed * time_left
    speed = hold_time
    time_left = time - hold_time
    my_dist = hold_time * (time - hold_time)

    my_dist(hold_time) vs my_dist(time - hold_time) - ?
    my_dist(time - hold_time) = (time - hold_time) * (time - (time - hold_time)) = (time - hold_time) * (hold_time) = hold_time * (time - hold_time) = my_dist(hold_time)
    my_dist(hold_time) = my_dist(time - hold_time)

    distance = hold_time * (time - hold_time)
    ht = x
    x * (t - x) = d
    -x^2 + tx - d = 0
    D = t^2 - 4(-1 * (-d)) = t^2 - 4d
    x = (-t +- sqrt(D)) / 2*(-1) = (t +- sqrt(D)) / 2
     */
    let d = (time * time) - (4 * distance_to_beat);
    if d < 0 {
        panic!("impossible distance {} for time {}", distance_to_beat, time);
    }
    let hold_time_to_beat = ((time as f32) - (d as f32).sqrt()) / 2f32;
    /*
    min_hold_time should be a next int after hold_time_to_beat
    so if hold_time_to_beat is 9.9, then min_hold_time should be 10
    but if hold_time_to_beat is 10.0 then min_hold_time should be 11
    which is not always correctly handled just by ceil
     */
    let mut min_hold_time = hold_time_to_beat.ceil() as isize;
    if (min_hold_time * (time - min_hold_time)) <= distance_to_beat {
        min_hold_time += 1;
    }
    /*
    total number of options is 0..=time, so count is time+1
    we need to get count of min_hold_time..=(time - min_hold_time)
    which is the same count as 0..=(time - 2*min_hold_time), which is time - 2*min_hold_time + 1
     */
    time - min_hold_time - min_hold_time + 1
}

#[test]
fn test_count_possible_wins() {
    assert_eq!(4, count_possible_wins(7, 9));
    assert_eq!(8, count_possible_wins(15, 40));
    assert_eq!(9, count_possible_wins(30, 200));
}
