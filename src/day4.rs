use crate::common::read_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pair {
    min: u32,
    max: u32
}

impl Pair {
    fn new(min: u32, max: u32) -> Pair {
        Pair {
            min,
            max
        }
    }

    fn fully_contains(&self, other: &Pair) -> bool {
        self.min <= other.min && other.max <= self.max
    }

    fn does_overlap(&self, other: &Pair) -> bool {
        other.min <= self.max && self.min <= other.max
    }
}

pub fn run_part1() -> Result<(), Box<dyn std::error::Error>> {
    let lines = read_file("day4.txt")?;
    let contain_pair_count: u32 = lines
        .iter()
        .map(|l| {
            let (lhs, rhs) = parse_line(l);
            if lhs.fully_contains(&rhs) || rhs.fully_contains(&lhs) {
                1
            }
            else {
                0
            }
        })
        .sum();

    println!("Pair sum: {}", contain_pair_count);
    Ok(())
}

pub fn run_part2() -> Result<(), Box<dyn std::error::Error>> {
    let lines = read_file("day4.txt")?;
    let contain_pair_count: u32 = lines
        .iter()
        .map(|l| {
            let (lhs, rhs) = parse_line(l);
            if lhs.does_overlap(&rhs) {
                1
            }
            else {
                0
            }
        })
        .sum();

    println!("Pair sum: {}", contain_pair_count);
    Ok(())
}

fn parse_pair(line: &str) -> Pair {
    match line.split_once('-') {
        None => panic!("Failed to split pair: {}", line),
        Some((lhs, rhs)) => Pair::new(lhs.parse().unwrap(), rhs.parse().unwrap())
    }
}

fn parse_line(line: &str) -> (Pair, Pair) {
    match line.split_once(',') {
        None => panic!("Failed to split line {}", line),
        Some((lhs, rhs)) => (parse_pair(lhs), parse_pair(rhs))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair("13-53"), Pair::new(13, 53));
        assert_eq!(parse_pair("17-82"), Pair::new(17, 82));
        assert_eq!(parse_pair("32-32"), Pair::new(32, 32));
        assert_eq!(parse_pair("32-42"), Pair::new(32, 42));
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("60-71,59-70"), (Pair::new(60, 71), Pair::new(59, 70)));
        assert_eq!(parse_line("91-92,4-90"), (Pair::new(91, 92), Pair::new(4, 90)));
        assert_eq!(parse_line("90-90,1-90"), (Pair::new(90, 90), Pair::new(1, 90)));
    }

    #[test]
    fn test_fully_contains() {
        assert_eq!(Pair::new(32, 42).fully_contains(&Pair::new(32, 32)), true);
        assert_eq!(Pair::new(60, 71).fully_contains(&Pair::new(59, 70)), false);
    }

    #[test]
    fn test_does_overlap() {
        assert_eq!(Pair::new(2,4).does_overlap(&Pair::new(6,8)), false);
        assert_eq!(Pair::new(2,3).does_overlap(&Pair::new(4,5)), false);

        assert_eq!(Pair::new(5,7).does_overlap(&Pair::new(7,9)), true);
        assert_eq!(Pair::new(2,8).does_overlap(&Pair::new(3,7)), true);
        assert_eq!(Pair::new(6,6).does_overlap(&Pair::new(4,6)), true);
        assert_eq!(Pair::new(2,6).does_overlap(&Pair::new(4,8)), true);

        assert_eq!(Pair::new(7,9).does_overlap(&Pair::new(5,7)), true);
        assert_eq!(Pair::new(3,7).does_overlap(&Pair::new(2,8)), true);
        assert_eq!(Pair::new(4,6).does_overlap(&Pair::new(6,6)), true);
        assert_eq!(Pair::new(4,8).does_overlap(&Pair::new(2,6)), true);
    }
}
