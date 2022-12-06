use crate::common::read_file;

fn find_first_dup(line: &[u8]) -> Option<u8> {
    for i in 0..(line.len()-1) {
        if line[i+1..].contains(&line[i]) {
            return Some(line[i])
        }
    }

    None
}

fn find_first_marker(line: &str, marker_len: usize) -> Result<usize, Box<dyn std::error::Error>> {
    let line = line.as_bytes();
    let mut ring_buffer = vec![0 as u8; marker_len];
    let mut idx = 0;

    let mut iter = line.iter();
    for x in ring_buffer.iter_mut() {
        *x = match iter.next() {
            None => return Err(Box::from("Not enough characters")),
            Some(v) => *v
        };
    }

    let mut count = marker_len;
    let mut last_dup = match find_first_dup(&ring_buffer) {
        None => return Ok(count),
        Some(d) => d
    };

    for x in iter {
        let dropping = ring_buffer[idx];
        ring_buffer[idx] = *x;
        count += 1;

        if dropping == last_dup {
            last_dup = match find_first_dup(&ring_buffer) {
                None => return Ok(count),
                Some(d) => d
            };
        }
        idx = (idx + 1) % marker_len;
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
        assert_eq!(find_first_dup(b"mjqj").unwrap(), b'j');
        assert_eq!(find_first_dup(b"mqjj").unwrap(), b'j');
        assert_eq!(find_first_dup(b"jaqj").unwrap(), b'j');

        assert_eq!(find_first_dup(b"mjqa"), None);
        assert_eq!(find_first_dup(b"abc"), None);
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
