mod day01;
mod day02;
mod day03;

fn main() {
    let (day1_1, day1_2) = day01::run();
    println!("problem 1.1: {}\nproblem 1.2: {}", day1_1, day1_2);
    let (day2_1, day2_2) = day02::run();
    println!("problem 2.1: {}\nproblem 2.2: {}", day2_1, day2_2);
    let (day3_1, day3_2) = day03::run();
    println!("problem 3.1: {}\nproblem 3.2: {}", day3_1, day3_2);
}
