use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    let file_contents = read_to_string("inputs/day22.txt").unwrap();
    let result = process_input(&file_contents, false);
    println!("{}", result);
    let result = process_input(&file_contents, true);
    println!("{}", result);
}

#[derive(Debug)]
struct Brick([usize; 3], [usize; 3]);

fn process_input(input: &str, is_pt2: bool) -> usize {
    let mut bricks = input.trim().lines().map(parse_brick).collect::<Vec<_>>();
    let sizes = get_field_sizes(&bricks);
    let mut field = vec![vec![vec![0; sizes[2]]; sizes[1]]; sizes[0]];
    bricks.sort_by(compare_bricks);
    let mut bricks_below_map = HashMap::new();
    for (index, brick) in bricks.iter_mut().enumerate() {
        let brick_num = index + 1;

        let mut z_iter = brick.0[2];
        loop {
            let bricks_below_current = get_bricks_below(&field, brick, z_iter);
            if bricks_below_current.len() > 0 {
                bricks_below_map.insert(brick_num, bricks_below_current);
                break;
            } else {
                z_iter -= 1;
            }
        }
        let shift = brick.0[2] - z_iter;
        brick.0[2] = z_iter;
        brick.1[2] -= shift;

        for x in brick.0[0]..=brick.1[0] {
            for y in brick.0[1]..=brick.1[1] {
                for z in brick.0[2]..=brick.1[2] {
                    field[x][y][z] = brick_num;
                }
            }
        }
    }

    let mut bricks_above_map = HashMap::new();
    for (index, brick) in bricks.iter().enumerate() {
        let brick_num = index + 1;
        bricks_above_map.insert(brick_num, get_bricks_above(&field, brick));
    }

    if is_pt2 {
        let mut fall_count = 0;
        for (index, _) in bricks.iter().enumerate() {
            let brick_num = index + 1;
            let mut fall_bricks = bricks_above_map.get(&brick_num)
                .unwrap()
                .iter()
                .filter(
                    |brick_above|
                        bricks_below_map.get(brick_above).unwrap().len() <= 1
                )
                .copied()
                .collect::<HashSet<_>>();
            let mut above_bricks = fall_bricks
                .iter()
                .map(|brick_num| bricks_above_map.get(brick_num).unwrap())
                .flatten()
                .copied()
                .collect::<HashSet<_>>();
            fall_bricks.insert(brick_num);
            loop {
                let mut has_new_falls = false;
                let mut new_above_bricks = HashSet::<usize>::new();
                for above_brick in above_bricks.iter() {
                    let bricks_below_above = bricks_below_map.get(above_brick).unwrap();
                    if bricks_below_above.is_subset(&fall_bricks) {
                        has_new_falls = true;
                        fall_bricks.insert(*above_brick);
                        new_above_bricks.extend(bricks_above_map.get(above_brick).unwrap());
                    }
                }
                if !has_new_falls {
                    break;
                }
                above_bricks = above_bricks.difference(&fall_bricks).copied().collect();
                above_bricks.extend(new_above_bricks);
            }
            let fall_for_current = fall_bricks.iter().count() - 1;
            fall_count += fall_for_current;
        }
        fall_count
    } else {
        bricks
            .iter()
            .enumerate()
            .filter(
                |(i, _)|
                    bricks_above_map.get(&(i + 1))
                        .unwrap()
                        .iter()
                        .find(
                            |brick_above|
                                bricks_below_map.get(brick_above).unwrap().len() <= 1
                        )
                        .is_none()
            )
            .count()
    }
}

fn parse_brick(line: &str) -> Brick {
    let mut iter = line.split('~').map(parse_coordinates);
    let start = iter.next().unwrap();
    let end = iter.next().unwrap();
    if start[2] > end[2] {
        Brick(end, start)
    } else {
        Brick(start, end)
    }
}

fn parse_coordinates(line: &str) -> [usize; 3] {
    let mut iter = line.split(',').map(|x| x.parse::<usize>().unwrap());
    [
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ]
}

fn get_field_sizes(bricks: &[Brick]) -> [usize; 3] {
    let mut iter = (0..3).map(
        |coord|
            bricks.iter().map(|x| cmp::max(x.0[coord], x.1[coord]) + 1).max().unwrap()
    );
    [
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ]
}

fn compare_bricks(x: &Brick, y: &Brick) -> cmp::Ordering {
    x.0[2].partial_cmp(&y.0[2]).unwrap()
}

fn get_bricks_below(field: &[Vec<Vec<usize>>], brick: &Brick, z: usize) -> HashSet<usize> {
    let mut below = HashSet::new();
    if z <= 1 {
        below.insert(0);
        return below;
    }
    for x in brick.0[0]..=brick.1[0] {
        for y in brick.0[1]..=brick.1[1] {
            let brick_num_below = field[x][y][z - 1];
            if brick_num_below > 0 {
                below.insert(brick_num_below);
            }
        }
    }
    below.into_iter().collect()
}

fn get_bricks_above(field: &[Vec<Vec<usize>>], brick: &Brick) -> HashSet<usize> {
    let mut above = HashSet::new();
    for x in brick.0[0]..=brick.1[0] {
        for y in brick.0[1]..=brick.1[1] {
            let brick_num_above = field[x][y][brick.1[2] + 1];
            if brick_num_above > 0 {
                above.insert(brick_num_above);
            }
        }
    }
    above
}

#[test]
fn test_process_input() {
    let input = "
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";
    let result = process_input(input, false);
    assert_eq!(5, result);
    let result = process_input(input, true);
    assert_eq!(7, result);
}
