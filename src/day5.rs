use crate::common::read_file2;
use regex::Regex;

struct Move {
    from: usize,
    to: usize,
    count: usize
}

type Stacks = std::vec::Vec<std::vec::Vec<char>>;
type Moves = std::vec::Vec<Move>;

fn parse_input(lines: &std::vec::Vec<String>) -> Result<(Stacks, Moves), Box<dyn std::error::Error>> {
    let stacks_text = match lines.iter().position(|s| s.starts_with(" 1")) {
            Some(i) => &lines[..i],
            None => return Err(Box::from("Invalid input")),
        };

    let stack_count = (stacks_text[0].len() / 4) + 1;
    let mut stacks_output: Stacks = vec![];
    stacks_output.resize_with(stack_count, Default::default);

    for row in stacks_text {
        for i in 0..stack_count {
            let base = i * 4;
            let c = row.as_bytes()[base + 1] as char;
            if c != ' ' {
                stacks_output[i].push(c)
            }
        }
    }

    let mut moves: Moves = vec![];
    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for m in &lines[stacks_text.len()+1..] {
        if m.is_empty() {
            continue;
        }

        let cap = match regex.captures(m) {
            None => return Err(Box::from("Failed to capture moves")),
            Some(c) => c
        };

        let count: usize = cap.get(1).unwrap().as_str().parse()?;
        let from: usize = cap.get(2).unwrap().as_str().parse()?;
        let to: usize = cap.get(3).unwrap().as_str().parse()?;

        moves.push(Move {
            count,
            from: from - 1,
            to: to - 1
        });
    }

    for s in &mut stacks_output {
        s.reverse();
    }

    Ok((stacks_output, moves))
}

fn move_crates(stacks: &mut Stacks, m: &Move, rev_moves: bool) {
    let base_from = {
        let st_from: &mut std::vec::Vec<char> = &mut stacks[m.from];
        st_from.len() - m.count
    };

    {
        let mut cr = stacks[m.from][base_from..].to_vec();
        if rev_moves {
            cr.reverse();
        }

        let st_to = &mut stacks[m.to];
        st_to.append(&mut cr);
    }

    stacks[m.from].truncate(base_from);
}

pub fn print_stacks(stacks: &Stacks) {
    let mx = stacks.iter().map(|s| s.len()).max().unwrap_or(0);

    for i in (0..mx).rev() {
        for s in stacks {
            if i < s.len() {
                print!(" [{}] ", s[i]);
            }
            else {
                print!("     ");
            }
        }
        println!("");
    }

    for (_, s) in stacks.iter().enumerate() {
        print!(" ({}) ", s.len());
    }
    println!("");
    for (i, _) in stacks.iter().enumerate() {
        print!("  {}  ", i);
    }
}

pub fn run_part1() -> Result<(), Box<dyn std::error::Error>> {
    let (mut stacks, moves) = parse_input(&read_file2("day5.txt")?)?;

    for m in moves {
        move_crates(&mut stacks, &m, true);
    }
    print_stacks(&stacks);
    Ok(())
}

