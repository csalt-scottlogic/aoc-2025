fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let lines = Vec::from_iter(content.lines());
    let grid_master = parse_lines(&lines);
    let mut grid = grid_master.to_vec();
    let count = process_grid(&mut grid);
    println!("{count}");
    count_first_row(&mut grid[0]);
    count_splitters_in_grid(&mut grid);
    let part2_count = sum_row(&grid[grid.len() - 1]);
    println!("{part2_count}");
}

#[derive(Debug)]
#[derive(Clone)]
enum GridSpace {
    Splitter,
    CountedSplitter(i64),
    Beam,
    CountedBeam(i64),
    Free
}

fn process_grid(grid: &mut Vec<Vec<GridSpace>>) -> i64 {
    let mut count = 0;
    for i in 1..grid.len() {
        let (before, after) = grid.split_at_mut(i);
        count += process_row(&mut after[0], &mut before[before.len() - 1]);
    }
    count
}

fn count_first_row(row: &mut Vec<GridSpace>) {
    for i in 0..row.len() {
        if let GridSpace::Beam = row[i] {
            row[i] = GridSpace::CountedBeam(1);
        }
    }
}

fn count_splitters_in_grid(grid: &mut Vec<Vec<GridSpace>>) {
    for i in 1..grid.len() {
        let (before, after) = grid.split_at_mut(i);
        count_splitters(&mut after[0], &mut before[before.len() - 1]);
    }
}

fn count_splitters(row: &mut Vec<GridSpace>, prev: &mut Vec<GridSpace>) {
    println!("{prev:?}");
    println!("{row:?}");
    for i in 0..row.len() {
        match row[i] {
            GridSpace::Splitter => {
                if let GridSpace::CountedBeam(incoming) = prev[i] {
                    match row[i - 1] {
                        GridSpace::Beam => {
                            row[i - 1] = GridSpace::CountedBeam(incoming);
                        }
                        GridSpace::CountedBeam(already) => {
                            row[i - 1] = GridSpace::CountedBeam(incoming + already);
                        },
                        _ => ()
                    }
                    if let GridSpace::Beam = row[i + 1] {
                        match prev[i + 1] {
                            GridSpace::CountedBeam(already) => {
                                row[i + 1] = GridSpace::CountedBeam(incoming + already);
                            }
                            _ => {
                                row[i + 1] = GridSpace::CountedBeam(incoming);
                            }
                        }
                    }
                    row[i] = GridSpace::CountedSplitter(incoming)
                }
            },
            GridSpace::Beam => {
                match prev[i] {
                    GridSpace::CountedBeam(incoming) => {
                        row[i] = GridSpace::CountedBeam(incoming);
                    },
                    GridSpace::Free => (),
                    _ => {
                        panic!("Hey!");
                    }
                }
            },
            _ => (),
        }
    }
    println!("{row:?}");
}

fn sum_row(row: &Vec<GridSpace>) -> i64 {
    let mut count = 0;
    for i in 0..row.len() {
        count += match row[i] {
            GridSpace::CountedBeam(x) => x,
            GridSpace::CountedSplitter(x) => x,
            _ => 0,
        };
    }
    count
}

fn process_row(row: &mut Vec<GridSpace>, prev: &mut Vec<GridSpace>) -> i64 {
    let mut splits = 0;
    for i in 0..row.len() {
        match row[i] {
            GridSpace::Free => {
                if let GridSpace::Beam = prev[i] {
                    row[i] = GridSpace::Beam;
                }
            },
            GridSpace::Splitter => {
                if let GridSpace::Beam = prev[i] {
                    if let GridSpace::Free = row[i - 1] {
                        row[i - 1] = GridSpace::Beam;
                    }
                    if let GridSpace::Free = row[i + 1] {
                        row[i + 1] = GridSpace::Beam;
                    }
                    splits += 1;
                }
            },
            GridSpace::Beam => (),
            _ => (),
        }
    }
    splits
}

fn parse_row(line: &str) -> Vec<GridSpace> {
    let mut output = Vec::<GridSpace>::new();
    for c in line.chars() {
        let parsed_val = match c {
            '.' => Some(GridSpace::Free),
            'S' => Some(GridSpace::Beam),
            '^' => Some(GridSpace::Splitter),
            _ => None
        };
        if let Some(cell) = parsed_val {
            output.push(cell);
        };
    }
    output
}

fn parse_lines(lines: &[&str]) -> Vec<Vec<GridSpace>> {
    let mut output = Vec::<Vec<GridSpace>>::new();
    for line in lines {
        output.push(parse_row(line));
    }
    output
}
