use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let file_contents = read_to_string("inputs/day19.txt").unwrap();
    let result = process_input_pt1(&file_contents);
    println!("{}", result);
    let result = process_input_pt2(&file_contents);
    println!("{}", result);
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Step<'a> {
    param: char,
    sign: char,
    value: usize,
    to_label: &'a str,
}

#[derive(Copy, Clone, Debug)]
struct Range {
    from: usize,
    to: usize,
}

#[derive(Copy, Clone, Debug)]
struct Ranges {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

fn process_input_pt1(input: &str) -> usize {
    let (workflows, parts) = input.trim().split_once("\n\n").unwrap();
    let workflows_map = parse_workflows_map(workflows);
    let parts = parts.split('\n').map(parse_part).collect::<Vec<_>>();
    parts
        .iter()
        .filter(|part| is_part_accepted(part, &workflows_map))
        .map(|part| part.x + part.m + part.a + part.s)
        .sum()
}

fn process_input_pt2(input: &str) -> usize {
    let (workflows, _) = input.trim().split_once("\n\n").unwrap();
    let workflows_map = parse_workflows_map(workflows);
    let mut stack = vec![(
        "in",
        Ranges{
            x: Range{from: 1, to: 4000},
            m: Range{from: 1, to: 4000},
            a: Range{from: 1, to: 4000},
            s: Range{from: 1, to: 4000},
        }
    )];
    let mut iterations = 0;
    let mut possible_combinations = 0;
    while let Some((label, mut ranges)) = stack.pop() {
        iterations += 1;
        if iterations >= 100000 {
            panic!("too many iterations {iterations}");
        }
        if label == "A" {
            possible_combinations +=
                (ranges.x.to - ranges.x.from + 1)
                * (ranges.m.to - ranges.m.from + 1)
                * (ranges.a.to - ranges.a.from + 1)
                * (ranges.s.to - ranges.s.from + 1);
            continue;
        }
        if label == "R" {
            continue;
        }
        let steps = workflows_map.get(label).unwrap();
        for step in steps {
            if step.param == ' ' {
                stack.push((step.to_label, ranges));
                break;
            }
            let mut split_ranges = ranges.clone();
            let (range_split, range_continue) = match step.param {
                'x' => (&mut split_ranges.x, &mut ranges.x),
                'm' => (&mut split_ranges.m, &mut ranges.m),
                'a' => (&mut split_ranges.a, &mut ranges.a),
                's' => (&mut split_ranges.s, &mut ranges.s),
                _ => panic!("unexpected param value {} in workflow {label}", step.param),
            };
            let (can_split, can_continue) = match step.sign {
                '>' => {
                    let can_split = range_split.to > step.value;
                    let can_continue = range_continue.from <= step.value;
                    range_split.from = step.value + 1;
                    range_continue.to = step.value;
                    (can_split, can_continue)
                },
                '<' => {
                    let can_split = range_split.from < step.value;
                    let can_continue = range_continue.to >= step.value;
                    range_split.to = step.value - 1;
                    range_continue.from = step.value;
                    (can_split, can_continue)
                },
                _ => panic!("unexpected sign value {} in workflow {label}", step.sign),
            };
            if can_split {
                stack.push((step.to_label, split_ranges));
            }
            if !can_continue {
                break;
            }
        }
    }
    possible_combinations
}

fn parse_workflows_map(input: &str) -> HashMap<&str, Vec<Step>> {
    let mut workflows_map = HashMap::new();
    for workflow in input.split('\n') {
        let (title, rest) = workflow.split_once('{').unwrap();
        let steps = rest[..rest.len() - 1].split(',').map(parse_step).collect::<Vec<_>>();
        workflows_map.insert(title, steps);
    }
    workflows_map
}

#[test]
fn test_process_input() {
    let input = "
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";
    let result = process_input_pt1(input);
    assert_eq!(19114, result);

    let result = process_input_pt2(input);
    assert_eq!(167409079868000, result);
}

fn parse_step(line: &str) -> Step {
    if let Some((condition, to_label)) = line.split_once(':') {
        let mut chars = condition.chars();
        let param = chars.next().unwrap();
        let sign = chars.next().unwrap();
        let value = condition[2..].parse().unwrap();
        Step{param, sign, value, to_label}
    } else {
        Step{param: ' ', sign: ' ', value: 0, to_label: line}
    }
}

#[test]
fn test_parse_step() {
    assert_eq!(Step{param: 'm', sign: '<', value: 1801, to_label: "hdj"}, parse_step("m<1801:hdj"));
    assert_eq!(Step{param: ' ', sign: ' ', value: 0, to_label: "rfg"}, parse_step("rfg"));
}

fn parse_part(line: &str) -> Part {
    let line = &line[1..(line.len() - 1)];
    let (mut x, mut m, mut a, mut s) = (0, 0, 0, 0);
    let parts = line.split(',').map(parse_value);
    for (param, value) in parts {
        match param {
            'x' => x = value,
            'm' => m = value,
            'a' => a = value,
            's' => s = value,
            _ => panic!("unexpected param {param}"),
        }
    }
    Part{x, m, a, s}
}

fn parse_value(line: &str) -> (char, usize) {
    let (param, value) = line.split_once('=').unwrap();
    let param = param.chars().next().unwrap();
    let value = value.parse().unwrap();
    (param, value)
}

#[test]
fn test_parse_part() {
    assert_eq!(Part{x: 787, m: 2655, a: 1222, s: 2876}, parse_part("{x=787,m=2655,a=1222,s=2876}"));
}

fn is_part_accepted(part: &Part, workflows: &HashMap<&str, Vec<Step>>) -> bool {
    let mut current_label = "in";
    let mut iterations = 0;
    loop {
        iterations += 1;
        if iterations >= 10000 {
            panic!("too many iterations {iterations}");
        }
        if current_label == "A" {
            return true;
        }
        if current_label == "R" {
            return false;
        }
        let steps = workflows.get(current_label).unwrap();
        for step in steps {
            if step.param == ' ' {
                current_label = step.to_label;
                break;
            }
            let part_value = match step.param {
                'x' => part.x,
                'm' => part.m,
                'a' => part.a,
                's' => part.s,
                _ => panic!("bad param value {} in workflow {current_label}", step.param),
            };
            let is_matching = match step.sign {
                '<' => part_value < step.value,
                '>' => part_value > step.value,
                _ => panic!("bad sign value {} in workflow {current_label}", step.sign),
            };
            if is_matching {
                current_label = step.to_label;
                break;
            }
        }
    }
}

#[test]
fn test_is_accepted() {
    let workflows = "
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}
";
    let workflows = workflows.trim();
    let workflows_map = parse_workflows_map(workflows);

    let part = parse_part("{x=787,m=2655,a=1222,s=2876}");
    assert_eq!(true, is_part_accepted(&part, &workflows_map));
    let part = parse_part("{x=1679,m=44,a=2067,s=496}");
    assert_eq!(false, is_part_accepted(&part, &workflows_map));
    let part = parse_part("{x=2036,m=264,a=79,s=2244}");
    assert_eq!(true, is_part_accepted(&part, &workflows_map));
    let part = parse_part("{x=2461,m=1339,a=466,s=291}");
    assert_eq!(false, is_part_accepted(&part, &workflows_map));
    let part = parse_part("{x=2127,m=1623,a=2188,s=1013}");
    assert_eq!(true, is_part_accepted(&part, &workflows_map));
}
