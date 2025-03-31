pub fn find(grid: Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for (row_index, row) in grid.iter().enumerate() {
        println!("{:?}", row);
        // let row = [row.clone(), [' '].to_vec()].concat();
        for (col_index, col) in row.iter().enumerate() {
            let col = *col;
            let row_forward_search = if col_index + 3 < row.len() {
                col == 'X'
                    && row[col_index + 1] == 'M'
                    && row[col_index + 2] == 'A'
                    && row[col_index + 3] == 'S'
            } else {
                false
            };
            if row_forward_search {
                count += 1;
            }

            let row_backward_search = if col_index > 2 {
                col == 'X'
                    && row[col_index - 1] == 'M'
                    && row[col_index - 2] == 'A'
                    && row[col_index - 3] == 'S'
            } else {
                false
            };
            if row_backward_search {
                count += 1;
            }

            let col_forward_search = if row_index + 3 < grid.len() {
                col == 'X'
                    && grid[row_index + 1][col_index] == 'M'
                    && grid[row_index + 2][col_index] == 'A'
                    && grid[row_index + 3][col_index] == 'S'
            } else {
                false
            };
            if col_forward_search {
                count += 1;
            }
            let col_backward_search = if row_index > 2 {
                col == 'X'
                    && grid[row_index - 1][col_index] == 'M'
                    && grid[row_index - 2][col_index] == 'A'
                    && grid[row_index - 3][col_index] == 'S'
            } else {
                false
            };
            if col_backward_search {
                count += 1;
            }
            let diagonal_forward_right = if row_index + 3 < grid.len() && col_index + 3 < row.len()
            {
                col == 'X'
                    && grid[row_index + 1][col_index + 1] == 'M'
                    && grid[row_index + 2][col_index + 2] == 'A'
                    && grid[row_index + 3][col_index + 3] == 'S'
            } else {
                false
            };
            if diagonal_forward_right {
                count += 1;
            }
            let diagonal_forward_left = if row_index + 3 <= grid.len() && col_index >= 3 {
                col == 'X'
                    && grid[row_index + 1][col_index - 1] == 'M'
                    && grid[row_index + 2][col_index - 2] == 'A'
                    && grid[row_index + 3][col_index - 3] == 'S'
            } else {
                false
            };
            if diagonal_forward_left {
                count += 1;
            }
            let diagonal_backward_right = if row_index >= 3 && col_index >= 3 {
                col == 'X'
                    && grid[row_index - 1][col_index - 1] == 'M'
                    && grid[row_index - 2][col_index - 2] == 'A'
                    && grid[row_index - 3][col_index - 3] == 'S'
            } else {
                false
            };
            if diagonal_backward_right {
                count += 1;
            }

            let diagonal_backward_left = if row_index >= 3 && col_index + 3 < row.len() {
                col == 'X'
                    && grid[row_index - 1][col_index + 1] == 'M'
                    && grid[row_index - 2][col_index + 2] == 'A'
                    && grid[row_index - 3][col_index + 3] == 'S'
            } else {
                false
            };
            if diagonal_backward_left {
                count += 1;
            }
        }
    }
    count
}
