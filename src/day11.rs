use crate::common::read_file2;
use nom;

#[derive(Debug, Clone, Copy)]
enum Op {
    Mul(u64),
    Add(u64),
    Square
}

impl Op {
    fn execute(&self, val: u64) -> u64 {
        match self {
            Op::Mul(x) => val * x,
            Op::Add(x) => val + x,
            Op::Square => val * val
        }
    }
}

enum WorryReducer {
    Div3,
    Mod(u64)
}

impl WorryReducer {
    fn execute(&self, val: u64) -> u64 {
        match self {
            WorryReducer::Div3 => val / 3,
            WorryReducer::Mod(m) => val % m
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: std::vec::Vec<u64>,
    op: Op,
    div: u64,
    true_monkey: usize,
    false_monkey: usize,
    throw_count: u64
}

fn parse(lines: &std::vec::Vec<String>) -> Result<std::vec::Vec<Monkey>, Box<dyn std::error::Error>> {
    let mut monkeys = vec![];

    use nom::{
        sequence::preceded,
        sequence::tuple,
        character::complete::space1,
        character::complete::digit1,
        bytes::complete::tag,
        multi::separated_list1,
        combinator::map_res,
        branch::alt,
        IResult
    };

    for text in lines.chunks(7) {
        let items: IResult<&str, std::vec::Vec<u64>> = preceded(
            tuple((space1, tag("Starting items:"), space1)),
            separated_list1(tag(", "), map_res(digit1, |s| u64::from_str_radix(s, 10)))
        )(&text[1]);
        let items = items.unwrap().1;

        let op: IResult<&str, Op> = preceded(
            tuple((space1, tag("Operation: new = old"), space1)),
            map_res(
                tuple((
                    alt((tag("*"), tag("+"))),
                    space1,
                    alt((digit1, tag("old")))
                )),
                |(op_code, _space, val)|
                    match (op_code, val) {
                        ("*", "old") => Ok(Op::Square),
                        ("*", _) => Ok(Op::Mul(val.parse().unwrap())),
                        ("+", _) => Ok(Op::Add(val.parse().unwrap())),
                        _ => Err(format!("Unknown op {}", op_code))
                    }
                ) 
        )(&text[2]);
        let op = op.unwrap().1;

        let div: IResult<&str, u64> = preceded(
            tuple((space1, tag("Test: divisible by"), space1)),
            map_res(digit1, |s| u64::from_str_radix(s, 10))
        )(&text[3]);
        let div = div.unwrap().1;

        let true_monkey: IResult<&str, usize> = preceded(
            tuple((space1, tag("If true: throw to monkey"), space1)),
            map_res(digit1, |s| usize::from_str_radix(s, 10))
        )(&text[4]);
        let true_monkey = true_monkey.unwrap().1;
        
        let false_monkey: IResult<&str, usize> = preceded(
            tuple((space1, tag("If false: throw to monkey"), space1)),
            map_res(digit1, |s| usize::from_str_radix(s, 10))
        )(&text[5]);
        let false_monkey = false_monkey.unwrap().1;

        monkeys.push(Monkey {
            items,
            op,
            div,
            true_monkey,
            false_monkey,
            throw_count: 0
        });
    }

    Ok(monkeys)
}

fn load_monkeys() -> Result<std::vec::Vec<Monkey>, Box<dyn std::error::Error>> {
    let lines = read_file2("day11.txt")?;
    Ok(parse(&lines)?)
}

fn run_monkeys(mut monkeys: std::vec::Vec<Monkey>, worry_reducer: WorryReducer, rounds: usize) {
    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            monkeys[m].throw_count += monkeys[m].items.len() as u64;

            let m = {
                let monkey = &mut monkeys[m];
                let ret = Monkey {
                    items: monkey.items.clone(),
                    op: monkey.op,
                    div: monkey.div,
                    true_monkey: monkey.true_monkey,
                    false_monkey: monkey.false_monkey,
                    throw_count: 0
                };
                monkey.items.clear();
                ret
            };

            for i in m.items {
                let worry_level = worry_reducer.execute(m.op.execute(i));
                let to_monkey = if (worry_level % m.div) == 0 { m.true_monkey } else { m.false_monkey };
                monkeys[to_monkey].items.push(worry_level);
            }
        }
    }

    let mut throw_counts: std::vec::Vec<_> = monkeys.iter().map(|m| m.throw_count).collect();
    throw_counts.sort_by(|a, b| u64::cmp(b, a));
    println!("Business: {}", throw_counts[0] * throw_counts[1]);
}

pub fn run_part1() -> Result<(), Box<dyn std::error::Error>> {
    let monkeys = load_monkeys()?;
    run_monkeys(monkeys, WorryReducer::Div3, 20);
    Ok(())
}

pub fn run_part2() -> Result<(), Box<dyn std::error::Error>> {
    let monkeys = load_monkeys()?;
    let modulus = monkeys.iter().map(|m| m.div).product();
    run_monkeys(monkeys, WorryReducer::Mod(modulus), 10000);
    Ok(())
}

