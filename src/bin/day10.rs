enum Instruction {
    AddX(i32),
    NoOp,
}

impl Instruction {
    fn parse(string: &str) -> Self {
        if let Some(addx) = string.strip_prefix("addx ") {
            Self::AddX(addx.parse().unwrap())
        } else {
            Self::NoOp
        }
    }

    fn cycles(&self) -> usize {
        match self {
            Self::AddX(_) => 2,
            Self::NoOp => 1,
        }
    }
}

fn main() {
    let input = aoc2022::read_input_for_day(10);

    // `.rev()` so the instructions can be obtained using `.pop()`
    let mut instructions = input.lines().rev().map(Instruction::parse).collect::<Vec<_>>();
    
    let mut register = 1;
    let mut executing = Instruction::NoOp;
    let mut stall = 0;

    let mut part1 = 0;
    let mut part2 = String::new();

    // Start at 0 to obtain the first instruction
    for cycle in 0.. {
        if cycle > 0 {
            let col = (cycle - 1) % 40;
            if col == register - 1 || col == register || col == register + 1 {
                part2.push('#');
            } else {
                part2.push('.');
            }

            if col == 39 {
                part2.push('\n');
            }
        }

        if let 20 | 60 | 100 | 140 | 180 | 220 = cycle {
            part1 += cycle * register;
        }

        if stall > 0 {
            stall -= 1;
            continue;
        }

        match executing {
            Instruction::AddX(x) => register += x,
            Instruction::NoOp => {}
        }

        if let Some(instr) = instructions.pop() {
            executing = instr;
            stall = executing.cycles() - 1; // -1 to count this cycle
        } else {
            break;
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2:\n{}", part2);
}