fn main() {
    // Parse strategy guide
    let strategy_data = include_str!("input.txt").trim().replace("\r", "");
    let strategy = strategy_data.lines().map(|line| Play::from(line));

    // Solve part 1 by totaling our score
    let solution: i32 = strategy.map(|s| s.your_points()).sum();
    println!("Part 1: {solution}");
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn beats(self, other: Move) -> Outcome {
        match (self, other) {
            (Move::Rock, Move::Scissors) | (Move::Paper, Move::Rock) | (Move::Scissors, Move::Paper) => Outcome::Win,
            _ if self == other => Outcome::Tie,
            _ => Outcome::Loss,
        }
    }

    fn score(self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Win,
    Loss,
    Tie
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Outcome::Loss => 0,
            Outcome::Tie => 3,
            Outcome::Win => 6,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Play {
    their_move: Move,
    your_move: Move,
}

impl Play {
    fn your_points(&self) -> i32 {
        self.your_move.score() +
            self.your_move.beats(self.their_move).score()
    }
}

impl From<&str> for Play {
    fn from(value: &str) -> Self {
        let mut moves = value.split(" ").map(|c| match c {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("{c} is not a valid move!"),
        });

        let result = Play {
            their_move: moves.next().unwrap(),
            your_move: moves.next().unwrap(),
        };
        assert_eq!(None, moves.next(), "line '{value}' contained too many moves");

        result
    }
}
