use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() -> (i32, i32) {
    let line = parse_input();

    let result1: i32 = extract_muls(&line);
    let result2: i32 = extract_muls(&extract_dos_and_donts(&line));

    (result1, result2)
}

fn parse_input() -> String {
    let file = File::open("./input/input03.txt").unwrap();
    let reader = BufReader::new(file);
    let mut output: String = "".to_string();

    for line in reader.lines() {
        output.push_str(&line.unwrap());
    }

    output
}

fn extract_muls(input: &str) -> i32 {
    let re = Regex::new(r"mul[(](\d+),(\d+)[)]").unwrap();
    let mut result = 0;
    for (_, [val1, val2]) in re.captures_iter(&input).map(|c| c.extract()) {
        let val1: i32 = val1.parse().unwrap();
        let val2: i32 = val2.parse().unwrap();
        result += val1 * val2;
    }
    result
}

fn extract_dos_and_donts(input: &str) -> String {
    let re = Regex::new(r"don't[(][)](.*?)do[(][)]").unwrap();
    let output = re.replace_all(input, "").to_string();
    output
}

#[cfg(test)]
mod tests {
    use crate::day03::{extract_dos_and_donts, extract_muls, parse_input};

    #[test]
    fn example3_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(extract_muls(input), 161);
    }

    #[test]
    fn example3_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(extract_muls(&extract_dos_and_donts(input)), 48);
    }

    #[test]
    fn parse_input_returns_correct_number_of_characters() {
        let input = parse_input();
        assert_eq!(input.len(), 19713);
    }

    #[test]
    fn extract_muls_functions() {
        let input = "amul(1,2)bmul(3,4)c";
        assert_eq!(extract_muls(input), 14);
    }

    #[test]
    fn dos_and_donts() {
        let result = extract_dos_and_donts(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        assert_eq!(result, "xmul(2,4)&mul[3,7]!^?mul(8,5))");
    }
}
