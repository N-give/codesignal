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

fn sudoku2(_grid: Vec<Vec<char>>) -> bool {
    // let mut mgrid = grid
    //     .iter()
    //     .map(|arr| arr.iter().map(|val| val.to_digit(10)).collect())
    //     .collect();
    true
}