pub fn run_part2() -> Result<(), Box<dyn std::error::Error>> {
    let (mut stacks, moves) = parse_input(&read_file2("day5.txt")?)?;

    for m in moves {
        move_crates(&mut stacks, &m, false);
    }
    print_stacks(&stacks);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let contents: std::vec::Vec<String> = vec![
            "                [B]     [L]     [S]".to_owned(),
            "        [Q] [J] [C]     [W]     [F]".to_owned(),
            "    [F] [T] [B] [D]     [P]     [P]".to_owned(),
            "    [S] [J] [Z] [T]     [B] [C] [H]".to_owned(),
            "    [L] [H] [H] [Z] [G] [Z] [G] [R]".to_owned(),
            "[R] [H] [D] [R] [F] [C] [V] [Q] [T]".to_owned(),
            "[C] [J] [M] [G] [P] [H] [N] [J] [D]".to_owned(),
            "[H] [B] [R] [S] [R] [T] [S] [R] [L]".to_owned(),
            " 1   2   3   4   5   6   7   8   9 ".to_owned(),
            "".to_owned(),
            "move 8 from 7 to 1".to_owned(),
            "move 9 from 1 to 9".to_owned(),
            "move 4 from 5 to 4".to_owned(),
            "move 4 from 6 to 1".to_owned()];

        let (stacks, moves) = match parse_input(&contents) {
            Ok(x) => x,
            Err(e) => panic!("{}", e)
        };

        let rev = |mut v: std::vec::Vec<char>| {
            v.reverse();
            v
        };

        assert_eq!(stacks.len(), 9);
        assert_eq!(stacks[0], vec!['H', 'C', 'R']);
        assert_eq!(stacks[1], rev(vec!['F', 'S', 'L', 'H', 'J', 'B']));
        assert_eq!(stacks[2], rev(vec!['Q', 'T', 'J', 'H', 'D', 'M', 'R']));
        assert_eq!(stacks[3], rev(vec!['J', 'B', 'Z', 'H', 'R', 'G', 'S']));
        assert_eq!(stacks[4], rev(vec!['B', 'C', 'D', 'T', 'Z', 'F', 'P', 'R']));
        assert_eq!(stacks[5], rev(vec!['G', 'C', 'H', 'T']));
        assert_eq!(stacks[6], rev(vec!['L', 'W', 'P', 'B', 'Z', 'V', 'N', 'S']));
        assert_eq!(stacks[7], rev(vec!['C', 'G', 'Q', 'J', 'R']));
        assert_eq!(stacks[8], rev(vec!['S', 'F', 'P', 'H', 'R', 'T', 'D', 'L']));

        assert_eq!(moves.len(), 4);
        assert_eq!(moves[0].count, 8);
        assert_eq!(moves[0].from, 6);
        assert_eq!(moves[0].to, 0);
        
        assert_eq!(moves[1].count, 9);
        assert_eq!(moves[1].from, 0);
        assert_eq!(moves[1].to, 8);
        
        assert_eq!(moves[2].count, 4);
        assert_eq!(moves[2].from, 4);
        assert_eq!(moves[2].to, 3);
        
        assert_eq!(moves[3].count, 4);
        assert_eq!(moves[3].from, 5);
        assert_eq!(moves[3].to, 0);
        
    }

    #[test]
    fn test_move_crates() {
        //fn move_crates(stacks: &mut Stacks, m: &Move)
        let mut stacks: Stacks = vec![vec!['A', 'B', 'C'], vec!['D'], vec!['E', 'F', 'G']];

        let m = Move {
            count: 1,
            from: 0,
            to: 2
        };

        move_crates(&mut stacks, &m, true);
        assert_eq!(stacks[0], vec!['A', 'B']);
        assert_eq!(stacks[1], vec!['D']);
        assert_eq!(stacks[2], vec!['E', 'F', 'G', 'C']);
    }

    #[test]
    fn test_move_crates2() {
        //fn move_crates(stacks: &mut Stacks, m: &Move)
        let mut stacks: Stacks = vec![vec!['A', 'B', 'C'], vec!['D'], vec!['E', 'F', 'G']];

        {
            let m = Move {
                count: 2,
                from: 0,
                to: 2
            };

            move_crates(&mut stacks, &m, true);
            assert_eq!(stacks[0], vec!['A']);
            assert_eq!(stacks[1], vec!['D']);
            assert_eq!(stacks[2], vec!['E', 'F', 'G', 'C', 'B']);
        }

        {
            let m = Move {
                count: 5,
                from: 2,
                to: 1
            };

            move_crates(&mut stacks, &m, true);
            assert_eq!(stacks[0], vec!['A']);
            assert_eq!(stacks[1], vec!['D', 'B', 'C', 'G', 'F', 'E']);
            assert_eq!(stacks[2], vec![] as std::vec::Vec<char>);
        }
    }
}

