struct Model {
    sums: Vec<u64>,
    current_sum: u64,
}

impl Model {
    fn new() -> Self {
        Self {
            sums: Vec::new(),
            current_sum: 0,
        }
    }

    fn add(&mut self, value: u64) {
        self.current_sum += value;
    }

    fn push_sum(&mut self) {
        self.sums.push(self.current_sum);
        self.current_sum = 0;
    }
}


fn main() {
    let input = aoc2022::read_input_for_day(1);

    let mut model = input.lines().fold(Model::new(), |mut model, line| {
        // Newline --> separator
        if line.is_empty() {
            model.push_sum();
        } else {
            model.add(line.parse::<u64>().unwrap());
        }

        model
    });
    
    // Sort from most to least
    model.sums.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let total: u64 = model.sums.iter().take(3).sum();

    println!("\n1st: {}\n2nd: {}\n3rd: {}\n\nTotal: {}", model.sums[0], model.sums[1], model.sums[2], total);
}