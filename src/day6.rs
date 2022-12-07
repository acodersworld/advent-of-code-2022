use crate::common::read_file;

fn find_last_dup_pair_idx(line: &[u8]) -> Option<usize> {
    let mut seen: u32 = 0;

    for i in (0..line.len()).rev() {
        let mask = 1 << line[i] - b'a';
        if (seen & mask) != 0 {
            return Some(i);
        }
        seen |= mask;
    }

    None
}

fn find_first_marker(line: &str, marker_len: usize) -> Result<usize, Box<dyn std::error::Error>> {
    if line.len() < marker_len {
        return Err(Box::from("Not enough characters"))
    }

    let line = line.as_bytes();
    let mut count = marker_len;
    let mut iter = 0..line.len() - marker_len;
    while let Some(idx) = iter.next() {
        match find_last_dup_pair_idx(&line[idx..idx + marker_len]) {
            None => return Ok(count),
            Some(d) => {
                if d > 0 { iter.nth(d - 1); } // next will be called in while loop so -1
                count += d + 1; // d is 0-index so + 1
            }
        };
    }

    Err(Box::from("Market not found"))
}

pub fn run_part1() -> Result<(), Box<dyn std::error::Error>> {
    let line = read_file("day6.txt")?;
    println!("Marker at: {}", find_first_marker(&line[0], 4)?);

    Ok(())
}

pub fn run_part2() -> Result<(), Box<dyn std::error::Error>> {
    let line = read_file("day6.txt")?;
    println!("Marker at: {}", find_first_marker(&line[0], 14)?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_duplicates() {
        assert_eq!(find_last_dup_pair_idx(b"mjqj").unwrap(), 1);
        assert_eq!(find_last_dup_pair_idx(b"mqjj").unwrap(), 2);
        assert_eq!(find_last_dup_pair_idx(b"jaqj").unwrap(), 0);

        assert_eq!(find_last_dup_pair_idx(b"mjqa"), None);
        assert_eq!(find_last_dup_pair_idx(b"abc"), None);
    }

    #[test]
    fn test_find_first_marker() {
        assert_eq!(find_first_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4).unwrap(), 7);
        assert_eq!(find_first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4).unwrap(), 5);
        assert_eq!(find_first_marker("nppdvjthqldpwncqszvftbrmjlhg", 4).unwrap(), 6);
        assert_eq!(find_first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4).unwrap(), 10);
        assert_eq!(find_first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4).unwrap(), 11);
    }

    #[test]
    fn test_find_first_marker2() {
        assert_eq!(find_first_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).unwrap(), 19);
        assert_eq!(find_first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14).unwrap(), 23);
        assert_eq!(find_first_marker("nppdvjthqldpwncqszvftbrmjlhg", 14).unwrap(), 23);
        assert_eq!(find_first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14).unwrap(), 29);
        assert_eq!(find_first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14).unwrap(), 26);
    }
}
