mod day01;
mod day02;

fn main() {
    let (day1_1, day1_2) = day01::run();
    println!("problem 1.1: {}\nproblem 1.2: {}", day1_1, day1_2);
    let (day2_1, day2_2) = day02::run();
    println!("problem 2.1: {}\nproblem 2.2: {}", day2_1, day2_2);
}
