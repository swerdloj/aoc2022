use std::collections::{HashMap, HashSet};


struct Valve {
    name: String,
    flow_rate: u64,
    connections: HashSet<String>,
}

impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool { self.name == other.name }
}
impl Eq for Valve {}
impl std::hash::Hash for Valve {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.name.hash(state); }
}


struct Model {
    current_location: String,
    released_pressure: u64,
    minutes_remaining: u64,
    opened: HashSet<String>,
    valves: HashMap<String, Valve>,
}

impl Model {
    fn new(valves: HashMap<String, Valve>, start: String) -> Self {
        Self {
            current_location: start,
            released_pressure: 0,
            minutes_remaining: 30,
            opened: HashSet::new(),
            valves,
        }
    }

    fn run(&mut self) -> u64 {
        while self.minutes_remaining > 0 {
            println!("Minutes remaining: {}", self.minutes_remaining);
            self.update();
        }
        self.released_pressure
    }

    fn update(&mut self) {        
        self.minutes_remaining -= 1;
        for open in &self.opened {
            self.released_pressure += self.valves.get(open).unwrap().flow_rate;
        }

        if self.minutes_remaining == 0 {
            // No move can help
            return;
        }

        // let choice = self.optimal_choice(&self.current_location, self.minutes_remaining, &mut 0, 1, &mut HashSet::new());
        let choice = self.optimal_choice();

        println!("Chose {}", choice);

        if *choice == self.current_location {
            self.opened.insert(choice.to_string());
        } else if choice != "" {
            self.current_location = choice.to_string();
        }
    }

    fn optimal_choice(&self) -> &String {
        // TODO:
        todo!()
    }

    // fn optimal_choice(&self, current: &str, time: u64, score: &mut u64, depth: usize, opened: &mut HashSet<String>) -> String {
    //     let mut best_move = current.to_string();

    //     // Don't go in loops
    //     if depth >= self.valves.len() {
    //         return best_move;
    //     }

    //     // Nothing more can be done
    //     if time <= 1 {
    //         return best_move;
    //     }

    //     let checking = self.valves.get(current).unwrap();
    //     if !self.opened.contains(&checking.name) && !opened.contains(&checking.name) {
    //         let possible_score = (time - 1) * checking.flow_rate;
    //         if possible_score > *score {
    //             *score = possible_score;
    //             best_move = checking.name.clone();
    //         }
    //     }
        
    //     for connection in &checking.connections {
    //         let mut possible_score = score.clone();
    //         let choice = self.optimal_choice(connection, time - 1, &mut possible_score, depth + 1, &mut opened.clone());
            
    //         if choice == *connection {
    //             opened.insert(choice.to_string());
    //         }

    //         if possible_score > *score {
    //             *score = possible_score;
    //             best_move = connection.clone();
    //         }
    //     }

    //     best_move
    // }
}

fn main() {
    let input = aoc2022::read_input_for_day(16);

    let input = 
"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    let mut start = String::new();
    let mut valves = HashMap::new();

    input.lines().for_each(|line| {
        let mut name = String::new();
        let mut flow_rate = 0;
        let mut connections = HashSet::new();

        line
            .replace("Valve ", "")
            .replace(" has flow rate=", ", ")
            .replace("; tunnels lead to valves ", ", ")
            .replace("; tunnel leads to valve ", ", ")
            .split(", ")
            .enumerate()
            .for_each(|(i, data)| {
                match i {
                    0 => {
                        name = data.to_string();

                        if start == "" {
                            start = name.clone();
                        }
                    },
                    1 => flow_rate = data.parse().unwrap(),
                    _ => { connections.insert(data.to_string()); },
                }
            });

            let valve = Valve {
                name,
                flow_rate,
                connections,
            };

            valves.insert(valve.name.clone(), valve);
    });

    let mut model = Model::new(valves, start);
    let part1 = model.run();

    println!("Part 1: {}", part1);
}