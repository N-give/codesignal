fn main() {
    println!(
        "sudo can be solved: {}",
        sudoku2(vec![
            vec!['.', '.', '.', '1', '4', '.', '.', '2', '.'],
            vec!['.', '.', '6', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '1', '.', '.', '.', '.', '.', '.'],
            vec!['.', '6', '7', '.', '.', '.', '.', '.', '9'],
            vec!['.', '.', '.', '.', '.', '.', '8', '1', '.'],
            vec!['.', '3', '.', '.', '.', '.', '.', '.', '6'],
            vec!['.', '.', '.', '.', '.', '7', '.', '.', '.'],
            vec!['.', '.', '.', '5', '.', '.', '.', '7', '.']
        ])
    );
}

fn sudoku2(grid: Vec<Vec<char>>) -> bool {
    let mut grid: Vec<Vec<Option<u32>>> = grid
        .iter()
        .map(|arr| arr.iter().map(|elem| elem.to_digit(10)).collect())
        .collect();

    sudoku3(&mut grid)
}

fn sudoku3(grid: &mut [Vec<Option<u32>>]) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if grid[i][j] == None {
                let valid_moves = get_valid_moves(&grid, i, j);
                valid_moves.iter().any(|mv| {
                    grid[i][j] = Some(*mv);
                    if is_solved(&grid) {
                        true
                    } else {
                        sudoku3(grid)
                    }
                });
            }
        }
    }
    false
}

fn get_valid_moves(grid: &[Vec<Option<u32>>], i: usize, j: usize) -> Vec<u32> {
    let mut impossible: Vec<u32> = vec![];
    grid.iter().for_each(|arr| {
        if let Some(e) = arr[j] {
            if !impossible.contains(&e) {
                impossible.push(e);
            }
        }
    });
    grid[i].iter().for_each(|elem| {
        if let Some(e) = elem {
            if !impossible.contains(e) {
                impossible.push(*e);
            }
        }
    });
    match i {
        0..=2 => {
            unimplemented!();
        }
        3..=5 => {
            unimplemented!();
        }
        6..=8 => {
            unimplemented!();
        }
        _ => unimplemented!(),
    };

    let mut possible: Vec<u32> = Vec::with_capacity(10);

    for i in 1..=9 {
        if !impossible.contains(&i) {
            possible.push(i);
        }
    }
    possible
}

fn is_solved(grid: &[Vec<Option<u32>>]) -> bool {
    true
}
