fn main() {
    let input = aoc2022::read_input_for_day(14);

    let occupied: std::collections::HashSet<(usize, usize)> = input
        .lines()
        .map(|line| {
            line
                .split(" -> ")
                .map(|coord_string| {
                    coord_string
                        .split_once(',')
                        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                        .unwrap()
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<_>>() // Input is parsed at this point
        .into_iter()
        .flat_map(|list| {
            list
                .windows(2)
                .flat_map(|window| {
                    // List of points between the two points in the window
                    let mut occupied_coords = Vec::new();

                    if window[0].0 != window[1].0 {
                        for x in window[0].0.min(window[1].0)..=window[0].0.max(window[1].0) {
                            occupied_coords.push((x, window[0].1));
                        }
                    } else {
                        for y in window[0].1.min(window[1].1)..=window[0].1.max(window[1].1) {
                            occupied_coords.push((window[0].0, y));
                        }
                    }
                    
                    occupied_coords
                })
                .collect::<Vec<_>>()
        })
        .collect();

    // Not the optimal time to check, but simple
    let lowest = *occupied.iter().map(|(_, y)| y).max().unwrap();

    let mut part1 = 0;
    let mut part2 = 0;

    for part in 1..=2 {
        let mut occupied = occupied.clone();

        'outer: loop {
            let mut sand = (500, 0);
            loop {
                let mut below      = occupied.contains(&(sand.0    , sand.1 + 1));
                let mut left_diag  = occupied.contains(&(sand.0 - 1, sand.1 + 1));
                let mut right_diag = occupied.contains(&(sand.0 + 1, sand.1 + 1));

                if part == 2 {
                    below      |= sand.1 + 1 == lowest + 2;
                    left_diag  |= sand.1 + 1 == lowest + 2;
                    right_diag |= sand.1 + 1 == lowest + 2;
                }
                
                match (below, left_diag, right_diag) {
                    // Sand keeps falling down
                    (false, _, _) => {
                        sand.1 += 1;
                    }
                    // Sand settles
                    (true, true, true) => {
                        if part == 1 {
                            part1 += 1;
                        } else {
                            part2 += 1;
                            if sand == (500, 0) {
                                break 'outer;
                            }
                        }
                        
                        occupied.insert(sand);
                        break;
                    }
                    // Sand prefers to fall left
                    (true, false, _) => {
                        sand.0 -= 1;
                        sand.1 += 1;
                    }
                    // Sand falls right
                    (true, true, false) => {
                        sand.0 += 1;
                        sand.1 += 1;
                    }
                }

                // Fell into the void
                if part == 1 && sand.1 > lowest {
                    break 'outer;
                }
            }
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}