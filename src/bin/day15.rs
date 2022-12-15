fn main() {
    let input = aoc2022::read_input_for_day(15);

    // Positions occupied by sensors and beacons
    let mut occupied = std::collections::HashSet::new();

    // List of (sensor, closest beacon) locations
    let locations: Vec<((i64, i64), (i64, i64))> = input.lines().map(|line| {
        let (sensor, beacon) = line.split_once(": closest beacon is at ").unwrap();
        
        let (sensor_x, sensor_y) = sensor.split_once(", ").unwrap();
        let sensor_x = sensor_x.split_once("x=").unwrap().1.parse().unwrap();
        let sensor_y = sensor_y.split_once("y=").unwrap().1.parse().unwrap();
        
        let (beacon_x, beacon_y) = beacon.split_once(", ").unwrap();
        let beacon_x = beacon_x.split_once("x=").unwrap().1.parse().unwrap();
        let beacon_y = beacon_y.split_once("y=").unwrap().1.parse().unwrap();

        occupied.insert((sensor_x, sensor_y));
        occupied.insert((beacon_x, beacon_y));

        ((sensor_x, sensor_y), (beacon_x, beacon_y))
    })
    .collect();

    let (largest_x_distance, x_min, x_max) = locations.iter()
        .fold((0, 0, 0), |(largest_x_distance, x_min, x_max), (sensor, beacon)| {
            (
                (sensor.0 - beacon.0).abs().max(largest_x_distance),
                sensor.0.min(x_min),
                sensor.0.max(x_max),
            )
        });

    let mut part1 = 0;
    for x in (x_min - largest_x_distance)..=(x_max + largest_x_distance) {
        for (sensor, beacon) in &locations {
            // Manhattan distance
            let range = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

            // Check whether (x, 2000000) is in range of the sensor
            if (x - sensor.0).abs() + (2000000 - sensor.1).abs() > range {
                continue;
            } else {
                if occupied.get(&(x, 2000000)).is_none() {
                    part1 += 1;
                }
                break;
            }
        }
    }

    let mut part2 = 0;
    // Walk the perimeter of the sensor's range, checking whether that point is within another beacon's range
    // If it is not, the solution is found
    'outer: for (sensor, beacon) in &locations {
        let range = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        let (x_min, x_max) = (sensor.0 - range, sensor.0 + range);
        let (y_min, y_max) = (sensor.1 - range, sensor.1 + range);

        // Too far away to matter
        if x_min > 4000000 || y_min > 4000000 || x_max < 0 || y_max < 0 {
            continue;
        }

        // Walk clockwise, starting at the left side
        let (mut x, mut y) = (x_min - 1, sensor.1);
        for (dx, dy) in [(1, 1), (1, -1), (-1, -1), (-1, 1)] {
            loop {
                match (dx, dy) {
                    // up and right
                    ( 1,  1) => if y == y_max + 1 { break }
                    // down and right
                    ( 1, -1) => if x == x_max + 1 { break }
                    // down and left
                    (-1, -1) => if y == y_min - 1 { break }
                    // up and left
                    (-1,  1) => if x == x_min - 1 { break }

                    _ => unreachable!(),
                }

                // Out of bounds -> skip comparisons
                if x > 4000000 || y > 4000000 || x < 0 || y < 0 {
                    x += dx;
                    y += dy;
                    continue;
                }

                let mut found = false;
                
                for (sensor2, beacon2) in &locations {
                    if sensor == sensor2 {
                        continue;
                    }

                    let range = (sensor2.0 - beacon2.0).abs() + (sensor2.1 - beacon2.1).abs();
                    
                    // In range of another beacon -> cannot be the solution
                    if (x - sensor2.0).abs() + (y - sensor2.1).abs() <= range {
                        found = false;
                        break;
                    } else {
                        found = true;
                    }
                }

                if found {
                    part2 = x * 4000000 + y;
                    break 'outer;
                }

                x += dx;
                y += dy;
            }
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}