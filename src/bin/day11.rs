enum Operation {
    Add(u64),
    Mul(u64),
    Square,
    Double,
}

struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisibility_test: u64,
    true_target: usize,
    false_target: usize,
}

impl Monkey {
    fn parse(string: &str) -> Self {
        let mut lines = string.lines().skip(1);
        
        let items = lines.next().unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();

        let operation = lines.next().unwrap()
            .split_once("old ")
            .unwrap()
            .1
            .split_once(' ')
            .map(|(op, value)| {
                match op {
                    "+" => {
                        if value == "old" {
                            Operation::Double
                        } else {
                            Operation::Add(value.parse().unwrap())
                        }
                    }

                    "*" => {
                        if value == "old" {
                            Operation::Square
                        } else {    
                            Operation::Mul(value.parse().unwrap())
                        }
                    }
                    
                    _ => panic!("Invalid operator"),
                }
            })
            .unwrap();

        let divisibility_test = lines.next().unwrap()
            .split_once("by ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        let true_target = lines.next().unwrap()
            .split(' ')
            .last()
            .map(|s| s.parse().unwrap())
            .unwrap();

        let false_target = lines.next().unwrap()
            .split(' ')
            .last()
            .map(|s| s.parse().unwrap())
            .unwrap();

        Self {
            items,
            operation,
            divisibility_test,
            true_target,
            false_target,
        }
    }
}

struct Game {
    monkeys: Vec<Monkey>,
    // How many items each monkey has inspected
    inspections: Vec<u64>,
    is_worry_level_divided: bool,
    lcm: u64,
}

impl Game {
    fn new(input: &str, is_worry_level_divided: bool) -> Self {
        let monkeys: Vec<_> = input
            .replace('\r', "") // Can't simply split on '\n\n' on Windows...
            .split("\n\n")
            .map(Monkey::parse)
            .collect();
        
        let inspections = (0..monkeys.len()).map(|_| 0).collect();

        let lcm = monkeys.iter().fold(1, |lcm, monkey| {
            // Get GCD first
            let mut a = lcm;
            let mut b = monkey.divisibility_test;

            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }

            let gcd = a;

            // Then calc LCM
            lcm * monkey.divisibility_test / gcd
        });

        Self {
            monkeys,
            inspections,
            is_worry_level_divided,
            lcm,
        }
    }

    fn play_round(&mut self) {
        // NOTE: Guarenteed safe because a monkey cannot throw to itself (no in-place mutation) -- can add an assertion for proof
        let monkeys_ptr = unsafe {
            #[allow(mutable_transmutes)]
            std::mem::transmute::<_, &mut Vec<Monkey>>(&self.monkeys)
        };

        for (i, monkey) in self.monkeys.iter_mut().enumerate() {
            for mut worry in monkey.items.drain(..) {
                self.inspections[i] += 1;
                
                match monkey.operation {
                    Operation::Add(n) => worry += n,
                    Operation::Mul(n) => worry *= n,
                    Operation::Double => worry += worry,
                    Operation::Square => worry *= worry,
                }

                if self.is_worry_level_divided {
                    worry /= 3;
                } else {
                    // NOTE:
                    // Given: set of numbers, S
                    // Goal:  test divisibilty of S while minimizing the size of worry, W
                    //
                    // Solution: 1. Calculate L = LCM(S)
                    //           2. Divisibilty of W foreach S is maintained by W % L
                    worry %= self.lcm;
                }

                if worry % monkey.divisibility_test == 0 {
                    monkeys_ptr[monkey.true_target].items.push(worry);
                } else {
                    monkeys_ptr[monkey.false_target].items.push(worry);
                }
            }
        }
    }

    // Returns top two monkeys' inspections
    fn top_monkeys(&self) -> (u64, u64) {
        let mut top_monkeys = (0, 0);

        self.inspections.iter().for_each(|&count| {
            if count > top_monkeys.0 {
                top_monkeys = (count, top_monkeys.0);
            } else if count > top_monkeys.1 {
                top_monkeys.1 = count;
            }
        });

        top_monkeys
    }
}

fn main() {
    let input = aoc2022::read_input_for_day(11);

    let mut game1 = Game::new(&input, true);
    (0..20).for_each(|_| game1.play_round());
    let part1 = game1.top_monkeys();

    let mut game2 = Game::new(&input, false);
    (0..10000).for_each(|_| game2.play_round());
    let part2 = game2.top_monkeys();
    
    println!("Part 1: {:?}", part1.0 * part1.1);
    println!("Part 2: {:?}", part2.0 * part2.1);
}