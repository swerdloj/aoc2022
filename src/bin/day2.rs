#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from(c: &str) -> Self {
        match c {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,

            _ => panic!("Invalid move"),
        }
    }

    fn value(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

fn main() {
    let input = aoc2022::read_input_for_day(2);

    // (opponent, you)
    // (A, X) = rock     = +1
    // (B, Y) = paper    = +2
    // (C, Z) = scissors = +3
    //   loss = +0
    //   draw = +3
    //   win  = +6

    let part1: u64 = input.lines().fold(0, |total, play| {
        let moves: Vec<_> = play.split(' ').map(Move::from).collect();

        let opponent = moves[0];
        let me = moves[1];

        let outcome = if opponent == me {
            // Draw
            3
        } else if opponent == Move::Rock  && me == Move::Paper
            || opponent == Move::Paper    && me == Move::Scissors
            || opponent == Move::Scissors && me == Move::Rock 
        {
            // Win
            6
        } else {
            // Loss
            0
        };

        total + me.value() + outcome
    });


    // (opponent, outcome)
    // A = rock     = +1
    // B = paper    = +2
    // C = scissors = +3
    //
    // X = loss     = +0
    // Y = draw     = +3
    // Z = win      = +6

    let part2: u64 = input.lines().fold(0, |mut total, play| {
        let moves: Vec<_> = play.split(' ').collect();

        let opponent = Move::from(moves[0]);
        let outcome = moves[1];

        match outcome {
            // Loss
            "X" => {
                 match opponent {
                    Move::Rock     => total += Move::Scissors.value(),
                    Move::Paper    => total += Move::Rock.value(),
                    Move::Scissors => total += Move::Paper.value(),
                }
            }

            // Draw
            "Y" => {
                total += 3;
                total += opponent.value();
            }

            // Win
            "Z" => {
                total += 6;

                match opponent {
                    Move::Rock     => total += Move::Paper.value(),
                    Move::Paper    => total += Move::Scissors.value(),
                    Move::Scissors => total += Move::Rock.value(),
                }
            }

            _ => panic!("Invalid outcome"),
        }

        total
    });

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}