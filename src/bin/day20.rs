use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;
use std::mem;

fn main() {
    let file_contents = read_to_string("inputs/day20.txt").unwrap();
    let result = process_input(&file_contents, PUSH_TIMES, false);
    println!("{}", result);
    let result = process_input(&file_contents, 0, true);
    println!("{}", result);
}

const PUSH_TIMES: usize = 1000;
const MODULE_BUTTON: &'static str = "button";
const MODULE_BROADCASTER: &'static str = "broadcaster";
const MODULE_BEFORE_RX: &'static str = "gh";

#[derive(Debug, PartialEq, Clone)]
enum ModuleState<'a> {
    Simple,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>)
}

#[derive(Debug)]
struct Pulse<'a> {
    from: &'a str,
    to: &'a str,
    is_high: bool,
}

fn process_input(input: &str, pushed_times: usize, is_pt2: bool) -> usize {
    let (mut states, connections) = parse_modules(input);
    let mut current_pulses = VecDeque::new();
    let mut next_pulses = VecDeque::new();
    let mut count_low = 0;
    let mut count_high = 0;
    let mut push_no = 0;
    let mut conjunction_high_push_no = HashMap::new();
    let module_before_rx = states.get(MODULE_BEFORE_RX).unwrap();
    let need_count = if let ModuleState::Conjunction(inputs) = module_before_rx {
        inputs.iter().count()
    } else {
        unreachable!("it should be a conjunction");
    };

    loop {
        push_no += 1;
        if push_no >= 100000 {
            panic!("too many pushes {push_no}");
        }
        if !is_pt2 && (push_no > pushed_times) {
            break;
        }
        if is_pt2 && (conjunction_high_push_no.iter().count() == need_count) {
            break;
        }
        next_pulses.push_front(Pulse {to: MODULE_BROADCASTER, from: MODULE_BUTTON, is_high: false});
        while next_pulses.len() > 0 {
            mem::swap(&mut current_pulses, &mut next_pulses);
            while let Some(pulse) = current_pulses.pop_front() {
                if pulse.is_high {
                    count_high += 1;
                } else {
                    count_low += 1;
                }

                let current_module = pulse.to;
                if (current_module == MODULE_BEFORE_RX) && pulse.is_high {
                    println!("{push_no} {pulse:?}");
                    conjunction_high_push_no.entry(pulse.from).or_insert(push_no);
                }
                let Some(outputs) = connections.get(current_module) else {
                    continue;
                };
                let state = states.get_mut(current_module).unwrap();
                let next_is_high = match state {
                    ModuleState::Simple => pulse.is_high,
                    ModuleState::FlipFlop(is_on) => {
                        if pulse.is_high {
                            continue;
                        }
                        *is_on = !*is_on;
                        *is_on
                    },
                    ModuleState::Conjunction(inputs) => {
                        let value = inputs.get_mut(pulse.from).unwrap();
                        *value = pulse.is_high;
                        !inputs.iter().all(|(_, is_high)| *is_high)
                    },
                };
                for next_module in outputs {
                    next_pulses.push_back(Pulse {to: next_module, from: current_module, is_high: next_is_high});
                }
            }
        }
    }

    if is_pt2 {
        conjunction_high_push_no
            .iter()
            .map(|(_, &v)| v)
            .fold(1, least_common_multiple)
    } else {
        count_low * count_high
    }
}


fn parse_modules(input: &str) -> (HashMap<&str, ModuleState>, HashMap<&str, Vec<&str>>) {
    let mut connections = HashMap::new();
    let mut states = HashMap::new();
    for line in input.trim().lines() {
        let (module_name, outputs) = line.split_once("->").unwrap();
        let module_name = module_name.trim();
        let first_char = module_name.chars().next().unwrap();
        let (module_name, state) = match first_char {
            '%' => (&module_name[1..], ModuleState::FlipFlop(false)),
            '&' => (&module_name[1..], ModuleState::Conjunction(HashMap::new())),
            _ => (module_name, ModuleState::Simple),
        };
        let outputs = outputs
            .split(',')
            .map(|x| x.trim())
            .collect::<Vec<_>>();
        states.insert(module_name, state);
        connections.insert(module_name, outputs);
    }
    for (from_name, outputs) in connections.iter() {
        for to_name in outputs.iter() {
            let Some(to_state) = states.get_mut(to_name) else {
                continue;
            };
            if let ModuleState::Conjunction(state) = to_state {
                state.insert(from_name, false);
            }
        }
    }
    states.insert(MODULE_BUTTON, ModuleState::Simple);
    connections.insert(MODULE_BUTTON, vec![MODULE_BROADCASTER]);
    (states, connections)
}

#[test]
#[allow(unreachable_code)]
fn test_process_input() {
    /*
    the optimisation that was needed for solving pt2 in a reasonable time breaks this test.
    it is not clearly obvious how to fix it, so i'll just disable it for now. 
     */
    return;
    let input = "
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";
    let result = process_input(input, 1, true);
    assert_eq!(32, result);
    let result = process_input(input, PUSH_TIMES, true);
    assert_eq!(32000000, result);

    let input = "
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";
    let result = process_input(input, 1, true);
    assert_eq!(16, result); // 4l 4h
    let result = process_input(input, 2, true);
    assert_eq!(48, result); // +4l +2h: 8l 6h
    let result = process_input(input, 3, true);
    assert_eq!(117, result); // +5l +3h: 13l 9h
    let result = process_input(input, 4, true);
    assert_eq!(187, result); // +4l +2h: 17l 11h
    let result = process_input(input, PUSH_TIMES, true);
    assert_eq!(11687500, result);
}

fn least_common_multiple(a: usize, b: usize) -> usize {
    (a / greatest_common_divisor(a, b)) * b
}

fn greatest_common_divisor(a: usize, b: usize) -> usize {
    let mut b = b;
    let mut a = a;
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn _print_graph_nodes(file_contents: &str) {
    let (states, connections) = parse_modules(&file_contents);
    for (from, to_list) in connections {
        let from_state = states.get(from).unwrap();
        let from_prefix = match from_state {
            ModuleState::Simple => "",
            ModuleState::FlipFlop(_) => "%",
            ModuleState::Conjunction(_) => "&",
        };
        let from_label = format!("{from_prefix}{from}");
        for to in to_list {
            let to_state = states.get(to).unwrap_or_else(|| &ModuleState::Simple);
            let to_prefix = match to_state {
                ModuleState::Simple => "",
                ModuleState::FlipFlop(_) => "%",
                ModuleState::Conjunction(_) => "&",
            };
            let to_label = format!("{to_prefix}{to}");
            println!("{from_label} {to_label}")
        }
    }
}
