fn main() {
    let lines = std::fs::read_to_string("input.txt").unwrap();
    let mut total = 0;
    for line in lines.split('\n') {
        if line.len() > 0 {
            total += highest_digits(line, 12);
        }
    }
    println!("{}", total);
}

fn highest_digits(data: &str, digit_count: usize) -> i64 {
    let mut sum: i64 = 0;
    let mut next_index = 0;
    for pos in (0..digit_count).rev() {
        sum *= 10;
        let next_highest = highest_digit(&data[next_index..(data.len() - pos)]);
        sum += next_highest.value;
        next_index += next_highest.index + 1;
    }
    sum
}

fn highest_digit(data: &str) -> HighestIndex {
    let mut highest = HighestIndex { value: 0, index: 0 };
    for i in 0..data.len() {
        let val = data[i..(i + 1)].parse::<i64>().unwrap();
        if val == 9 {
            return HighestIndex { value: 9, index: i };
        }
        if val > highest.value {
            highest.value = val;
            highest.index = i;
        }
    }
    highest
}

struct HighestIndex {
    value: i64,
    index: usize,
}
