pub fn part_2(grid: Vec<Vec<char>>) -> usize {
    let upper = grid.len() - 2;
    let mut count = 0;
    for i in 0..upper {
        for j in 0..upper {
            let col = grid[i][j];
            if col == 'M' {
                let is_x_mas_fr = grid[i + 1][j + 1] == 'A' && grid[i + 2][j + 2] == 'S';
                let is_x_mas_bk = grid[i][j + 2] == 'M' && grid[i + 2][j] == 'S';
                let is_x_mas_bl = grid[i][j + 2] == 'S' && grid[i + 2][j] == 'M';

                if is_x_mas_fr && (is_x_mas_bk || is_x_mas_bl) {
                    count += 1;
                }
            }
            if col == 'S' {
                let is_x_mas_fr = grid[i + 1][j + 1] == 'A' && grid[i + 2][j + 2] == 'M';
                let is_x_mas_bk = grid[i][j + 2] == 'M' && grid[i + 2][j] == 'S';
                let is_x_mas_bl = grid[i][j + 2] == 'S' && grid[i + 2][j] == 'M';

                if is_x_mas_fr && (is_x_mas_bk || is_x_mas_bl) {
                    count += 1;
                }
            }
        }
    }

    count
}
