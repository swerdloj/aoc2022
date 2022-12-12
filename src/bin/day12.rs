#[derive(Clone)]
struct AStarNode {
    pos: (usize, usize),
    height: usize,
    // The "parent" node (treated as an index)
    path_previous: (usize, usize),
    // Total cost from start (at the moment)
    g_cost: usize,
    // Heuristic cost (distance to target in this case)
    h_cost: usize,
}

impl AStarNode {
    fn new(pos: (usize, usize), height: usize, heuristic: usize) -> Self {
        Self {
            pos,
            height,
            path_previous: pos,
            g_cost: 0,
            h_cost: heuristic,
        }
    }

    // Current guess for total cost
    fn f_cost(&self) -> usize {
        self.g_cost + self.h_cost
    }
}

fn heuristic(pos: (usize, usize), target: (usize, usize)) -> usize {
    (pos.0 as isize - target.0 as isize).abs() as usize + (pos.1 as isize - target.1 as isize).abs() as usize
}

fn main() {
    let input = aoc2022::read_input_for_day(12);

    let mut start = (0, 0);
    let mut potential_starts = Vec::new();
    let mut target = (0, 0);

    let map: Vec<Vec<_>> = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            match c {
                'S' => {
                    start = (x, y);
                    potential_starts.push((x, y));
                    'a' as usize
                }

                'a' => {
                    potential_starts.push((x, y));
                    c as usize
                }

                'E' => {
                    target = (x, y);
                    'z' as usize
                }

                _ => c as usize,
            }
        })
        .collect::<Vec<_>>().into_iter() // This line is for convenience (borrow check on `target`)
        .enumerate()
        .map(|(x, height)| {
            AStarNode::new((x, y), height, heuristic((x, y), target))
        })
        .collect()
    })
    .collect();

    let part1 = a_star(map.clone(), start, target).unwrap().len() - 1;

    // Far from optimal, but simple (and very fast in release mode)
    let part2 = potential_starts
        .iter()
        .filter_map(|s| a_star(map.clone(), *s, target))
        .min_by_key(|path| path.len())
        .map(|path| path.len() - 1)
        .unwrap();
    
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn backtrace_path(map: Vec<Vec<AStarNode>>, start: (usize, usize), target: (usize, usize)) -> Vec<(usize, usize)> {
    let mut path = Vec::new();
    let mut current = target;
    while current != start {
        path.push(current);
        current = map[current.1][current.0].path_previous;
    }
    path.push(start);
    path.reverse();
    path
}

fn a_star(mut map: Vec<Vec<AStarNode>>, start: (usize, usize), target: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let is_move_valid = |to, from| {
        // Can go down any amount, but only up by one
        to <= from + 1
    };

    // NOTE: Ideally, `open` would be a MinHeap (would remove all the uses of `open.iter()..`)
    let mut open = vec![map[start.1][start.0].clone()];    
    let mut closed = std::collections::HashSet::new();

    while !open.is_empty() {
        // Get index of node with lowest f_cost (to look at next)
        let next_index = open.iter().enumerate().fold((0, usize::MAX), |(next, min), (index, node)| {
            if node.f_cost() < min {
                (index, node.f_cost())
            } else {
                (next, min)
            }
        }).0;

        let current = open.remove(next_index);
        closed.insert(current.pos);
        
        if current.pos == target {
            return Some(backtrace_path(map, start, target));
        }
        
        let mut neighbors = Vec::with_capacity(4);
        // [Left, Right, Up, Down]
        for delta in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_pos = (current.pos.0 as isize + delta.0, current.pos.1 as isize + delta.1);
            if new_pos.0 >= 0 && new_pos.0 < map[0].len() as isize && new_pos.1 >= 0 && new_pos.1 < map.len() as isize {
                let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
                if is_move_valid(map[new_pos.1][new_pos.0].height, current.height) {
                    neighbors.push(new_pos);
                }
            }
        }

        for pos in neighbors {
            if closed.contains(&pos) {
                continue;
            }
            
            // Distance from current to neighbor is always 1
            let new_g_cost = current.g_cost + 1; 
            let neighbor_node = &mut map[pos.1][pos.0];

            // Found better path to the neighbor
            if new_g_cost < neighbor_node.g_cost || !open.iter().any(|n| neighbor_node.pos == n.pos) {
                neighbor_node.g_cost = new_g_cost;
                neighbor_node.path_previous = current.pos;

                if !open.iter().any(|n| neighbor_node.pos == n.pos) {
                    open.push(neighbor_node.clone());
                }
            }
        }
    }

    None
}