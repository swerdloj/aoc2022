fn main() {
    let input = aoc2022::read_input_for_day(4);

    let part1 = input.lines().filter(|line| {
        let numbers: Vec<u64> = line
            .replace(',', "-")
            .split('-')
            .map(|string| string.parse().unwrap())
            .collect();

        // Cases (start/end can aslo be equal):
        //    3-7, 4-6 | 4-6, 3-7
           numbers[0] >= numbers[2] && numbers[1] <= numbers[3]
        || numbers[0] <= numbers[2] && numbers[1] >= numbers[3]
    })
    .count();


    let part2 = input.lines().filter(|line| {
        let numbers: Vec<u64> = line
            .replace(',', "-")
            .split('-')
            .map(|string| string.parse().unwrap())
            .collect();

        // Front overlap
           numbers[0] <= numbers[2] && numbers[1] >= numbers[2]
        || numbers[0] <= numbers[3] && numbers[1] >= numbers[3]
        
        // End overlap
        || numbers[0] >= numbers[2] && numbers[1] <= numbers[3]
        
        // Redundant case (ranges always count up)
        // || numbers[0] >= numbers[3] && numbers[1] <= numbers[3]
    })
    .count();

    
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}