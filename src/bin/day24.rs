use std::fs::read_to_string;

fn main() {
    let file_contents = read_to_string("inputs/day24.txt").unwrap();
    let result = process_input_pt1(&file_contents, 200000000000000, 400000000000000);
    println!("{}", result);
    let result = process_input_pt2(&file_contents);
    println!("{}", result);
}

#[derive(Debug)]
struct Stone {
    pos: Position,
    vel: Velocity,
}

#[derive(Debug, PartialEq)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, PartialEq)]
struct Velocity {
    x: isize,
    y: isize,
    z: isize,
}

fn process_input_pt1(input: &str, coord_min: usize, coord_max: usize) -> usize {
    let stones = input.trim().lines().map(parse_stone).collect::<Vec<_>>();
    let coord_min = coord_min as f64;
    let coord_max = coord_max as f64;
    let mut count = 0;
    for (index, stone1) in stones.iter().enumerate() {
        for stone2 in &stones[index + 1..] {
            if is_intersect_within(stone1, stone2, coord_min, coord_max) {
                count += 1;
            }
        }
    }
    count
}

fn parse_stone(line: &str) -> Stone {
    let (position, velocity) = line.split_once('@').unwrap();
    let mut position = position.trim().split(',').map(|x| x.trim().parse().unwrap());
    let mut velocity = velocity.trim().split(',').map(|x| x.trim().parse().unwrap());
    Stone{
        pos: Position {
            x: position.next().unwrap(),
            y: position.next().unwrap(),
            z: position.next().unwrap(),
        },
        vel: Velocity {
            x: velocity.next().unwrap(),
            y: velocity.next().unwrap(),
            z: velocity.next().unwrap(),
        }
    }
}

fn is_intersect_within(stone1: &Stone, stone2: &Stone, coord_min: f64, coord_max: f64) -> bool {
    /*
    (x - xa) / (xb - xa) = (y - ya) / (yb - ya)
    xb - xa = vel.x
    (x - pos.x) / vel.x = (y - pos.y) / vel.y
    (x - pos.x)vel.y/vel.x = y - pos.y
    (x - pos.x)vel.y/vel.x + pos.y = y
    y = (vel.y/vel.x)x + (pos.y - pos.x(vel.y/vel.x))
    set (vel.y/vel.x) = k
    set (pos.y - pos.x(vel.y/vel.x)) = b
    y = kx + b
    find intersections
    y = k1x + b1
    y = k2x + b2
    0 = (k1 - k2)x + b1 - b2
    (k1 - k2)x = b2 - b1
    x = (b2 - b1) / (k1 - k2)
    if (k1 - k2) = 0 then they don't intersect
     */
    let k1 = stone1.vel.y as f64 / stone1.vel.x as f64;
    let k2 = stone2.vel.y as f64 / stone2.vel.x as f64;
    let k_diff = k1 - k2;
    if k_diff.abs() < 0.0000001 {
        //println!("no intersection");
        return false;
    }
    let b1 = (stone1.pos.y as f64) - (stone1.pos.x as f64 * k1);
    let b2 = (stone2.pos.y as f64) - (stone2.pos.x as f64 * k2);
    let intersect_x = (b2 - b1) / (k1 - k2);
    let intersect_y = (k1 * intersect_x) + b1;
    if (intersect_x < coord_min) || (intersect_x > coord_max) {
        //println!("x out of bounds");
        return false;
    }
    if (intersect_y < coord_min) || (intersect_y > coord_max) {
        //println!("y out of bounds");
        return false;
    }
    let x_diff1 = intersect_x - (stone1.pos.x as f64);
    if (x_diff1 > 0.0) != (stone1.vel.x > 0) {
        //println!("intersection x is in the past for stone 1");
        return false;
    }
    let x_diff2 = intersect_x - (stone2.pos.x as f64);
    if (x_diff2 > 0.0) != (stone2.vel.x > 0) {
        //println!("intersection x is in the past for stone 2");
        return false;
    }
    let y_diff1 = intersect_y - (stone1.pos.y as f64);
    if (y_diff1 > 0.0) != (stone1.vel.y > 0) {
        //println!("intersection y is in the past for stone 1");
        return false;
    }
    let y_diff2 = intersect_y - (stone2.pos.y as f64);
    if (y_diff2 > 0.0) != (stone2.vel.y > 0) {
        //println!("intersection y is in the past for stone 2");
        return false;
    }
    //println!("intersect in bounds");
    return true;
}

fn process_input_pt2(input: &str) -> usize {
    let stones = input.trim().lines().map(parse_stone).collect::<Vec<_>>();
    /*
    todo: need to intersect all lines
        at different times in the future
     */
    1
}

#[test]
fn test_process_input() {
    let input = "
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";
    let result = process_input_pt1(input, 7, 27);
    assert_eq!(2, result);
    // todo: test it when it's implemented
    //let result = process_input_pt2(input);
    //assert_eq!(47, result);
}
