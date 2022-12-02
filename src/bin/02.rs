#[derive(PartialEq, Eq)]
enum Sign {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn value(&self) -> usize {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl Sign {
    fn value(&self) -> usize {
        match self {
            Sign::Rock => 1,
            Sign::Paper => 2,
            Sign::Scissors => 3,
        }
    }
    fn play(&self, other: &Sign) -> Outcome {
        match (self, other) {
            (a, b) if a == b => Outcome::Draw,
            (Sign::Rock, Sign::Paper) => Outcome::Loss,
            (Sign::Paper, Sign::Scissors) => Outcome::Loss,
            (Sign::Scissors, Sign::Rock) => Outcome::Loss,
            _ => Outcome::Win,
        }
    }
    fn score(&self, other: &Sign) -> usize {
        let match_outcome = self.play(other);
        self.value() + match_outcome.value()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = String::from_utf8(std::fs::read("inputs/02.txt")?)?;

    let mut total_score = 0;
    for line in input.lines() {
        let mut line_chars = line.chars();
        let opponent_sign = line_chars.next().unwrap();
        line_chars.next();
        let my_sign = line_chars.next().unwrap();

        let opponent_sign = match opponent_sign {
            'A' => Sign::Rock,
            'B' => Sign::Paper,
            'C' => Sign::Scissors,
            _ => panic!("invalid move"),
        };

        let my_sign = match my_sign {
            'X' => Sign::Rock,
            'Y' => Sign::Paper,
            'Z' => Sign::Scissors,
            _ => panic!("invalid move"),
        };

        let result = my_sign.score(&opponent_sign);
        total_score += result
    }

    println!("total score: {total_score}");

    Ok(())
}
