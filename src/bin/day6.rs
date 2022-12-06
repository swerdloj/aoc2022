fn main() {
    let input = aoc2022::read_input_for_day(6);

    // `array_chunks()` still unstable...

    for part in 1..=2 {
        let count = if part == 1 { 4 } else { 14 };
        
        let answer = input
            .chars()
            .collect::<Vec<_>>()
            .windows(count)
            .enumerate()
            .take_while(|(_pos, string)| {
                string.iter().collect::<std::collections::HashSet<_>>().len() != count
            })
            .last()
            .unwrap()
            .0 + 1 + count; // want end position (+count) and not zero-indexed (+1)
        
        println!("Part {}: {} ('{}')", part, answer, input.get(answer-count..answer).unwrap());
    }
}