use knapsack::solve;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn parse_file(path: String) -> std::io::Result<(usize, usize, Vec<usize>)> {
    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;
    let lines: Vec<_> = content.lines().collect();

    let line1 = lines[0];
    let line1_split: Vec<_> = line1.split_ascii_whitespace().collect();
    let max = line1_split[0].parse::<usize>().unwrap();
    let n = line1_split[1].parse::<usize>().unwrap();

    let line2 = lines[1];
    let mut items: Vec<usize> = vec![];
    for i in line2.split_ascii_whitespace().collect::<Vec<&str>>() {
        items.push(i.parse::<usize>().unwrap());
    }
    return Ok((max, n, items));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).unwrap();
    let (max, _, items) = parse_file(path.to_string()).unwrap();
    let items = items.iter().map(|i| (*i, *i)).collect::<Vec<_>>();
    let (v, w, r) = solve(&items, max);
    println!("{} {}\n{:?}", v, w, r);
}
