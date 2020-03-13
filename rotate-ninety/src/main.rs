fn main() {
    let a = rotate(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    a.iter().for_each(|arr| println!("{:?}", arr));
}

fn rotate(mut a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = a.len();
    for i in 0..(n / 2) {
        for j in 0..(n - i - 1) {
            let tmp: i32 = a[i][j];
            a[i][j] = a[n - 1 - j][i];
            a[n - 1 - j][i] = a[n - 1 - i][n - 1 - j];
            a[n - 1 - i][n - 1 - j] = a[j][n - 1 - i];
            a[j][n - 1 - i] = tmp;
        }
    }
    a
}

// [ [(0,0), (0,1), (0,2)]
// , [(1,0), (1,1), (1,2)]
// , [(2,0), (2,1), (2,2)]
// ]
