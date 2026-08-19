pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; size as usize]; size as usize];
    let total = size * size;
    let mut row = 0i32;
    let mut col = 0i32;
    // right - 0, 1
    // down - 1, 0
    // left - 0, -1
    // up - -1, 0
    let mut row_increment = 0;
    let mut col_increment = 1;
    for i in 1..=total {
        matrix[row as usize][col as usize] = i;

        let potential_next_row = row + row_increment;
        let potential_next_col = col + col_increment;
        if not_valid_cell(potential_next_row, potential_next_col, size, &matrix) {
            let tmp = row_increment;
            row_increment = col_increment;
            col_increment = -tmp;
        }

        row += row_increment;
        col += col_increment;
    }
    matrix
}

fn not_valid_cell(row: i32, col: i32, border: u32, matrix: &[Vec<u32>]) -> bool {
    row < 0
        || row as u32 >= border
        || col < 0
        || col as u32 >= border
        || matrix[row as usize][col as usize] != 0
}
