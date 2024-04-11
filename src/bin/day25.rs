use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let file_contents = read_to_string("inputs/day25.txt").unwrap();
    let result = process_input(&file_contents);
    println!("{}", result);
}

fn process_input(input: &str) -> usize {
    // todo: finish the day25
    let edges = parse_edges(input);
    for (from, to_list) in edges {
        for to in to_list {
            println!("{from} {to}");
        }
    }
    1
}

fn parse_edges(input: &str) -> HashMap<&str, Vec<&str>> {
    let lines = input.trim().lines();
    let mut edges = HashMap::new();
    for line in lines {
        let (from, to_list) = line.split_once(':').unwrap();
        for to in to_list.trim().split(' ') {
            edges.entry(from).or_insert(vec![]).push(to);
            edges.entry(to).or_insert(vec![]).push(from);
        }
    }
    edges
}

#[test]
#[allow(unreachable_code)]
fn test_process_input() {
    // todo: test it when it's implemented
    return;
    let input = "
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
";
    let result = process_input(input);
    assert_eq!(54, result);
}
