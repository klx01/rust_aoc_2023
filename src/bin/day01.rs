use std::fs::read_to_string;

fn main() {
    let file_contents = read_to_string("inputs/day01.txt").unwrap();
    let result = process_input(&file_contents);
    println!("{}", result);
}

fn process_input(input: &str) -> usize {
    input.trim().lines().map(line_to_number).sum()
}

#[test]
fn test_process_input() {
    let input = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
    let result = process_input(input);
    assert_eq!(142, result);

    let input = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
    ";
    let result = process_input(input);
    assert_eq!(281, result);
}

fn line_to_number(line: &str) -> usize {
    let lc_line = line.to_ascii_lowercase();
    let chars = lc_line.bytes().collect::<Vec<_>>();
    let mut numbers = vec![];
    let str_numbers: [&[u8]; 10] = [
        b"zero", b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];
    for (index, &char) in chars.iter().enumerate() {
        if char.is_ascii_digit() {
            numbers.push((char as char).to_digit(10).unwrap() as usize);
        } else {
            for (number, &str_number) in str_numbers.iter().enumerate() {
                let str_number_len = str_number.len();
                let slice = chars.get(index..(index + str_number_len));
                if let Some(slice) = slice {
                    if slice == str_number {
                        numbers.push(number);
                        break;
                    }
                }
            }
        }
    }

    let numbers_count = numbers.len();
    match numbers_count {
        0 => 0,
        _ => (numbers[0] * 10) + numbers[numbers_count - 1]
    }
}

#[test]
fn test_line_to_number() {
    assert_eq!(38, line_to_number("pqr3stu8vwx"));
    assert_eq!(15, line_to_number("a1b2c3d4e5f"));
    assert_eq!(77, line_to_number("treb7uchet"));
    assert_eq!(0, line_to_number(""));
    assert_eq!(29, line_to_number("two1nine"));
    assert_eq!(83, line_to_number("eightwothree"));
    assert_eq!(13, line_to_number("abcone2threexyz"));
    assert_eq!(24, line_to_number("xtwone3four"));
    assert_eq!(42, line_to_number("4nineeightseven2"));
    assert_eq!(14, line_to_number("zoneight234"));
    assert_eq!(76, line_to_number("7pqrstsixteen"));
    assert_eq!(21, line_to_number("twone"));
}
