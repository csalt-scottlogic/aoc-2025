fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let rows = Vec::from_iter(content.lines().map(|line| fields_in_line(line)));
    let table = rotator(rows);
    let part_1_total: i64 = table.iter().fold(0 as i64, |total, r| total + sum_row(r));
    println!("{}", part_1_total);
    let grid = Vec::from_iter(content.lines().map(|line| Vec::from_iter(line.chars())));
    let part_2_total = do_puzzle_2(&grid);
    println!("{}", part_2_total);
}

fn rotator(table: Vec<Vec<&str>>) -> Vec<Vec<&str>> {
    let mut columns = Vec::<Vec<&str>>::new();
    for col in 0..table[0].len() {
        let mut the_column = Vec::<&str>::new();
        for row in 0..table.len() {
            the_column.push(table[row][col]);
        }
        columns.push(the_column);
    }
    columns
}

fn sum_row(row: &Vec<&str>) -> i64 {
    let last_idx = row.len() - 1;
    let operand = row[last_idx];
    let init: i64 = match operand {
        "+" => 0,
        "*" => 1,
        _ => { panic!("Unexpected operand") }
    };
    row[..last_idx].iter().fold(init as i64, |total, item| {
        let val = item.parse::<i64>().unwrap();
        match operand {
            "+" => total + val,
            "*" => total * val,
            _ => panic!("Unexpected operand"),
        }
    })
}

fn fields_in_line(line: &str) -> Vec<&str> {
    Vec::from_iter(line.split_whitespace())
}

fn do_puzzle_2(grid: &Vec<Vec<char>>) -> i64 {
    let starting_cols = get_start_locs(&grid[grid.len() - 1]);
    calculate_sums(grid, &starting_cols)
}

fn calculate_sums(grid: &Vec<Vec<char>>, starting_cols: &Vec<usize>) -> i64 {
    let mut total: i64 = 0;
    for i in 0..(starting_cols.len() - 1) {
        total += calculate_sum(grid, starting_cols, i);
    }
    total
}

fn calculate_sum(grid: &Vec<Vec<char>>, starting_cols: &Vec<usize>, starting_col_num: usize) -> i64 {
    let operand = grid[grid.len() - 1][starting_cols[starting_col_num]];
    let mut total: i64;
    match operand {
        '+' => total = 0,
        '*' => total = 1,
        _ => { panic!("Unexpected operand")}
    }
    for col in starting_cols[starting_col_num]..(starting_cols[starting_col_num + 1] - 1) {

        let item = parse_col(grid, col);
        match operand {
            '+' => total += item,
            '*' => total *= item,
            _ => { panic!("Unexpected operand") },     
        }
    }
    total
}

fn parse_col(grid: &Vec<Vec<char>>, col: usize) -> i64 {
    let mut val: i64 = 0;
    for i in 0..(grid.len() - 1) {
        if grid[i][col].is_numeric() {
            val *= 10;
            val += grid[i][col].to_digit(10).unwrap() as i64;
        }
    }
    val
}

fn get_start_locs(last_row: &Vec<char>) -> Vec<usize> {
    let mut sum_start_locs = Vec::<usize>::new();
    for i in 0..last_row.len() {
        if !last_row[i].is_whitespace() {
            sum_start_locs.push(i);
        }
    }
    sum_start_locs.push(last_row.len() + 1);
    sum_start_locs
}