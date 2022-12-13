#[derive(Clone, PartialEq)]
enum Data {
    List(Vec<Data>),
    Int(u64),
}

fn parse(string: &mut String) -> Vec<Data> {
    let mut data = Vec::new();

    let mut number_string = String::new();

    loop {
        if string.is_empty() {
            return data;
        }
        let c = string.remove(0);

        match c {
            // Recurse to parse the nested list
            '[' => data.push(Data::List(parse(string))),

            ',' => continue,
            
            ']' => return data,

            n if n.is_numeric() => {
                number_string.push(n);

                loop {
                    let next = string.remove(0);

                    if next.is_numeric() {
                        number_string.push(next);
                    } else {
                        // Not a digit -> put it back and stop
                        string.insert(0, next);
                        break;
                    }
                }

                data.push(Data::Int(number_string.parse().unwrap()));
                number_string.clear();
            }

            _ => panic!("Invalid character: {c}"),
        }
    }
}


#[derive(PartialEq)]
enum Case {
    Yes,
    Maybe,
    No,
}

fn is_properly_ordered(left: &Data, right: &Data) -> Case {
    match (left, right) {
        (Data::Int(l), Data::Int(r)) => 
            if l < r { Case::Yes } else if l > r { Case::No } else { Case::Maybe }

        // Convert the int to a list and compare
        (Data::List(_), Data::Int(r)) => 
            is_properly_ordered(left, &Data::List(vec![Data::Int(*r)])),
        
        (Data::Int(l), Data::List(_)) =>
            is_properly_ordered(&Data::List(vec![Data::Int(*l)]), right),
        
        // Compare recursively
        (Data::List(l), Data::List(r)) => {
            for i in 0.. {
                let left_data = l.get(i);
                let right_data = r.get(i);

                match (left_data, right_data) {
                    (Some(l), Some(r)) => {
                        let result = is_properly_ordered(l, r);
                        if result != Case::Maybe {
                            return result;
                        }
                    }

                    // Left ran out first
                    (None, Some(_)) => return Case::Yes,
                    // Right ran out first
                    (Some(_), None) => return Case::No,
                    // Both ran out at the same time
                    (None, None) => return Case::Maybe,
                }
            }
            unreachable!("The loop always executes");
        }
    }
}


fn main() {
    let input = aoc2022::read_input_for_day(13);

    let pairs: Vec<_> = input
        .replace('\r', "") // Windows
        .split("\n\n")
        .map(|pair| {
            pair
                .split_once('\n')
                .map(|pair| {
                    (parse(&mut pair.0.into()), parse(&mut pair.1.into()))
                })
                // FIXME: Outermost `Data` always ends up in an enclosing `Vec`
                .map(|mut pair| (pair.0.pop().unwrap(), pair.1.pop().unwrap()))
                .unwrap()
        })
        .collect();


    let part1 = pairs.iter().enumerate().fold( 0, |mut part1, (index, pair)|{
        if is_properly_ordered(&pair.0, &pair.1) == Case::Yes {
            part1 += index + 1;
        }
        part1
    });


    // Flatten the packets into a `Vec<Data>` and include the dividers
    let dividers = vec![
        Data::List(vec![Data::List(vec![Data::Int(2)])]),
        Data::List(vec![Data::List(vec![Data::Int(6)])]),
    ];
    let mut packets: Vec<_> = pairs.into_iter().flat_map(|pair| [pair.0, pair.1]).collect();
    packets.append(&mut dividers.clone());

    let num_packets = packets.len();
    
    // Need to start with one element to compare against
    let mut sorted_packets = vec![packets.pop().unwrap()];
    
    // Compare each unsorted packet to the already sorted ones, finding the optimal spot to sort into
    while sorted_packets.len() < num_packets {
        for i in 0..packets.len() {
            // Want to find the lowest slot that the packet fits into (like sorting an array)
            let mut best_slot = usize::MAX;
            for (slot, sorted) in sorted_packets.iter().enumerate() {
                if is_properly_ordered(&packets[i], sorted) == Case::Yes {
                    if slot > best_slot {
                        break
                    }
                    best_slot = slot;
                }
            }

            // Place at specific location
            if best_slot < usize::MAX {
                sorted_packets.insert(best_slot, packets.remove(i));
                break;
            }
            // Place on the bottom
            else if is_properly_ordered(&sorted_packets[sorted_packets.len()-1], &packets[i]) == Case::Yes {
                sorted_packets.push(packets.remove(i));
                break;
            }
        }
    }

    let part2 = sorted_packets.iter().enumerate().fold(1, |mut part2, (i, packet)| {
        if dividers.contains(packet) {
            part2 *= i + 1
        }
        part2
    });


    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}