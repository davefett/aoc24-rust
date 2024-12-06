use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() -> (i32, i32) {
    let lines = parse_input();
    let mut count1 = 0;
    let mut count2 = 0;

    for line in lines {
        if check_line(&line) {
            count1 += 1;
            count2 += 1;
        } else if check_permutations(&line) {
            count2 += 1;
        }
    }

    (count1, count2)
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

fn check_line(line: &Vec<i32>) -> bool {
    let mut line = line.clone();
    if line[0] > line[1] {
        line.reverse()
    }
    for i in 1..line.len() {
        let diff = line[i] - line[i - 1];
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}

fn check_permutations(line: &Vec<i32>) -> bool {
    for i in 0..line.len() {
        let mut line = line.clone();
        line.remove(i);

        if check_line(&line) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::day02::{check_line, check_permutations, parse_input};

    #[test]
    fn test_parse_input() {
        let input = parse_input();
        assert_eq!(input.len(), 1000);
    }

    #[test]
    fn test_parsing_input_lines() {
        let input = parse_input();
        assert_eq!(input[0], [65, 68, 71, 72, 71]);
        assert_eq!(input[999], [37, 34, 31, 29, 27, 25, 22, 19]);
    }

    #[test]
    fn check_identical_numbers() {
        assert_eq!(check_line(&vec![1, 1, 1, 1]), false);
    }

    #[test]
    fn check_increasing_numbers() {
        assert_eq!(check_line(&vec![1, 2, 3, 4, 5]), true);
    }

    #[test]
    fn check_large_gap_in_numbers() {
        assert_eq!(check_line(&vec![1, 2, 3, 4, 8]), false);
        assert_eq!(check_line(&vec![1, 2, 6, 7, 8]), false);
        assert_eq!(check_line(&vec![1, 5, 6, 7, 8]), false);
    }

    #[test]
    fn check_example1() {
        assert_eq!(check_line(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(check_line(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(check_line(&vec![9, 7, 6, 2, 1]), false);
        assert_eq!(check_line(&vec![1, 3, 2, 4, 5]), false);
        assert_eq!(check_line(&vec![8, 6, 4, 4, 1]), false);
        assert_eq!(check_line(&vec![1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn check_example2() {
        assert_eq!(check_permutations(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(check_permutations(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(check_permutations(&vec![9, 7, 6, 2, 1]), false);
        assert_eq!(check_permutations(&vec![1, 3, 2, 4, 5]), true);
        assert_eq!(check_permutations(&vec![8, 6, 4, 4, 1]), true);
        assert_eq!(check_permutations(&vec![1, 3, 6, 7, 9]), true);
    }
}
