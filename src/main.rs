mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
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
        "--day7-part1" => day7::run_part1(),
        "--day7-part2" => day7::run_part2(),
        "--day8-part1" => day8::run_part1(),
        "--day8-part2" => day8::run_part2(),
        "--day9-part1" => day9::run_part1(),
        "--day9-part2" => day9::run_part2(),
        "--day10-part1" => day10::run_part1(),
        "--day10-part2" => day10::run_part2(),
        "--day11-part1" => day11::run_part1(),
        "--day11-part2" => day11::run_part2(),
        "--day12-part1" => day12::run_part1(),
        "--day12-part2" => day12::run_part2(),
        _ => {
            eprintln!("Unknown part");
            Ok(())
        }
    }
}

