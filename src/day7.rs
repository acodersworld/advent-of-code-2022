use crate::common::read_file;

struct Dir {
    name: String,
    file_size: usize,
    total_size: usize,

    dirs: std::vec::Vec<Dir>
}

impl Dir {
    fn new(name: &str) -> Dir {
        Dir {
            name: name.to_string(),
            file_size: 0,
            total_size: 0,
            dirs: vec![]
        }
    }

    fn add_dir(&mut self, name: &str) {
        if !self.dirs.iter().any(|x| x.name == name) {
            self.dirs.push(Dir::new(name))
        }
    }
}

fn parse(dir: &mut Dir, lines: &[String]) -> usize {
    let mut i = 0;
    let len = lines.len();

    let re = regex::Regex::new(r#"(\d+) (.+)"#).unwrap();
    while i < len {
        let ln = &lines[i];
        match &ln[..4] {
            "$ cd" =>  {
                let dir_name = &ln[5..];
                if dir_name == ".." {
                    return i + 1;
                }

                let mut found = false;
                for d in &mut dir.dirs {
                    if d.name == dir_name {
                        i += parse(d, &lines[i+1..]);
                        found = true;
                        break;
                    }
                }
                assert!(found);
            },
            "$ ls" => {
            },
            "dir " => dir.add_dir(&ln[4..]),
            _ => {
                let cap = match re.captures(ln) {
                    None => panic!("Failed to capture: {}", ln),
                    Some(x) => x
                };
                let size: usize = cap.get(1).unwrap().as_str().parse().unwrap();
                let _filename: String = cap.get(2).unwrap().as_str().to_string();
                dir.file_size += size;
            }
        }

        i += 1;
    }

    i
}

fn sum_totals(d: &mut Dir) -> usize {
    if d.dirs.is_empty() {
        d.total_size = d.file_size;
        return d.file_size
    }

    let mut total = d.file_size;
    for c in &mut d.dirs {
        total += sum_totals(c);
    }

    d.total_size = total;
    total
}

fn total_undersized_dirs(d: &Dir) -> usize {
    let mut total = 0;
    if d.total_size < 100000 {
        total = d.total_size;
    }

    for c in &d.dirs {
        total += total_undersized_dirs(c);
    }

    total
}

fn find_smallest_over_threshold(d: &Dir, threshold: usize) -> Option<usize> {
    if d.total_size < threshold {
        return None
    }

    let mut candidate = d.total_size;
    for c in &d.dirs {
        if let Some(smallest) = find_smallest_over_threshold(c, threshold) {
            if smallest < candidate {
                candidate = smallest;
            }
        }
    }

    Some(candidate)
}

pub fn run_part1() -> Result<(), Box<dyn std::error::Error>> {
    let lines = read_file("day7.txt")?;
    let mut d = Dir::new("/");
    parse(&mut d, &lines[1..]);
    sum_totals(&mut d);
    println!("{}", total_undersized_dirs(&d));
    Ok(())
}

pub fn run_part2() -> Result<(), Box<dyn std::error::Error>> {
    let lines = read_file("day7.txt")?;
    let mut d = Dir::new("/");
    parse(&mut d, &lines[1..]);
    let total_size = sum_totals(&mut d);

    let free_space = 70000000 - total_size;
    let required_free = 30000000 - free_space;
    match find_smallest_over_threshold(&d, required_free) {
        None => println!("Not found"),
        Some(sz) => println!("Dir size: {}", sz)
    };

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_file_size() {
        let contents: std::vec::Vec<String> = vec![
            "$ ls".to_owned(),
            "dir a".to_owned(),
            "100 b.txt".to_owned(),
            "250 c.dat".to_owned()];

        let mut d = Dir::new("/");
        assert_eq!(parse(&mut d, &contents), 4);
        assert_eq!(d.file_size, 350);
    }

    #[test]
    fn test_dir() {
        let contents: std::vec::Vec<String> = vec![
            "$ ls".to_owned(),
            "dir a".to_owned(),
            "dir d".to_owned()];

        let mut d = Dir::new("/");
        assert_eq!(parse(&mut d, &contents), 3);
        assert_eq!(d.dirs.len(), 2);
        assert_eq!(d.dirs[0].name, "a");
        assert_eq!(d.dirs[1].name, "d");
    }

    #[test]
    fn test_cd() {
        let contents: std::vec::Vec<String> = vec![
            "$ ls".to_owned(),
            "dir a".to_owned(),
            "dir b".to_owned(),
            "$ cd a".to_owned(),
            "$ ls".to_owned(),
            "dir a".to_owned(),
            "dir b".to_owned(),
            "dir c".to_owned(),
            "$ cd ..".to_owned(),
            "$ cd b".to_owned(),
            "$ ls".to_owned(),
            "dir x".to_owned(),
            "dir y".to_owned(),
            "dir z".to_owned()];

        let mut d = Dir::new("/");
        assert_eq!(parse(&mut d, &contents), 14);
        assert_eq!(d.dirs.len(), 2);
        assert_eq!(d.dirs[0].name, "a");
        assert_eq!(d.dirs[1].name, "b");

        let a = &d.dirs[0];
        assert_eq!(a.dirs.len(), 3);
        assert_eq!(a.dirs[0].name, "a");
        assert_eq!(a.dirs[1].name, "b");
        assert_eq!(a.dirs[2].name, "c");

        let b = &d.dirs[1];
        assert_eq!(b.dirs[0].name, "x");
        assert_eq!(b.dirs[1].name, "y");
        assert_eq!(b.dirs[2].name, "z");
    }

    #[test]
    fn test_nested_file_size() {
        let contents: std::vec::Vec<String> = vec![
            "$ ls".to_owned(),
            "dir a".to_owned(),
            "dir b".to_owned(),
            "$ 20 file".to_owned(),
            "$ cd a".to_owned(),
            "$ ls".to_owned(),
            "$ 100 file1".to_owned(),
            "$ 200 file2".to_owned(),
            "$ cd ..".to_owned(),
            "$ cd b".to_owned(),
            "$ ls".to_owned(),
            "$ 300 file1".to_owned(),
            "$ 400 file2".to_owned(),
        ];
        let mut d = Dir::new("/");
        assert_eq!(parse(&mut d, &contents), 13);
        assert_eq!(d.dirs.len(), 2);

        let a = &d.dirs[0];
        let b = &d.dirs[1];

        assert_eq!(d.file_size, 20);
        assert_eq!(a.file_size, 300);
        assert_eq!(b.file_size, 700);
    }

    #[test]
    fn test() {
        let content = vec![
            "$ cd /".to_owned(),
            "$ ls".to_owned(),
            "dir a".to_owned(),
            "14848514 b.txt".to_owned(),
            "8504156 c.dat".to_owned(),
            "dir d".to_owned(),
            "$ cd a".to_owned(),
            "$ ls".to_owned(),
            "dir e".to_owned(),
            "29116 f".to_owned(),
            "2557 g".to_owned(),
            "62596 h.lst".to_owned(),
            "$ cd e".to_owned(),
            "$ ls".to_owned(),
            "584 i".to_owned(),
            "$ cd ..".to_owned(),
            "$ cd ..".to_owned(),
            "$ cd d".to_owned(),
            "$ ls".to_owned(),
            "4060174 j".to_owned(),
            "8033020 d.log".to_owned(),
            "5626152 d.ext".to_owned(),
            "7214296 k".to_owned()];

        let mut r = Dir::new("/");
        parse(&mut r, &content[1..]);

        sum_totals(&mut r);

        let a = &r.dirs[0];
        let d = &r.dirs[1];

        let e = &a.dirs[0];
        assert_eq!(e.total_size, 584);
        assert_eq!(a.total_size, 94853);
        assert_eq!(d.total_size, 24933642);
        assert_eq!(r.total_size, 48381165);

        assert_eq!(total_undersized_dirs(&r), 95437);
        assert_eq!(find_smallest_over_threshold(&r, 8381165), Some(24933642));
    }

    #[test]
    fn test_sum_total() {
        let mut d = Dir::new("/");
        d.file_size = 10;

        d.dirs.push({
            let mut a = Dir::new("");
            a.file_size = 20;

            a.dirs.push({
                let mut b = Dir::new("");
                b.file_size = 15;
                b
            });

            a.dirs.push({
                let mut c = Dir::new("");
                c.file_size = 25;
                c
            });

            a
        });

        d.dirs.push({
            let mut a = Dir::new("");
            a.file_size = 30;
            a
        });

        assert_eq!(sum_totals(&mut d), 100);
    }
}

