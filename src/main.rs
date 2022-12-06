mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod common;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let env: std::vec::Vec<String> = std::env::args().collect();
    match env[1].as_str() {
        "--day1-part1" => day1::run_part1(),
        "--day1-part2" => day1::run_part2(),
        "--day2-part1" => day2::run_part1(),
        "--day2-part2" => day2::run_part2(),
        "--day3-part1" => day3::run_part1(),
        "--day3-part2" => day3::run_part2(),
        "--day4-part1" => day4::run_part1(),
        "--day4-part2" => day4::run_part2(),
        "--day5-part1" => day5::run_part1(),
        "--day5-part2" => day5::run_part2(),
        "--day6-part1" => day6::run_part1(),
        "--day6-part2" => day6::run_part2(),
        _ => {
            eprintln!("Unknown part");
            Ok(())
        }
    }
}

