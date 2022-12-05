use std::collections::HashSet;

fn main() {
    let input = aoc2022::read_input_for_day(3);

    let part1: u64 = input.lines().fold(0, |mut sum, line| {
        let (compartment1, compartment2) = line.split_at(line.len() / 2);

        let items: HashSet<_> = compartment1.chars().collect();

        for c in compartment2.chars() {
            if items.contains(&c) {
                sum += priority(c);
                break
            }
        }

        sum
    });

    // Unstable...
    // input.lines().array_chunks::<3>().fold(...)

    let part2: u64 = input.lines().collect::<Vec<_>>().windows(3).step_by(3).fold(0, |mut sum, bags| {
        let bag1_items: HashSet<_> = bags[0].chars().collect();

        let shared = bags[1].chars().fold(HashSet::new(), |mut shared, c| {
            if bag1_items.contains(&c) {
                shared.insert(c);
            }

            shared
        });

        for c in bags[2].chars() {
            if shared.contains(&c) {
                sum += priority(c);
                break
            }
        }
        
        sum
    });

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn priority(c: char) -> u64 {
    const LOWERCASE_OFFSET: u64 = 'a' as u64;
    const UPPERCASE_OFFSET: u64 = 'A' as u64;

    if c.is_lowercase() {
        c as u64 - LOWERCASE_OFFSET + 1
    } else {
        c as u64 - UPPERCASE_OFFSET + 27
    }
}