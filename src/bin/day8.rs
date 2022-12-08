fn main() {
    let input = aoc2022::read_input_for_day(8);

    let trees: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars().map(|c| c.to_digit(10).unwrap()).collect()
        })
        .collect();

    let rows = trees.len();
    let cols = trees[0].len();

    let mut visible_from_outside = 0;
    let mut max_scenic_score = 0;

    for row in 0..rows {
        for col in 0..cols {
            let height = trees[row][col];

            // edge
            // NOTE: Scenic score for edge is always 0
            if row == 0 || col == 0 || row == rows - 1 || col == cols - 1 {
                visible_from_outside += 1;
                continue;
            }

            // NOTE: Don't need to subtract one from `row` and `col` in up & left because ranges are not inclusive
            let up    = visibilty(&trees, height, row    , 0   , col, false);                
            let down  = visibilty(&trees, height, row + 1, rows, col, false);
            let left  = visibilty(&trees, height, col    , 0   , row, true);
            let right = visibilty(&trees, height, col + 1, cols, row, true);

            if up.0 || down.0 || left.0 || right.0 {
                visible_from_outside += 1;
            }

            max_scenic_score = max_scenic_score.max(up.1 * down.1 * left.1 * right.1);
        }
    }

    println!("Part 1: {}", visible_from_outside);
    println!("Part 2: {}", max_scenic_score);
}

fn visibilty(trees: &Vec<Vec<u32>>, height: u32, from: usize, to: usize, static_pos: usize, is_static_row: bool) -> (bool, u32) {
    let mut score = 0;
    
    // FIXME: Want to return the iterators themselves rather than call `collect`, but their types do not match
    let range: Vec<_> = if from < to {(from..to).collect()} else {(to..from).rev().collect()};
    for pos in range {
        // The blocking tree counts as being visible for the scenic score (so always add one)
        score += 1;

        if trees[if is_static_row {static_pos} else {pos}][if is_static_row{pos} else {static_pos}] >= height {
            return (false, score);
        }
    }
    
    (true, score)
}