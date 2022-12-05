fn main() {
    let input = aoc2022::read_input_for_day(5);
    
    const EMPTY: Vec<char> = Vec::new();
    let mut p1_stacks = [EMPTY; 9];

    input.lines().take(8).for_each(|line| {
        let chars: Vec<_> = line.chars().collect();

        for pos in 0..9 {
            // Crate labels start at position 1 and occur every 4 columns
            let c = chars[pos * 4 + 1];

            if c.is_alphabetic() {
                p1_stacks[pos].push(c);
            }
        }
    });

    p1_stacks.iter_mut().for_each(|s| s.reverse());
    let mut p2_stacks = p1_stacks.clone();

    input.lines().skip(10).for_each(|line| {
        let instructions: Vec<usize> = line
            .split(' ')
            .filter_map(|item| item.parse().ok())
            .collect();

        let count = instructions[0];
        let from  = instructions[1] - 1;
        let to    = instructions[2] - 1;

        // Part 1 solution
        for _ in 0..count {
            let grabbed = p1_stacks[from].pop().unwrap();
            p1_stacks[to].push(grabbed);
        }

        // Part 2 solution
        let mut grabbed = Vec::new();
        for _ in 0..count {
            grabbed.push(p2_stacks[from].pop().unwrap());
        }
        grabbed.iter().rev().for_each(|c| p2_stacks[to].push(*c));
    });


    print!("Part 1: ");
    for stack in p1_stacks {
        print!("{}", stack.last().unwrap())
    }

    print!("\nPart 2: ");
    for stack in p2_stacks {
        print!("{}", stack.last().unwrap())
    }
}