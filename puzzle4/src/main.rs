fn main() {
    let file_contents = std::fs::read_to_string("input.txt").unwrap();
    let mut data = Vec::from_iter(file_contents.lines().map(|x| x.to_string()));
    let mut removed: usize = 0;
    loop {
        let count_results = count_over_grid(&data, 4);
        if count_results.removable.len() == 0 {
            break;
        }
        removed += count_results.removable.len();
        remove_these(&mut data, &count_results.removable);
    }
    println!("{removed}");
}

struct Location {
    x: usize,
    y: usize,
}

struct CountResult {
    removable: Vec<Location>,
}

fn count_over_grid(data: &Vec<String>, lim: i32) -> CountResult {
    let mut results: Vec<Location> = Vec::new();
    for y in 0..data.len() {
        for x in 0..(data[y].len()) {
            if data[y].chars().nth(x).unwrap() == '@' {
                if count_in_grid(data, i16::try_from(x).unwrap(), i16::try_from(y).unwrap(), usize::try_from(lim).unwrap()) {
                    results.push(Location { x, y })
                }
            }
        }
    }
    CountResult { removable: results }
}

fn remove_these(data: &mut Vec<String>, removables: &Vec<Location>) {
    for loc in removables {
        remove_this(data, loc.x, loc.y);
    }
}

fn remove_this(data: &mut Vec<String>, x: usize, y: usize) {
    let mut chars = Vec::from_iter(data[y].chars());
    chars[x] = '.';
    data[y] = chars.iter().collect();
}

fn count_in_grid(data: &Vec<String>, x: i16, y: i16, lim: usize) -> bool {
    let chars = chars_collect(data, x, y);
    let count = count_chars(&chars, '@');
    count < lim
}

fn chars_collect(data: &Vec<String>, x: i16, y: i16) -> Vec<char> {
    let mut collected: Vec<char> = Vec::new();
    for i in (x - 1)..(x + 2) {
        if let Some(c) = char_at(data, i, y - 1) {
            collected.push(c);
        }
        if let Some(c) = char_at(data, i, y + 1) {
            collected.push(c);
        }
    }
    if let Some(c) = char_at(data, x - 1, y) {
        collected.push(c);
    }
    if let Some(c) = char_at(data, x + 1, y) {
        collected.push(c);
    }
    collected
}

fn count_chars(data: &Vec<char>, c: char) -> usize {
    data.iter().filter(|x| **x == c).count()
}

fn char_at(data: &Vec<String>, x: i16, y: i16) -> Option<char> {
    if x < 0 || y < 0 {
        return None;
    }
    let x_size = usize::from(x as u16);
    let y_size = usize::from(y as u16);
    if y_size < data.len() && x_size < data[y_size].len() {
        Some(data[y_size].chars().nth(x_size).unwrap())
    }
    else {
        None
    }
}
