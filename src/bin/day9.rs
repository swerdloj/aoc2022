fn main() {
    let input = aoc2022::read_input_for_day(9);

    for part in 1..=2 {
        let mut seen = std::collections::HashSet::new();
        
        // Rope is either 2 or 10 knots, all starting at (0, 0)
        let mut rope: Vec<_> = (0..if part == 1 {2} else {10}).map(|_| (0, 0)).collect();
        
        for line in input.lines() {
            let (direction, steps) = line.split_once(' ').unwrap();
            
            let delta = match direction {
                "L" => (-1,  0),
                "R" => ( 1,  0),
                "U" => ( 0,  1),
                "D" => ( 0, -1),
                
                _ => panic!("Invalid direction"),
            };
            
            for _ in 0..steps.parse().unwrap() {
                let mut previous = (0, 0);

                for (knot, position) in rope.iter_mut().enumerate() {
                    // Move the head
                    if knot == 0 {
                        position.0 += delta.0;
                        position.1 += delta.1;
                        
                        previous = *position;
                        continue;
                    }
                    
                    // Compare each knot to the one before, then move accordingly
                    let distance = (previous.0 - position.0, previous.1 - position.1);
                    
                    match distance {
                        // right or left
                        (2, 0) | (-2,  0) => position.0 += distance.0 / 2,

                        // up or down
                        (0, 2) | ( 0, -2) => position.1 += distance.1 / 2,
                        
                        // up-right
                        (2, 1) | (1, 2) | (2, 2) => {
                            position.0 += 1;
                            position.1 += 1;
                        }
                        
                        // down-right
                        (2, -1) | (1, -2) | (2, -2) => {
                            position.0 += 1;
                            position.1 -= 1;
                        }
                        
                        // up-left
                        (-2, 1) | (-1, 2) | (-2, 2) => {
                            position.0 -= 1;
                            position.1 += 1;
                        }
                        
                        // down-left
                        (-2, -1) | (-1, -2) | (-2, -2) => {
                            position.0 -= 1;
                            position.1 -= 1;
                        }
                        
                        // Knot does not move
                        _ => {}
                    }

                    previous = *position;
                }
                // Register the tail's location after moving the whole rope one step
                seen.insert(rope[rope.len() - 1]);
            }
        }

        println!("Part {part}: {}", seen.len());
    }
}