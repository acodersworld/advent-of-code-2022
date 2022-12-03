mod day1;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let env: std::vec::Vec<String> = std::env::args().collect();
    match env[1].as_str() {
        "--day1-part1" => day1::run_part1(),
        "--day1-part2" => day1::run_part2(),
        _ => {
            eprintln!("Unknown part");
            Ok(())
        }
    }
}

