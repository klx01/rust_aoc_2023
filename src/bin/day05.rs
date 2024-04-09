use std::fs::read_to_string;
use std::cmp;

fn main() {
    let file_contents = read_to_string("inputs/day05.txt").unwrap();
    let result = process_input(&file_contents, false);
    println!("{}", result);
    let result = process_input(&file_contents, true);
    println!("{}", result);
}

fn process_input(input: &str, parse_as_ranges: bool) -> isize {
    let sections = input.trim().split("\n\n").filter(|&x| x != "");
    let mut seeds = vec![];
    let mut seed_to_soil_map = vec![];
    let mut soil_to_fertilizer_map = vec![];
    let mut fertilizer_to_water_map = vec![];
    let mut water_to_light_map = vec![];
    let mut light_to_temperature_map = vec![];
    let mut temperature_to_humidity_map = vec![];
    let mut humidity_to_location_map = vec![];
    for section in sections {
        let (section_head, section_body) = section.split_once(':').unwrap();
        match section_head {
            "seeds" => seeds = parse_seeds(section_body, parse_as_ranges),
            _ => {
                let parsed_map = parse_map(section_body);
                match section_head {
                    "seed-to-soil map" => seed_to_soil_map = parsed_map,
                    "soil-to-fertilizer map" => soil_to_fertilizer_map = parsed_map,
                    "fertilizer-to-water map" => fertilizer_to_water_map = parsed_map,
                    "water-to-light map" => water_to_light_map = parsed_map,
                    "light-to-temperature map" => light_to_temperature_map = parsed_map,
                    "temperature-to-humidity map" => temperature_to_humidity_map = parsed_map,
                    "humidity-to-location map" => humidity_to_location_map = parsed_map,
                    _ => panic!("unknown section {}", section_head),
                }
            }
        }
    }
    let map_stages = [
        seed_to_soil_map,
        soil_to_fertilizer_map,
        fertilizer_to_water_map,
        water_to_light_map,
        light_to_temperature_map,
        temperature_to_humidity_map,
        humidity_to_location_map,
    ];
    let mut mapped_ranges = seeds;
    for stage in map_stages {
        mapped_ranges = map_ranges(&mapped_ranges, &stage);
    }
    mapped_ranges.iter().map(|x| x.0).min().unwrap()
}

#[test]
fn test_process_input() {
    let input = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
    let result = process_input(input, false);
    assert_eq!(35, result);

    let result = process_input(input, true);
    assert_eq!(46, result);
}

fn parse_seeds(text: &str, parse_as_ranges: bool) -> Vec<(isize, isize)> {
    let numbers = text
        .trim()
        .split([' ', '\n'])
        .filter(|&x| x != "")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();
    if parse_as_ranges {
        if (numbers.len() % 2) != 0 {
            panic!("Can't process seeds, odd number {}: {}", numbers.len(), text);
        }
        let mut ranges = vec![];
        for chunk in numbers.chunks(2) {
            ranges.push((chunk[0], chunk[1]));
        }
        ranges
    } else {
        numbers
            .iter()
            .map(|&x| (x, 1))
            .collect()
    }
}

#[test]
fn test_parse_seeds() {
    assert_eq!([(79, 1), (14, 1), (55, 1), (13, 1)], parse_seeds(" 79 14 55 13 ", false)[..]);
    let text = "
1  2
30 40
";
    assert_eq!([(1, 1), (2, 1), (30, 1), (40, 1)], parse_seeds(text, false)[..]);
    assert_eq!([(5, 5)], parse_seeds(" 5 5 ", true)[..]);
}

fn parse_map(text: &str) -> Vec<(isize, isize, isize)> {
    text
        .trim()
        .lines()
        .map(parse_map_line)
        .collect()
}

#[test]
fn test_parse_map() {
    let text = "
0 15 37
37 52 2
39 0 15
";
    let expected = [
        (0, 15, 37),
        (37, 52, 2),
        (39, 0, 15),
    ];
    assert_eq!(expected, parse_map(text)[..]);
}

