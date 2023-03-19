fn main() {
    // Parse strategy guide
    let strategy_data = include_str!("day2.txt").trim().replace("\r", "");
    let strategy = strategy_data.lines().map(|line| Play::from(line));

    // Solve part 1 by totaling our score
    let solution: i32 = strategy.map(|s| s.your_points()).sum();
    println!("Part 1: {solution}");

    // Solve part 2 by parsing the input with the updated instructions
    let strategy = strategy_data.lines().map(|line| Play::from_part2(line));
    let solution: i32 = strategy.map(|s| s.your_points()).sum();
    println!("Part 2: {solution}");
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

    /// Returns the move against this move to get the desired outcome
    fn counter_for_outcome(self, outcome: Outcome) -> Move {
        match outcome {
            Outcome::Tie => self,
            Outcome::Win => match self {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
            Outcome::Loss => match self {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
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

    fn from_part2(value: &str) -> Self {
        let mut parts = value.split(" ");

        let their_move = match parts.next().unwrap() {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            part => panic!("{part} is not a valid move!"),
        };

        let desired_outcome = match parts.next().unwrap() {
            "X" => Outcome::Loss,
            "Y" => Outcome::Tie,
            "Z" => Outcome::Win,
            part => panic!("{part} is not a valid desired outcome!"),
        };

        assert_eq!(None, parts.next(), "line '{value}' contained too many columns");

        Play {
            their_move,
            your_move: their_move.counter_for_outcome(desired_outcome),
        }
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
