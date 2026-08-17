pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for (row_idx, row) in input.iter().enumerate() {
        if let Some(&current_row_max) = row.iter().max() {
            for (col_idx, &col) in row.iter().enumerate() {
                if col == current_row_max && input.iter().all(|r| r[col_idx] >= col) {
                    res.push((row_idx, col_idx));
                }
            }
        }
    }
    res
}
