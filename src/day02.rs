use crate::day02::Direction::{Down, Up};
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Direction {
    Up,
    Down,
}

fn parse_input() -> Vec<Vec<i32>> {
    let file = File::open("./input/input02.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let values: Vec<i32> = line
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        lines.push(values);
    }

    lines
}

pub fn run() -> (i32, i32) {
    let lines = parse_input();
    let mut count1 = 0;

    for line in &lines {
        if line[0] == line[1] {
            continue;
        }
        let direction = if line[0] > line[1] { Down } else { Up };

        for i in 1..line.len() {
            let diff = line[i] - line[i - 1];
            match direction {
                Up => {
                    if diff < 1 || diff > 3 {
                        break;
                    }
                }
                Down => {
                    if diff > -1 || diff < -3 {
                        break;
                    }
                }
            }
            if i == line.len() - 1 {
                count1 += 1;
            }
        }
    }

    let mut count2 = 0;

    (count1, 0)
}
