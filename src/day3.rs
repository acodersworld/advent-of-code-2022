use std::fs;

pub fn run_part1() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("day3.txt")?;
    let compartments = parse_input(&contents)?;
    let score: u32 = compartments
    .iter()
    .map(|c| {
        let common = find_common(c.0, c.1);
        score_char(common)
    })
    .sum();

    println!("Score: {}", score);

    Ok(())
}

pub fn run_part2() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("day3.txt")?;
    let rucksacks = parse_input3(&contents);

    let total: u32 = rucksacks
        .iter()
        .map(|x| {
            let c = find_common3(&x.0, &x.1, &&x.2);
            score_char(c)
        })
        .sum();

    println!("Sum: {}", total);
    Ok(())
}

fn find_common3(a: &str, b: &str, c: &str) -> char {
    let mut a: std::vec::Vec<char> = a.chars().collect();
    let mut b: std::vec::Vec<char> = b.chars().collect();
    let mut c: std::vec::Vec<char> = c.chars().collect();

    a.sort();
    a.dedup();
    b.sort();
    b.dedup();
    c.sort();
    c.dedup();

    a.append(&mut b);
    a.append(&mut c);
    a.sort();

    let len = a.len();
    for i in 2..len {
        if a[i] == a[i-2] && a[i] == a[i-1] {
            return a[i]
        }
    }

    panic!("No dups");
}

fn parse_input(input: &str) -> Result<std::vec::Vec<(&str, &str)>, Box<dyn std::error::Error>> {
    let rucksacks = input.split("\n");

    let compartments = rucksacks
    .filter(|line| !line.is_empty())
    .map(|line| {
        if (line.len() % 2) != 0 {
            panic!("Input length is not even")
        }

        let mid = line.len() / 2;
        (&line[..mid], &line[mid..])
    }).collect::<>();

    Ok(compartments)
}

fn parse_input3(input: &str) -> std::vec::Vec<(String, String, String)> {
    let rucksacks: std::vec::Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();

    let iter = rucksacks.chunks_exact(3);
    if iter.remainder().len() != 0 {
        panic!("Did not expect any remainder");
    }

    iter.map(|x| (x[0].to_string(), x[1].to_string(), x[2].to_string())).collect()
}

fn find_common(a: &str, b: &str) -> char {
    let res: std::vec::Vec<char> = a.chars().filter(|c| {
        match b.find(*c) {
            None => false,
            Some(_) => true
        }
    }).collect();

    if res.len() == 0 {
        panic!("Expected none empty")
    }

    return res[0]
}

fn score_char(c: char) -> u32 {
    if 'a' <= c && c <= 'z' {
        return c as u32 - 'a' as u32 + 1
    }
    else if 'A' <= c && c <= 'Z' {
        return c as u32 - 'A' as u32 + 27
    }

    panic!("Unexpected character {}", c);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input =
r#"CjhshBJCSrTTsLwqwqwb
GtmnFHlDfcpHbLZjtTTRLWwb
fDfNHHjVFNvvrvVBJJdS"#;
        let contents = parse_input(input).unwrap();
        assert_eq!(contents.len(), 3);
        assert_eq!(contents[0].0, "CjhshBJCSr");
        assert_eq!(contents[0].1, "TTsLwqwqwb");

        assert_eq!(contents[1].0, "GtmnFHlDfcpH");
        assert_eq!(contents[1].1, "bLZjtTTRLWwb");

        assert_eq!(contents[2].0, "fDfNHHjVFN");
        assert_eq!(contents[2].1, "vvrvVBJJdS");
    }

    #[test]
    fn test_find_common() {
        assert_eq!(find_common("CjhshBJCSr", "TTsLwqwqwb"), 's');
        assert_eq!(find_common("GtmnFHlDfcpH", "bLZjtTTRLWwb"), 't');
        assert_eq!(find_common("fDfNHHjVFN", "vvrvVBJJdS"), 'V');
    }

    #[test]
    fn test_score_char() {
        assert_eq!(score_char('a'), 1);
        assert_eq!(score_char('z'), 26);
        assert_eq!(score_char('A'), 27);
        assert_eq!(score_char('Z'), 52);
    }

    #[test]
    fn test_parse_input3() {
        let input =
r#"CjhshBJCSrTTsLwqwqwb
GtmnFHlDfcpHbLZjtTTRLWwb
fDfNHHjVFNvvrvVBJJdS
PPWvWQjPhrPQwlMWJJdMDGbJTdCJ
rsqsStgNNggBNBZHSrJGdJdCFRRZCFbGbTdJ
qgBqqHzzggBpzSnBNqNSSSgcfhrVlVmwPljQVLVwVvQmmzVl
bBBGBfmGvBTnGtGJBtGpcJbZrrddjqrZhDldwdcqrjrjDr
HWPSQMsPHFsMWPVVMVSHCwDCDwwZZvwjwQZZwjdd
vVHPgHHFRLfpfJTLLtJL"#;

        let r = parse_input3(input);
        assert_eq!(r.len(), 3);
        assert_eq!(r[0], ("CjhshBJCSrTTsLwqwqwb".to_owned(), "GtmnFHlDfcpHbLZjtTTRLWwb".to_owned(), "fDfNHHjVFNvvrvVBJJdS".to_owned()));
        assert_eq!(r[1], ("PPWvWQjPhrPQwlMWJJdMDGbJTdCJ".to_owned(), "rsqsStgNNggBNBZHSrJGdJdCFRRZCFbGbTdJ".to_owned(), "qgBqqHzzggBpzSnBNqNSSSgcfhrVlVmwPljQVLVwVvQmmzVl".to_owned()));
        assert_eq!(r[2], ("bBBGBfmGvBTnGtGJBtGpcJbZrrddjqrZhDldwdcqrjrjDr".to_owned(), "HWPSQMsPHFsMWPVVMVSHCwDCDwwZZvwjwQZZwjdd".to_owned(), "vVHPgHHFRLfpfJTLLtJL".to_owned()));
    }

    #[test]
    fn test_find_common3() {
        assert_eq!(find_common3("abcdef", "ghijfkl", "mnopfqrd"), 'f');
    }
}
