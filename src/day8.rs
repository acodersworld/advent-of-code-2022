use crate::common::read_file;

type Forest = std::vec::Vec<std::vec::Vec<u8>>;

fn set_visible<'a, I>(iter: I) where I: Iterator<Item = &'a mut u8>
{
    let mut tallest = 0u8;
    for v in iter {
        if (*v & 0x7f) > tallest {
            tallest = *v & 0x7f;
            *v |= 0x80; 
        }
    }
}

fn calc_scenic_score_iter<'a, I>(iter: I, max_height: u8) -> usize where I: Iterator<Item = &'a u8> {
    let mut score = 0;
    for v in iter {
        score += 1;
        if *v >= max_height {
            break;
        }
    }

    score
}

fn calc_scenic_score(forest: &Forest, row: usize, col: usize) -> usize {
    let height = forest[row][col];

    let up = calc_scenic_score_iter(forest[0..row].iter().map(|r| &r[col]).rev(), height);
    let down = calc_scenic_score_iter(forest[row+1..forest.len()].iter().map(|r| &r[col]), height);

    let left = calc_scenic_score_iter(forest[row][0..col].iter().rev(), height);
    let right = calc_scenic_score_iter(forest[row][col+1..forest[row].len()].iter(), height);
    
    up * down * left * right
}

pub fn generate_forest() -> Result<Forest, Box<dyn std::error::Error>> {
    let lines = read_file("day8.txt")?;

    let forest = lines.iter()
        .map(|ln| ln.as_bytes().iter().map(|v| v - b'0').collect())
        .collect();

    Ok(forest)
}

pub fn run_part1() -> Result<(), Box<dyn std::error::Error>> {
    let mut forest = generate_forest()?;

    let row_1 = forest.len() - 1;
    let col_1 = forest[0].len() - 1;

    for v in forest[0].iter_mut() { *v |= 0x80; }
    for v in &mut forest[row_1].iter_mut() { *v |= 0x80; }

    for row in &mut forest[1..row_1] {
        row[0] |= 0x80;
        row[col_1] |= 0x80;
    }

    for r in &mut forest {
        set_visible(r.iter_mut());
        set_visible(r.iter_mut().rev());
    }

    for col in 0..forest[0].len() {
        set_visible(forest.iter_mut().map(|r| &mut r[col]));
        set_visible(forest.iter_mut().map(|r| &mut r[col]).rev());
    }

    let count = forest.iter()
                    .flat_map(|row| row.iter())
                    .filter(|v| (*v & 0x80) == 0x80)
                    .count();

    println!("Count: {}", count);
    Ok(())
}

pub fn run_part2() -> Result<(), Box<dyn std::error::Error>> {
    let forest = generate_forest()?;

    let mut max_score = 0;

    let col_len = forest[0].len() - 1;
    for row in 0..forest.len()-1 {
        for col in 0..col_len {
            max_score = std::cmp::max(max_score, calc_scenic_score(&forest, row, col));
        }
    }

    println!("Max score: {}", max_score);
    Ok(())
}

