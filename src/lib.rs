pub fn read_input_for_day(day: u32) -> String {
    std::fs::read_to_string(format!("./input/day{}.txt", day))
        .expect("No input file found")
}