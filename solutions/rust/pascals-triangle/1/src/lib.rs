pub struct PascalsTriangle {
    triangle: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle: Vec<Vec<u32>> = vec![vec![]; row_count as usize];
        if let Some(row) = triangle.get_mut(0) {
            row.push(1);
        }
        for i in 1..row_count {
            for j in 0..i + 1 {
                let left = j
                    .checked_sub(1)
                    .and_then(|idx| triangle[(i - 1) as usize].get(idx as usize))
                    .unwrap_or(&0);
                let right = triangle[(i - 1) as usize].get(j as usize).unwrap_or(&0);
                let sum = *left + *right;
                triangle[i as usize].push(sum);
            }
        }
        Self { triangle }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.clone()
    }
}
