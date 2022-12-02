use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("day1.txt")?;
    
    let cals = contents.split("\n");
    let mut elves = vec![];
    let mut curr = 0;

    for x in cals {
        if x.is_empty() {
            elves.push(curr);
            curr = 0;
        }
        else {
            curr += x.parse::<i32>()?;
        }
    }

    match elves.iter().enumerate().max_by(|(_, a), (_, b)| a.cmp(b)) {
        None => println!("No MAX"),
        Some((i, x)) => println!("MAX: {}/{}", i, x)
    }
    Ok(())
}
