use crate::common::read_file;

struct CPU {
    cycle: i32,
    reg_x: i32,

    signal_strength_sum: i32,
    output_crt: bool,
    crt: std::vec::Vec<String>
}

impl CPU {
    fn new() -> CPU {
        CPU {
            cycle: 0,
            reg_x: 1,
            signal_strength_sum: 0,

            output_crt: false,
            crt: vec![]
        }
    }

    fn cycle(&mut self) {
        if self.output_crt {
            let drawing_pixel_pos = (self.cycle % 40) + 1;
            if drawing_pixel_pos == 1 {
                self.crt.push(String::new());
            }

            let row = self.crt.last_mut().unwrap();
            let draw_pixel = (self.reg_x..self.reg_x + 3).contains(&drawing_pixel_pos);
            row.push(if draw_pixel { '#' } else { '.' });
        }

        self.cycle += 1;
        if self.cycle > 19 {
            let rel = self.cycle - 20;
            if (rel % 40) == 0 {
                self.signal_strength_sum += self.cycle * self.reg_x;
            }
        }
    }

    fn noop(&mut self) {
        self.cycle();
    }

    fn add(&mut self, val: i32) {
        self.cycle();
        self.cycle();
        self.reg_x += val;
    }
}

fn run(cpu: &mut CPU, lines: &std::vec::Vec<String>) -> Result<i32, Box<dyn std::error::Error>> {
    for ln in lines {
        match &ln[..4] {
            "noop" => cpu.noop(),
            "addx" => {
                let val: i32 = ln[5..].parse()?;
                cpu.add(val);
            },
            cmd => return Err(Box::from(format!("Unknown command {}", cmd)))
        };
    }
    
    Ok(cpu.signal_strength_sum)
}

pub fn run_part1() -> Result<(), Box<dyn std::error::Error>> {
    let mut cpu = CPU::new();
    let lines = read_file("day10.txt")?;
    let strength_sum = run(&mut cpu, &lines)?;

    println!("Strength sum: {}", strength_sum);
    Ok(())
}

pub fn run_part2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cpu = CPU::new();
    cpu.output_crt = true;

    let lines = read_file("day10.txt")?;
    run(&mut cpu, &lines)?;

    for row in cpu.crt {
        println!("{}", row);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let content =
r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

        let mut cpu = CPU::new();
        cpu.output_crt = true;
        let val = match run(&mut cpu, &content.split("\n").map(|l| l.to_owned()).collect()) {
            Err(x) => panic!("{}", x),
            Ok(v) => v
        };

        assert_eq!(val, 13140);

        let expected_crt: std::vec::Vec<&str> =
r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#.split('\n').collect();

        for (a, b) in itertools::zip_eq(expected_crt, cpu.crt) {
            assert_eq!(a, b);
        }
    }
}

