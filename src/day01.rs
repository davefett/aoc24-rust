use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_input() -> (Vec<i32>, Vec<i32>) {
    let file = File::open("./input/input01.txt").unwrap();
    let reader = BufReader::new(file);

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line1 = line.unwrap();

        let line_items = line1.split_whitespace().collect::<Vec<_>>();

        list1.push(line_items[0].to_string().parse::<i32>().unwrap());
        list2.push(line_items[1].to_string().parse::<i32>().unwrap());
    }

    (list1, list2)
}

pub fn run() -> (i32, i32) {
    let (mut list1, mut list2) = load_input();
    list1.sort();
    list2.sort();

    let mut sum1 = 0;

    for (i, val) in list1.iter().enumerate() {
        let diff = val - list2[i];
        sum1 += diff.abs();
    }

    let mut sum2 = 0;
    for val in list1.iter() {
        let cnt = list2.iter().filter(|&x| *x == *val).count() as i32;
        sum2 += val * cnt;
    }

    (sum1, sum2)
}
