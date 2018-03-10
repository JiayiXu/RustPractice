fn main() {
    let plane = vec![vec![1.1],vec![1.2],vec![1.3]];
    print!("{}", find_best(plane));
}

fn find_best(plane: Vec<Vec<f32>>) -> f32{
    let num_rows = plane.len();
    let num_cols = plane[0].len();

    let mut array = vec![vec![0.0f32; num_cols]; num_rows];

    for i in 0..num_rows {
        for j in 0..num_cols {
            array[i][j] = plane[i][j];
            if i-1 > 0 && array[i-1][j] < array[i][j] {
                array[i][j] = array[i-1][j];
            }

            if i-1 > 0 && j-1 > 0 && array[i-1][j-1] < array[i][j] {
                array[i][j] = array[i-1][j-1];
            }
        }
    }

    let mut max = 0.0;
    for i in 0..num_rows {
        if array[i][num_cols-1] > max {
            max = array[i][num_cols-1];
        }
    }

    3.2
}