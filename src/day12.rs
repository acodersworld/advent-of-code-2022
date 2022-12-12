use crate::common::read_file;

#[derive(Copy, Clone, PartialEq, Eq)]
struct Cell {
    row: usize,
    col: usize
}

impl Cell {
    fn new(row: usize, col: usize) -> Cell {
        Cell { row, col }
    }
}

struct QueuedCell {
    cell: Cell,
    steps: u32
}

impl QueuedCell {
    fn new(cell: Cell, steps: u32) -> QueuedCell {
        QueuedCell { cell, steps }
    }
}

fn get_index_mask(cell: &Cell, width: usize) -> (usize, u8) {
    let idx = (cell.row * width) + cell.col;
    let mask = 1 << (idx % 8);

    (idx, mask)
}

fn set_explored(cell: &Cell, width: usize, explored: &mut std::vec::Vec<u8>) {
    let (idx, mask) = get_index_mask(cell, width);
    explored[idx / 8] |= mask;
}

fn is_explored(cell: &Cell, width: usize, explored: &std::vec::Vec<u8>) -> bool {
    let (idx, mask) = get_index_mask(cell, width);
    (explored[idx / 8] & mask) != 0
}

fn search(start: &Cell, map: &std::vec::Vec<std::vec::Vec<char>>, from_a: bool) {
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    let mut queue = std::collections::VecDeque::new();
    let mut explored = vec![0u8; ((height * width) + 7 / 8) as usize];

    queue.push_back(QueuedCell::new(*start, 0));
    set_explored(start, width as usize, &mut explored);

    if from_a {
        for (r, row) in map.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if cell == 'a' {
                    let cell = Cell::new(r, c);
                    queue.push_back(QueuedCell::new(cell, 0));
                    set_explored(&cell, width as usize, &mut explored);
                }
            }
        }
    }

    while !queue.is_empty() {
        let next = queue.pop_front().unwrap();

        if map[next.cell.row][next.cell.col] == 'E' {
            println!("Found: {}", next.steps);
            return;
        }

        for (row, col) in [(0,1), (0,-1), (1,0), (-1,0)] {
            let nr = next.cell.row as i32 + row;
            let nc = next.cell.col as i32 + col;

            if nr < 0 || nc < 0 || nr >= height || nc >= width {
                continue;
            }

            if is_explored(&Cell::new(nr as usize, nc as usize), width as usize, &explored) {
                continue;
            }

            let mut this_cell = map[next.cell.row][next.cell.col];
            if this_cell == 'S' {
                this_cell = 'a';
            }

            let mut next_cell = map[nr as usize][nc as usize];
            if next_cell == 'E' {
                next_cell = 'z';
            }

            if (next_cell as i8 - this_cell as i8) > 1 {
                continue;
            }

            let steps = next.steps + 1;

            let cell = Cell::new(nr as usize, nc as usize);
            queue.push_back(QueuedCell::new(cell, steps));
            set_explored(&cell, width as usize, &mut explored);
        }
    }
}

fn find_start(map: &std::vec::Vec<std::vec::Vec<char>>) -> Result<Cell, Box<dyn std::error::Error>> {
    for (r, row) in map.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                return Ok(Cell::new(r, c));
            }
        }
    }

    Err(Box::from("Did not find start"))
}

pub fn run_part1() -> Result<(), Box<dyn std::error::Error>> {
    let map: std::vec::Vec<_> = read_file("day12.txt")?.iter().map(|line| line.chars().collect::<std::vec::Vec<char>>()).collect();
    let start = find_start(&map)?;

    search(&start, &map, false);
    Ok(())
}

pub fn run_part2() -> Result<(), Box<dyn std::error::Error>> {
    let map: std::vec::Vec<_> = read_file("day12.txt")?.iter().map(|line| line.chars().collect::<std::vec::Vec<char>>()).collect();
    let start = find_start(&map)?;

    search(&start, &map, true);
    Ok(())
}