fn parse_map_line(line: &str) -> (isize, isize, isize) {
    let vec = line
        .trim()
        .split(' ')
        .filter(|&x| x != "")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();
    if vec.len() != 3 {
        panic!("Unexpected map line length {} {}", vec.len(), line);
    }
    (vec[0], vec[1], vec[2])
}

#[test]
fn test_parse_map_line() {
    assert_eq!((0, 15, 37), parse_map_line("0 15 37"));
    assert_eq!((37, 52, 2), parse_map_line("37 52 2"));
}

fn map_ranges(ranges: &[(isize, isize)], maps: &[(isize, isize, isize)]) -> Vec<(isize, isize)> {
    let mut result_ranges = vec![];
    let mut this_iteration = vec![];
    let mut next_iteration = ranges.to_owned();
    for &map in maps {
        (this_iteration, next_iteration) = (next_iteration, this_iteration);
        for &range in this_iteration.iter() {
            let range_end = range.0 + range.1 - 1;
            let map_range_end = map.1 + map.2 - 1;
            let size_before_map = map.1 - range.0;
            let intersection_start = cmp::max(range.0, map.1);
            let intersection_end = cmp::min(range_end, map_range_end);
            let intersection_size = intersection_end - intersection_start + 1;
            if intersection_size > 0 {
                result_ranges.push((map.0 + cmp::max(range.0 - map.1, 0), intersection_size));
                if size_before_map > 0 {
                    next_iteration.push((range.0, size_before_map));
                }
                let size_after_map = range_end - map_range_end;
                if size_after_map > 0 {
                    next_iteration.push((intersection_end + 1, size_after_map));
                }
            } else {
                next_iteration.push(range);
            }
        }
        this_iteration.truncate(0);
    }
    result_ranges.append(&mut next_iteration);
    result_ranges
}

#[test]
fn test_map_ranges() {
    assert_eq!([(5, 5)], map_ranges(&[(5, 5)], &[(20, 10, 5)])[..]);
    assert_eq!([(20, 1), (5, 4)], map_ranges(&[(5, 5)], &[(20, 9, 5)])[..]);
    assert_eq!([(20, 2), (5, 3)], map_ranges(&[(5, 5)], &[(20, 8, 5)])[..]);
    assert_eq!([(20, 3), (5, 2)], map_ranges(&[(5, 5)], &[(20, 7, 5)])[..]);
    assert_eq!([(20, 4), (5, 1)], map_ranges(&[(5, 5)], &[(20, 6, 5)])[..]);
    assert_eq!([(20, 5)], map_ranges(&[(5, 5)], &[(20, 5, 5)])[..]);
    assert_eq!([(21, 4), (9, 1)], map_ranges(&[(5, 5)], &[(20, 4, 5)])[..]);
    assert_eq!([(22, 3), (8, 2)], map_ranges(&[(5, 5)], &[(20, 3, 5)])[..]);
    assert_eq!([(23, 2), (7, 3)], map_ranges(&[(5, 5)], &[(20, 2, 5)])[..]);
    assert_eq!([(24, 1), (6, 4)], map_ranges(&[(5, 5)], &[(20, 1, 5)])[..]);
    assert_eq!([(5, 5)], map_ranges(&[(5, 5)], &[(20, 0, 5)])[..]);
    assert_eq!([(22, 5)], map_ranges(&[(5, 5)], &[(20, 3, 10)])[..]);
    assert_eq!([(20, 5), (3, 2), (10, 3)], map_ranges(&[(3, 10)], &[(20, 5, 5)])[..]);

    let values = [(79, 1), (14, 1), (55, 1), (13, 1)];
    let map = [
        (50, 98, 2),
        (52, 50, 48),
    ];
    let expected = [(81, 1), (57, 1), (14, 1), (13, 1)];
    assert_eq!(expected, map_ranges(&values, &map)[..]);
}
