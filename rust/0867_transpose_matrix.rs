impl Solution {
  pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (row_count, col_count) = (matrix.len(), matrix[0].len());
    let mut transposed_matrix: Vec<Vec<i32>> = vec![vec![0; row_count]; col_count];

    for r in 0..row_count {
      for c in 0..col_count {
        transposed_matrix[c][r] = matrix[r][c]
      }
    }

    transposed_matrix
  }
}
