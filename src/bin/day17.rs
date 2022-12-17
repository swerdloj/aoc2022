use std::collections::HashSet;

fn get_all_positions(rock: u64, position: &(u64, u64)) -> Vec<(u64, u64)> {
    match rock {
        // -
        0 => vec![(position.0, position.1), (position.0+1, position.1), (position.0+2, position.1), (position.0+3, position.1)],
        // +
        1 => vec![(position.0, position.1), (position.0+1, position.1), (position.0+2, position.1), (position.0+1, position.1+1), (position.0+1, position.1-1)],
        // _|
        2 => vec![(position.0, position.1), (position.0+1, position.1), (position.0+2, position.1), (position.0+2, position.1+1), (position.0+2, position.1+2)],
        // |
        3 => vec![(position.0, position.1), (position.0, position.1+1), (position.0, position.1+2), (position.0, position.1+3)],
        // []
        4 => vec![(position.0, position.1), (position.0+1, position.1), (position.0, position.1+1), (position.0+1, position.1+1)],
        
        _ => unreachable!()
    }
}

fn intersects_map(rock: u64, position: &(u64, u64), map: &HashSet<(u64, u64)>) -> bool {
    // Check the left & right edges
    let edge = match rock {
        // -
        0 => position.0 == 0 || position.0+3 == 8 ,
        // +
        1 => position.0 == 0 || position.0+2 == 8,
        // _|
        2 => position.0 == 0 || position.0+2 == 8,
        // |
        3 => position.0 == 0 || position.0 == 8,
        // []
        4 => position.0 == 0 || position.0 + 1 == 8,
        
        _ => unreachable!()
    };

    edge || get_all_positions(rock, position).iter().any(|p| map.contains(p))
}

fn intersects_below(rock: u64, position: &(u64, u64), map: &HashSet<(u64, u64)>) -> bool {
    // Check the bottom edges against the map and the floor
    match rock {
        // -
        0 => position.1 == 0 || map.contains(&(position.0, position.1)) || map.contains(&(position.0+1, position.1)) || map.contains(&(position.0+2, position.1)) || map.contains(&(position.0+3, position.1)),
        // +
        1 => position.1-1 == 0 || map.contains(&(position.0, position.1)) || map.contains(&(position.0+1, position.1-1)) || map.contains(&(position.0+2, position.1)),
        // _|
        2 => position.1 == 0 || map.contains(&(position.0, position.1)) || map.contains(&(position.0+1, position.1)) || map.contains(&(position.0+2, position.1)),
        // |
        3 => position.1 == 0 || map.contains(&(position.0, position.1)),
        // []
        4 => position.1 == 0 || map.contains(&(position.0, position.1)) || map.contains(&(position.0+1, position.1)),
        
        _ => unreachable!()
    }
}

fn main() {
    let input = aoc2022::read_input_for_day(17);
    
    // Test input
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    let mut directions = input.chars().cycle();

    let mut pushes_left = || {
        directions.next().unwrap() == '<'
    };

    // Walls at x=0, x=8. Floor at y=0.
    let mut map = std::collections::HashSet::new();

    let mut rock = 0;
    let mut height = 0;

    let mut part1 = 0;

    for i in 0..1_000_000_000_000u64 {
        // Left-most edge starts at x=3 with y=height + 4 (except for "+" rock)
        // `position` means leftmost for "-", leftmost for "+", bottom-left for "_|", bottom for "|", bottom-left for "[]"
        let mut position = if rock != 1 {
            // -, _|, |, []
            (3, height + 4)
        } else {
            // +
            (3, height + 5)
        };

        // Push, check if possible. Fall, check if possible. Repeat until settled.
        loop {
            let mut old = position;
            position.0 = if pushes_left() { position.0 - 1 } else { position.0 + 1 };

            if intersects_map(rock, &position, &map) {
                position = old;
            }
            
            old = position;
            position.1 -= 1;

            if intersects_below(rock, &position, &map) {
                position = old;
                break;
            }
        }

        for piece in get_all_positions(rock, &position) {
            height = height.max(piece.1);
            map.insert(piece);
        }

        if i == 2021 {
            part1 = height;
        }

        rock = (rock + 1) % 5;
    }

    // Print the map
    // for y in (1..=height+1).rev() {
    //     for x in 1..=7 {
    //         print!("{}", if map.contains(&(x, y)) { '#' } else { '.' });
    //     }
    //     println!();
    // }

    println!("Part1: {part1}");
    println!("Part2: {height}");
}