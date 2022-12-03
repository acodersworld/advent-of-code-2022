use std::fs;

fn parse_input(input: &str) -> Result<std::vec::Vec<i32>, Box<dyn std::error::Error>> {
    let mut elves = vec![];
    let cals = input.split("\n");

    let mut curr = 0;
    for x in cals {
        if x.is_empty() {
            if curr > 0 {
                elves.push(curr);
            }
            curr = 0;
        }
        else {
            curr += x.parse::<i32>()?;
        }
    }

    if curr > 0 {
        elves.push(curr);
    }
    Ok(elves)
}

fn find_top_three(input: std::vec::Vec<i32>) -> std::vec::Vec<i32> {
    let mut top = vec![];

    for x in &input {
        let pos = top.partition_point(|v| *v > *x);
        if pos < 3 {
            top.insert(pos, *x);
            if top.len() > 3 {
                top.pop();
            }
        }
    }

    top
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("day1.txt")?;
    
    let elves = parse_input(&contents)?;

    let top_three = find_top_three(elves);
    let sum: i32 = top_three.iter().sum();

    println!("{}", sum);
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse_input() {
        let input = 
r#"
1
2
3

5
6
7

8
9
1"#;
        let output = super::parse_input(input).unwrap();
        for x in &output  { println!("X: {} ", x); }
        assert_eq!(output.len(), 3);
        assert_eq!(output[0], 6);
        assert_eq!(output[1], 18);
        assert_eq!(output[2], 18);
    }

    #[test]
    fn test_top_three() {

        let output = super::find_top_three(vec![5, 3, 5, 7, 1, 9, 10]);
        assert_eq!(output, vec![10, 9, 7]);
    }
}

