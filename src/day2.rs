use std::fs;

#[derive(Eq, PartialEq, Debug)]
enum GameResult {
    Win,
    Lose,
    Draw
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors
}

impl Hand {
    fn from_str(input: &str) -> Option<Hand> {
        match input {
            "A" | "X" => Some(Hand::Rock),
            "B" | "Y" => Some(Hand::Paper),
            "C" | "Z" => Some(Hand::Scissors),
            _ => None
        }
    }

    fn get_advised_hand(&self, input: &str) -> Option<Hand> {
        let desired_result = match input {
            "X" => GameResult::Lose,
            "Y" => GameResult::Draw,
            "Z" => GameResult::Win,
            _ => panic!("Bad advised input")
        };

        Some(
            if desired_result == GameResult::Draw {
                *self
            }
            else if desired_result == GameResult::Win {
                match *self {
                    Hand::Rock => Hand::Paper,
                    Hand::Paper => Hand::Scissors,
                    Hand::Scissors => Hand::Rock
                }
            }
            else { // Lose
                match *self {
                    Hand::Rock => Hand::Scissors,
                    Hand::Paper => Hand::Rock,
                    Hand::Scissors => Hand::Paper
                }
            }
        )
    }

    fn score(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3
        }
    }

    fn get_result(&self, opp: Hand) -> GameResult {
        if *self == opp {
            return GameResult::Draw
        }

        let win = match *self {
            Hand::Rock => opp == Hand::Scissors,
            Hand::Paper => opp == Hand::Rock,
            Hand::Scissors => opp == Hand::Paper
        };

        if win {
            GameResult::Win
        }
        else {
            GameResult::Lose
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Round {
    opp: Hand,
    me: Hand
}

impl Round {
    fn new(opp: Hand, me: Hand) -> Round {
        Round {
            opp,
            me
        }
    }

    fn calc_result(&self) -> i32 {
        let res = self.me.get_result(self.opp);
        let hand_score = self.me.score();

        hand_score + if res == GameResult::Draw {
            3
        }
        else if res == GameResult::Win {
            6
        }
        else { // Lose
            0
        }
    }
}

pub fn run_part1() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("day2.txt")?;
    let total_score = calc_total_score(&contents);
    println!("Total score: {}", total_score); 
    Ok(())
}

pub fn run_part2() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("day2.txt")?;
    let total_score = calc_total_score_advised(&contents);
    println!("Total score: {}", total_score); 
    Ok(())
}

fn calc_total_score(contents: &str) -> i32 {
    let rounds = parse_input(&contents);

    rounds.iter().map(|r| r.calc_result()).sum()
}

fn calc_total_score_advised(contents: &str) -> i32 {
    let rounds = parse_input_advised(&contents);

    rounds.iter().map(|r| r.calc_result()).sum()
}
fn parse_input(input: &str) -> std::vec::Vec<Round> {
    let rounds = input.split("\n");
    let rounds: std::vec::Vec<Round> = rounds.filter(|line| !line.is_empty()).map(|line| {
        let mut hands = line.split(" ");

        let opp = match hands.next() {
            Some(x) => x.trim(),
            None => panic!("Expected first hand")
        };

        let me = match hands.next() {
            Some(x) => x.trim(),
            None => panic!("Expected second hand")
        };

        Round::new(
            Hand::from_str(opp).expect("Valid hand opp"),
            Hand::from_str(me).expect("Valid hand me"))
    }).collect();

    rounds
}

