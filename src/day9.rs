use std::collections::HashSet;

use crate::common::read_file;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Vec2d {
    x: i32,
    y: i32
}

impl Vec2d {
    fn new(x: i32, y: i32) -> Vec2d {
        Vec2d { x, y }
    }

    fn surroundings(&self) -> HashSet<Vec2d> {
        let mut hash = HashSet::<Vec2d>::new();
        for x in -1..=1 {
            for y in -1..=1 {
                hash.insert(Vec2d::new(self.x + x, self.y + y));         
            }
        }

        hash
    }
}

#[derive(Clone, Copy, Debug)]
struct Rope {
    head: Vec2d,
    tail: Vec2d
}

impl Rope {
    fn new() -> Rope {
        let start = Vec2d::new(0,0);
        Rope {
            head: start,
            tail: start
        }
    }

    fn move_head(&mut self, dir: &Vec2d) {
        assert!(dir.x.abs() <= 1);
        assert!(dir.y.abs() <= 1);

        let last_pos = self.head;
        self.head.x += dir.x;
        self.head.y += dir.y;

        let x_dist = (self.head.x - self.tail.x).abs();
        let y_dist = (self.head.y - self.tail.y).abs();

        if x_dist > 1 || y_dist > 1 {
            self.tail = last_pos;
        }
    }
}

fn parse_moves() -> Result<std::vec::Vec<Vec2d>, Box<dyn std::error::Error>> {
    let lines = read_file("day9.txt")?;

    let mut result = vec![];

    for ln in lines {
        let (dir, dist) = match ln.split_once(' ') {
            None => return Err(Box::from("Unable to split line")),
            Some(x) => (x.0.as_bytes()[0], x.1.parse::<usize>()?)
        };

        let iter = std::iter::repeat(
            match dir {
                b'U' => Vec2d::new(0, 1),
                b'D' => Vec2d::new(0, -1),
                b'L' => Vec2d::new(-1, 0),
                b'R' => Vec2d::new(1, 0),
                _ => return Err(Box::from(format!("Unknown direction: {}", dir as char)))
            });

        result.extend(iter.take(dist));
    }

    Ok(result)
}

pub fn run_part1() -> Result<(), Box<dyn std::error::Error>> {
    let moves = parse_moves()?;
    let mut visited = HashSet::new();

    let mut rope = Rope::new();
    for m in moves {
        rope.move_head(&m);
        visited.insert(rope.tail);
    }

    println!("Visited: {}", visited.len());
    Ok(())
}

pub fn run_part2() -> Result<(), Box<dyn std::error::Error>> {
    let mut rope = vec![Vec2d::new(0,0); 10];
    let mut visited = HashSet::new();

    visited.insert(rope[0]);
    let moves = parse_moves()?;
    for m in moves {
        rope[0].x += m.x;
        rope[0].y += m.y;

        for i in 1..rope.len() {
            
            let last = rope[i-1].clone();
            let s1 = last.surroundings();
            let s2 = rope[i].surroundings();
            let intersection: std::vec::Vec<_> = s1.intersection(&s2).collect();
            if intersection.contains(&&rope[i]) {
                break;
            }
            else if intersection.len() == 1 {
                rope[i] = *intersection[0];
            }
            else {
                rope[i] = {
                    let mut r = None;
                    for i in 0..intersection.len() {
                        let v = intersection[i];
                        let x = v.x - last.x; 
                        let y = v.y - last.y;
                        if x == 0 || y == 0 {
                            r = Some(*v);
                            break;
                        }
                    }

                    match r {
                        None => return Err(Box::from("Did not find adjacent cell")),
                        Some(v) => v
                    }
                }
            }

            if i == (rope.len() - 1) {
                visited.insert(rope[i]);
            }
        }
    }

    println!("Visited: {}", visited.len());
    Ok(())
}