fn parse_input_advised(input: &str) -> std::vec::Vec<Round> {
    let rounds = input.split("\n");
    let rounds: std::vec::Vec<Round> = rounds.filter(|line| !line.is_empty()).map(|line| {
        println!("{}", line);
        let mut hands = line.split(" ");

        let opp = match hands.next() {
            Some(x) => x.trim(),
            None => panic!("Expected first hand")
        };

        let me = match hands.next() {
            Some(x) => x.trim(),
            None => panic!("Expected second hand")
        };

        let opp = Hand::from_str(opp).expect("Valid hand opp");
        Round::new(
            opp,
            opp.get_advised_hand(me).expect("Valid hand me"))
    }).collect();

    rounds
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input =
r#"A X
A Y
A Z
B X
B Y
B Z
C X
C Y
C Z"#;
        let rounds = parse_input(input);
        assert_eq!(rounds.len(), 9);
        assert_eq!(rounds[0], Round::new(Hand::Rock, Hand::Rock));
        assert_eq!(rounds[1], Round::new(Hand::Rock, Hand::Paper));
        assert_eq!(rounds[2], Round::new(Hand::Rock, Hand::Scissors));

        assert_eq!(rounds[3], Round::new(Hand::Paper, Hand::Rock));
        assert_eq!(rounds[4], Round::new(Hand::Paper, Hand::Paper));
        assert_eq!(rounds[5], Round::new(Hand::Paper, Hand::Scissors));

        assert_eq!(rounds[6], Round::new(Hand::Scissors, Hand::Rock));
        assert_eq!(rounds[7], Round::new(Hand::Scissors, Hand::Paper));
        assert_eq!(rounds[8], Round::new(Hand::Scissors, Hand::Scissors));
    }

    #[test]
    fn test_hand() {
        assert_eq!(Hand::Rock.get_result(Hand::Rock), GameResult::Draw);
        assert_eq!(Hand::Paper.get_result( Hand::Paper), GameResult::Draw);
        assert_eq!(Hand::Scissors.get_result( Hand::Scissors), GameResult::Draw);

        assert_eq!(Hand::Paper.get_result( Hand::Scissors), GameResult::Lose);
        assert_eq!(Hand::Scissors.get_result(Hand::Rock), GameResult::Lose);
        assert_eq!(Hand::Rock.get_result(Hand::Paper), GameResult::Lose);

        assert_eq!(Hand::Scissors.get_result(Hand::Paper), GameResult::Win);
        assert_eq!(Hand::Rock.get_result(Hand::Scissors), GameResult::Win);
        assert_eq!(Hand::Paper.get_result(Hand::Rock), GameResult::Win);
    }

    #[test]
    fn test_round() {
        assert_eq!(Round::new(Hand::Rock, Hand::Rock).calc_result(), 3 + 1);
        assert_eq!(Round::new(Hand::Paper, Hand::Paper).calc_result(), 3 + 2);
        assert_eq!(Round::new(Hand::Scissors, Hand::Scissors).calc_result(), 3 + 3);

        assert_eq!(Round::new(Hand::Paper, Hand::Rock).calc_result(), 1);
        assert_eq!(Round::new(Hand::Scissors, Hand::Paper).calc_result(), 2);
        assert_eq!(Round::new(Hand::Rock, Hand::Scissors).calc_result(), 3);

        assert_eq!(Round::new(Hand::Scissors, Hand::Rock).calc_result(), 6 + 1);
        assert_eq!(Round::new(Hand::Rock, Hand::Paper).calc_result(), 6 + 2);
        assert_eq!(Round::new(Hand::Paper, Hand::Scissors).calc_result(), 6 + 3);
    }

    #[test]
    fn total_test() {
        let input =
r#"A Y
B X
C Z"#;

        let total_score = calc_total_score(&input);
        assert_eq!(total_score, 15);
    }

    #[test]
    fn test_hand_advised() {
        assert_eq!(Hand::Rock.get_advised_hand("X").unwrap(), Hand::Scissors);
        assert_eq!(Hand::Rock.get_advised_hand("Y").unwrap(), Hand::Rock);
        assert_eq!(Hand::Rock.get_advised_hand("Z").unwrap(), Hand::Paper);

        assert_eq!(Hand::Paper.get_advised_hand("X").unwrap(), Hand::Rock);
        assert_eq!(Hand::Paper.get_advised_hand("Y").unwrap(), Hand::Paper);
        assert_eq!(Hand::Paper.get_advised_hand("Z").unwrap(), Hand::Scissors);

        assert_eq!(Hand::Scissors.get_advised_hand("X").unwrap(), Hand::Paper);
        assert_eq!(Hand::Scissors.get_advised_hand("Y").unwrap(), Hand::Scissors);
        assert_eq!(Hand::Scissors.get_advised_hand("Z").unwrap(), Hand::Rock);
    }

    #[test]
    fn test_round_advised() {
        let input =
r#"A X
A Y
A Z
B X
B Y
B Z
C X
C Y
C Z"#;
        let rounds = parse_input_advised(input);
        assert_eq!(rounds.len(), 9);
        assert_eq!(rounds[0], Round::new(Hand::Rock, Hand::Scissors));
        assert_eq!(rounds[1], Round::new(Hand::Rock, Hand::Rock));
        assert_eq!(rounds[2], Round::new(Hand::Rock, Hand::Paper));

        assert_eq!(rounds[3], Round::new(Hand::Paper, Hand::Rock));
        assert_eq!(rounds[4], Round::new(Hand::Paper, Hand::Paper));
        assert_eq!(rounds[5], Round::new(Hand::Paper, Hand::Scissors));

        assert_eq!(rounds[6], Round::new(Hand::Scissors, Hand::Paper));
        assert_eq!(rounds[7], Round::new(Hand::Scissors, Hand::Scissors));
        assert_eq!(rounds[8], Round::new(Hand::Scissors, Hand::Rock));
    }
}
